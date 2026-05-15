# Porting Journal — MAME m68000 → Rust

## 2026-05-15: Phase 1 — Infrastructure

### Completed
- **Cargo.toml**: crate skeleton with dev-deps for test harness (serde, flate2)
- **types.rs**: All constants (S_RESET..S_LINEF, SR flags, SSW bits), Bus trait
- **cpu.rs**: M68000 struct with all fields matching C++ `m68000_device`:
  - `da[17]`, `sp`, `pc`, `ipc`, `au`, `at`, `aob`, `dt`, `int_vector`
  - `isr`, `sr`, `dbin`, `dbout`, `edb`, `irc`, `ir`, `ird`, `ftu`
  - `aluo`, `alue`, `alub`, `movemr`, `irdi`, `base_ssw`, `ssw`, `dcr`
  - `inst_state`, `next_state`, interrupt state
  - Helper methods: `set_16h`, `set_16l`, `ext32`, `high16`, `merge_16_32`,
    `set_ftu_const`, `step_movem`, `step_movem_predec`, `update_user_super`,
    `update_interrupt`, `set_irq`, `fc()`
- **alu.rs**: Core ALU ops — add, add8, addc, addx, addx8, sub, sub8, subc, subx, subx8,
  and, andx, and8, and8x, or, or8, eor, eor8
- **alu_ext.rs**: ext, not, not8, abcd8, sbcd8, sla0, sla1, over, asl, asl8, asl32,
  asr, asr8, asr32, lsl, lsl8, lsl32, lsr, lsr8, lsr32
- **alu_rot.rs**: rol, rol8, rol32, ror, ror8, ror32, roxl, roxl8, roxl32, roxr, roxr8,
  roxr32, roxr32ms, roxr32mu, sr_z, sr_nz_u, sr_nzvc, sr_nzvc_u, sr_xnzvc, sr_xnzvc_u
- **decode.rs**: 1525-entry packed decode table, `init_decode_table()` expansion to 64K

### Verification
- 9 unit tests passing: ALU add/sub/and/asr/lsl, decode table validity,
  set_ftu_const, update_interrupt
- Decode table produces 45815 valid entries (matches C++ expansion)
- Max state number: 1534 (matches handler count in m68000-head.h)

### Notes
- Bus trait passes no function code — the CPU computes FC internally from `base_ssw` + SR.
  The Bus impl can call `cpu.fc()` if needed, but for simplicity the trait methods don't
  take FC. **Revisit if Amiga integration needs FC per-access.**
- `icount` field is `i32` (matching C++ `int m_icount`) but we use it as a cycle accumulator
  that gets decremented per the C++ pattern. The execute loop will set it to a budget and
  run until ≤ 0.
- Stripped: `m_inst_substate`, `m_post_run`, `m_post_run_cycles`, `m_bcount`,
  `m_count_before_instruction_step` — all MAME scheduler artifacts.

## 2026-05-15: Phase 2 — Code Generator (Revised Approach)

### Failed Approach: Parsing Generated C++
Attempted to parse `m68000-sdf.cpp` (the generated C++ output) with a Python translator.
Got down to ~610 errors from 3678 but hit fundamental issues:
- Type system: C++ implicit u32→u16 truncation requires per-expression cast analysis
- Brace matching: nested if/else/goto patterns require full AST understanding
- Ternary operators: many special cases with nested ternaries
- goto/label: 526 handlers use gotos, 148 with backward jumps (MUL/DIV loops)

**Conclusion:** Parsing generated C++ is the wrong approach.

### Correct Approach: Fork the Generator

The `m68000gen.py` generator has a clean intermediate representation:
- `["=", reg, expr...]` — assignment
- `["alu", op, mask, info, args...]` — ALU operation
- `["bus_end", id, space, is_byte, is_write, ...]` — bus access
- `["label", name]` / `["goto", name]` / `["cgoto", [targets]]` — control flow
- `["istep"]` — internal 2-cycle step
- `["next_trace"]` / `["next_stop"]` — instruction end
- `["=t", expr]` — set branch condition
- `["priv"]` — privilege check
- etc.

The generator already knows:
- Which registers are u16 vs u32 (via the R enum and regname table)
- Which ALU ops are 8/16/32-bit (via ALUInfo flags)
- Where bus accesses happen and their type (program/data/cpu, read/write, byte/word)
- Control flow structure (labels, gotos, conditional gotos)

**Plan:**
1. Copy `m68000gen.py` → `gen_rust.py`
2. Replace `generate_source_from_code()` with Rust-emitting version
3. Replace `generate_source_file()` with Rust file output (split into chunks)
4. Add a `generate_dispatch()` function for the match statement
5. Strip: icount blocks, substate, access_to_be_redone, debugger hooks
6. Keep: ALL microcode logic, cycle counting, bus access ordering, control flow

### Key Differences in Rust Output

| Intermediate op | C++ output | Rust output |
|----------------|-----------|-------------|
| `["=", R.au, ...]` | `m_au = expr;` | `self.au = expr;` |
| `["=", R.sr, ...]` | `m_sr = expr;` | `self.sr = expr;` (with u16 cast if needed) |
| `["=16h", R.atl, ...]` | `set_16h(m_at, expr);` | `M68000::set_16h(&mut self.at, expr);` |
| `["alu", ...]` | `alu_xxx(a, b);` | `self.alu_xxx(a as u16, b as u16);` |
| `["bus_end", ...]` (read) | `m_edb = m_program.read_interruptible(...)` | `self.edb = bus.read_data(self.aob & !1);` |
| `["bus_end", ...]` (write) | `m_program.write_interruptible(...)` | `bus.write_data(self.aob & !1, self.dbout);` |
| `["istep"]` | `m_icount -= 2;` | `self.icount -= 2;` |
| `["label", x]` | `x:` | (handled by loop/match structure) |
| `["goto", x]` | `goto x;` | `__state = N; continue;` |
| `["cgoto", [...]]` | `if(m_t) goto x; else goto y;` | `match self.t { ... }` |
| icount check block | 8 lines of scheduler code | **omitted entirely** |
| `["trap", ...]` | `debugger_exception_hook(...)` | **omitted** |

### Goto Handling
The generator knows exactly which handlers have labels/gotos (from the intermediate code).
For those handlers, emit:
```rust
let mut __state = 0u32;
loop { match __state {
    0 => { ... __state = N; continue; }
    N => { ... }
    _ => return,
}}
```

### Type Safety
The generator knows register widths from the R enum:
- `R.axl`, `R.ayl`, `R.aul`, `R.atl`, `R.pcl`, `R.aobl` → low 16 bits of u32 regs
- `R.axh`, `R.ayh`, `R.auh`, `R.ath`, `R.pch`, `R.aobh` → high 16 bits of u32 regs
- `R.alue`, `R.alub`, `R.dbin`, `R.aluo`, `R.ftu`, `R.sr` → u16 regs
- `R.dcr` → u8

The `make_expression()` function can emit proper casts because it knows the source type.

### Next Steps
1. Fork m68000gen.py → gen_rust.py
2. Implement `generate_rust_source_from_code()` 
3. Implement `generate_rust_file()` with file splitting
4. Run, compile, iterate on errors
5. Verify with ProcessorTests

