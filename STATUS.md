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

### Next: Phase 2 — Code Generator
- Write Python script to parse m68000-sdf.cpp → Rust handler functions
- Target: mechanical translation preserving exact cycle counts and bus access order
