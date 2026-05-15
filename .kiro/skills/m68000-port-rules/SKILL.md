---
name: m68000-port-rules
description: Rules and constraints for porting MAME m68000gen.py to emit Rust. Use when writing or modifying gen_rust.py, when translating IR opcodes to Rust, or when debugging test failures.
---

# M68000 Generator Port — Rules & Constraints

## Architecture

The port works by forking `m68000/m68000gen.py` into `gen_rust.py`. We replace ONLY the output
functions (`generate_source_from_code` and `generate_source_file`). All microcode decoding,
intermediate representation generation, block graph construction, and dependency-sorted ordering
remain UNCHANGED from the original.

The intermediate representation (IR) is a list of code blocks. Each block is a list of operations.
Operations arrive PRE-SORTED by `sort_and_append` — their order is the correct execution order.

## Type System

### Register Types

| Category | Registers | Rust type | Notes |
|----------|-----------|-----------|-------|
| 32-bit address/data | da[17], au, at, pc, aob, dt, movems, int_vector, ipc, sp | u32 | Full 32-bit registers |
| 16-bit internal | alue, alub, dbin, dbout, aluo, ftu, ir, irc, ird, edb, sr, isr, movemr, base_ssw, ssw, new_sr, inst_state | u16 | Pipeline/ALU registers |
| 8-bit | dcr | u8 | Divide counter |
| Signed | t, icount | i32 | Branch condition, cycle counter |
| Packed | next_state, int_next_state | u32 | Low bits = state, high bits = interrupt level |

### Expression Types

The `make_expression` function in the original generator produces C++ text. Our `make_rust_expression`
must produce `(rust_code: String, type: ExprType)` where ExprType is U8, U16, or U32.

Rules for determining expression type:
- `R.axl` through `R.moveml` (indices 1-14): these map to u32 registers → ExprType::U32
- `R.axh` through `R.movemh` (indices 15-28): wrapped in `high16()` → ExprType::U16
- `R.alue` through `R.movemr` (indices 29-43): native u16 → ExprType::U16
- `R.dcr`: u8 → ExprType::U8
- `R.dcro` / `R.dcro8`: computed expressions → ExprType::U16
- `["c", val]`: constant → ExprType::U16 (always 16-bit in microcode context)
- `["+c", expr, N]`: same type as expr (add constant preserves type)
- `["+r", expr1, expr2]`: if either is U32, result is U32; else U16
- `["merge", hi, lo]`: always U32
- `["ext32", expr]`: always U32 (sign-extends u16 to u32)
- `["ext32h", expr]`: always U16 (produces 0x0000 or 0xFFFF)
- `["+1/2", expr, reg_name]`: same type as expr (U32)
- `["-1/2", expr, reg_name]`: same type as expr (U32)
- Special strings ("ftu-i", "ftu-ssw", "trap-tvn"): U16
- Special string "int-tvn": U32

### Cast Rules

When assigning expression to target:
- U32 expr → u16 target: emit `(expr) as u16`
- U16 expr → u32 target: emit `(expr) as u32`
- U32 expr → u32 target: no cast
- U16 expr → u16 target: no cast

When passing to ALU functions (all take u16 except alu_roxr32 which takes u32):
- U32 expr → u16 param: emit `(expr) as u16`
- U16 expr → u16 param: no cast

When passing to ext32/high16 (take u16):
- U32 expr → u16 param: emit `(expr) as u16`

## IR Opcode Translation Rules

### Assignments

| IR opcode | Rust output |
|-----------|-------------|
| `["=", reg, expr...]` | `{regname} = {expr};` (with cast if types differ) |
| `["=16h", reg, expr...]` | `M68000::set_16h(&mut {regname}, {expr});` |
| `["=16l", reg, expr...]` | `M68000::set_16l_32(&mut {regname}, {expr});` |
| `["=8", reg, expr...]` | `M68000::set_8_32(&mut {regname}, ({expr}) as u8);` |
| `["=8h", reg, expr...]` | `M68000::set_8h(&mut {regname}, ({expr}) as u8);` |
| `["=8xl", reg, expr...]` | `M68000::set_8xl(&mut {regname}, {expr});` |
| `["=8xh", reg, expr...]` | `M68000::set_8xh(&mut {regname}, {expr});` |
| `["=sr", reg, expr...]` | `self.isr = ({expr}) & (SR_CCR\|SR_SR); self.sr = self.isr; self.update_user_super(); self.update_interrupt();` |
| `["=srd", reg, expr...]` | `self.new_sr = ({expr}) & (SR_CCR\|SR_SR); self.isr = self.new_sr;` |
| `["=ccr", reg, expr...]` | `self.isr = ({expr}) & SR_CCR \| (self.sr & SR_SR); self.sr = self.isr;` |
| `["=sri7"]` | `self.sr \|= 0x0700; self.update_interrupt();` |
| `["=sri"]` | `self.sr = (self.sr & !SR_I) \| (((self.next_state >> 16) as u16) & SR_I); self.update_interrupt();` |

### ALU

| IR opcode | Rust output |
|-----------|-------------|
| `["alu", op, mask, info, arg1]` | `self.{alu_name}({arg1_cast});` + optional `self.sr_{flags}();` |
| `["alu", op, mask, info, arg1, arg2]` | `self.{alu_name}({arg1_cast}, {arg2_cast});` + optional `self.sr_{flags}();` |

ALU argument casting: always cast to the function's parameter type (u16 for most, u8 for *8 variants, u32 for alu_roxr32).

### Bus Access

| IR opcode | Rust output |
|-----------|-------------|
| `["bus_end", ...]` read, program space | `self.edb = bus.read_program(self.aob & !1u32);` |
| `["bus_end", ...]` read, data space | `self.edb = bus.read_data(self.aob & !1u32);` |
| `["bus_end", ...]` read, cpu space | `self.edb = bus.read_cpu(self.aob);` |
| `["bus_end", ...]` read, byte | Same as word read + `if self.aob & 1 == 0 { self.edb >>= 8; }` |
| `["bus_end", ...]` write, word | `bus.write_data(self.aob & !1u32, self.dbout);` |
| `["bus_end", ...]` write, byte | `bus.write_data(self.aob & !1u32, self.dbout);` |

After every bus access: `self.icount -= 4;`

For non-byte, non-cpu-space reads: add address error check:
```rust
if self.aob & 1 != 0 {
    self.icount -= 4;
    self.inst_state = S_ADDRESS_ERROR; // or S_DOUBLE_FAULT if ci[8] (TAS/critical)
    return;
}
```

If `ci[10]` (sr_update): emit `self.sr = self.new_sr; self.update_user_super(); self.update_interrupt();` after the bus access.

If interrupt vector lookup: emit `self.start_interrupt_vector_lookup();` before and `self.end_interrupt_vector_lookup();` after.

### Control Flow

| IR opcode | Rust output |
|-----------|-------------|
| `["istep"]` | `self.icount -= 2;` |
| `["halt"]` | `self.icount = 0; return;` |
| `["label", name]` | Start of new match arm in loop/match |
| `["goto", name]` | `__state = {N}; continue;` |
| `["cgoto", [t0, t1]]` | `if self.t != 0 { __state = {t1_id}; } else { __state = {t0_id}; } continue;` |
| `["cgoto", [t0, t1, t2]]` | `match self.t { 0 => __state = {t0}, 1 => __state = {t1}, _ => __state = {t2} }; continue;` |
| `["cgoto", [t0, t1, t2, t3]]` | `match self.t { 0 => __state = {t0}, 1 => __state = {t1}, 2 => __state = {t2}, _ => __state = {t3} }; continue;` |
| `["next_trace"]` | `self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] }; if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; } return;` |
| `["next_stop"]` | `self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] }; return;` |
| `["set_trace"]` | `if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }` |
| `["clear_trace"]` | `if self.next_state == S_TRACE as u32 { self.next_state = 0; }` |

### Special

| IR opcode | Rust output |
|-----------|-------------|
| `["priv"]` | `if self.sr & SR_S == 0 { self.inst_state = S_PRIVILEGE; return; }` |
| `["update_ssw"]` | `self.ssw = self.base_ssw \| (if self.sr & SR_S != 0 { SSW_S } else { 0 });` |
| `["trap", val]` | SKIP (debugger hook) |
| `["reset", 0/1]` | SKIP (reset line callback, not needed for emulation) |
| `["i", str]` | Translate via lookup table (see below) |

### Raw Inline Translations (`["i", str]`)

| C++ string | Rust output |
|------------|-------------|
| `"set_ftu_const();"` | `self.set_ftu_const();` |
| `"step_movem();"` | `self.step_movem();` |
| `"step_movem_predec();"` | `self.step_movem_predec();` |
| `"update_user_super();"` | `self.update_user_super();` |
| `"update_interrupt();"` | `self.update_interrupt();` |
| `"m_sr = (m_sr & ~SR_T) \| SR_S;"` | `self.sr = (self.sr & !SR_T) \| SR_S;` |
| `"if(m_next_state != S_TRACE) m_next_state = m_int_next_state;"` | `if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }` |
| `"if(!m_cmpild_instr_callback..."` | SKIP |
| `"if(!m_rte_instr_callback..."` | SKIP |
| Comments (`"// ..."`) | SKIP |
| `"FAIL"` | `panic!("FAIL: cross-dependency in microcode");` |
| `"logerror(...)"` | SKIP |

### Branch Condition Translations (`["=t", str]`)

| C++ expression | Rust expression (assigned to self.t: i32) |
|----------------|-------------------------------------------|
| `"0"` | `0` |
| `"1"` | `1` |
| `"m_irc & 0x0800"` | `(self.irc & 0x0800) as i32` |
| `"m_dcr & 0x10"` | `(self.dcr & 0x10) as i32` |
| `"m_isr & SR_C"` | `(self.isr & SR_C) as i32` |
| `"m_isr & SR_N"` | `(self.isr & SR_N) as i32` |
| `"m_isr & SR_V"` | `(self.isr & SR_V) as i32` |
| `"m_isr & SR_Z"` | `(self.isr & SR_Z) as i32` |
| `"!(m_isr & (SR_V\|SR_N))"` | `if self.isr & (SR_V\|SR_N) == 0 { 1 } else { 0 }` |
| `"!(m_au & 0x3f)"` | `if self.au & 0x3f == 0 { 1 } else { 0 }` |
| `"m_alue & 1"` | `(self.alue & 1) as i32` |
| `"!m_movemr"` | `if self.movemr == 0 { 1 } else { 0 }` |
| `"m_sr & SR_C"` | `(self.sr & SR_C) as i32` |
| `"!(m_sr & SR_C)"` | `if self.sr & SR_C == 0 { 1 } else { 0 }` |
| `"m_sr & SR_Z"` | `(self.sr & SR_Z) as i32` |
| `"!(m_sr & SR_Z)"` | `if self.sr & SR_Z == 0 { 1 } else { 0 }` |
| `"m_sr & SR_V"` | `(self.sr & SR_V) as i32` |
| `"!(m_sr & SR_V)"` | `if self.sr & SR_V == 0 { 1 } else { 0 }` |
| `"m_sr & SR_N"` | `(self.sr & SR_N) as i32` |
| `"!(m_sr & SR_N)"` | `if self.sr & SR_N == 0 { 1 } else { 0 }` |
| `"(m_sr & (SR_C\|SR_Z)) == 0"` | `if (self.sr & (SR_C\|SR_Z)) == 0 { 1 } else { 0 }` |
| `"(m_sr & (SR_C\|SR_Z)) != 0"` | `if (self.sr & (SR_C\|SR_Z)) != 0 { 1 } else { 0 }` |
| `"((m_sr & (SR_N\|SR_V)) == SR_N) \|\| ((m_sr & (SR_N\|SR_V)) == SR_V)"` | `if (self.sr & (SR_N\|SR_V)) == SR_N \|\| (self.sr & (SR_N\|SR_V)) == SR_V { 1 } else { 0 }` |
| `"((m_sr & (SR_N\|SR_V)) == (SR_N\|SR_V)) \|\| ((m_sr & (SR_N\|SR_V)) == 0)"` | `if (self.sr & (SR_N\|SR_V)) == (SR_N\|SR_V) \|\| (self.sr & (SR_N\|SR_V)) == 0 { 1 } else { 0 }` |
| `"((m_sr & (SR_N\|SR_V\|SR_Z)) == (SR_N\|SR_V)) \|\| ((m_sr & (SR_N\|SR_V\|SR_Z)) == 0)"` | `if (self.sr & (SR_N\|SR_V\|SR_Z)) == (SR_N\|SR_V) \|\| (self.sr & (SR_N\|SR_V\|SR_Z)) == 0 { 1 } else { 0 }` |
| `"(m_sr & SR_Z) \|\| ((m_sr & (SR_N\|SR_V)) == SR_N) \|\| ((m_sr & (SR_N\|SR_V)) == SR_V)"` | `if self.sr & SR_Z != 0 \|\| (self.sr & (SR_N\|SR_V)) == SR_N \|\| (self.sr & (SR_N\|SR_V)) == SR_V { 1 } else { 0 }` |
| `"(m_isr & SR_Z) ? 2 : (m_isr & SR_N) ? 1 : 0"` | `if self.isr & SR_Z != 0 { 2 } else if self.isr & SR_N != 0 { 1 } else { 0 }` |
| `"(m_isr & (SR_Z\|SR_N)) != 0"` | `if (self.isr & (SR_Z\|SR_N)) != 0 { 1 } else { 0 }` |
| `"!(m_au & 0x3f) ? 0 : m_alue & 2 ? 1 : 2"` | `if self.au & 0x3f == 0 { 0 } else if self.alue & 2 != 0 { 1 } else { 2 }` |
| `"!(m_au & 0x3f) ? 0 : (m_alue & 3) == 1 ? 1 : (m_alue & 3) == 2 ? 2 : 3"` | `if self.au & 0x3f == 0 { 0 } else if self.alue & 3 == 1 { 1 } else if self.alue & 3 == 2 { 2 } else { 3 }` |

Any `=t` string NOT in this table → emit `compile_error!("unknown =t: {string}")`.

## Arithmetic Rules

ALL arithmetic on u32 values MUST use wrapping operations:
- `a + b` → `a.wrapping_add(b)`
- `a - b` → `a.wrapping_sub(b)`

For u16 arithmetic in ALU functions, wrapping is handled internally (operations use u32 intermediates).

For the `["+c", expr, N]` expression:
- If expr type is U32: `{expr}.wrapping_add({N})` or `{expr}.wrapping_sub({-N})`
- If expr type is U16: `{expr}.wrapping_add({N})` or `{expr}.wrapping_sub({-N})`

For `["+r", expr1, expr2]`:
- Both U32: `{expr1}.wrapping_add({expr2})`
- Mixed: cast the U16 to U32 first, then wrapping_add

## Goto Handler Structure

Handlers with labels/gotos use this template:
```rust
pub fn handler_NAME(&mut self, bus: &mut impl Bus) {
    // rx/ry declarations if needed
    let mut __state: u32 = 0;
    loop { match __state {
    0 => {
        // block 0 code
        __state = N; continue;
    }
    N => {
        // block N code
    }
    _ => return,
    }}
}
```

Handlers WITHOUT labels/gotos are emitted as plain linear code (no loop/match).

## CRITICAL INVARIANTS

1. **NEVER reorder statements within a code block.** The dependency sort has already determined the correct order.
2. **NEVER omit the address error check** after a non-byte, non-cpu-space bus access. It's real CPU behavior, not MAME overhead.
3. **NEVER omit `self.icount -= 4`** after bus access or `self.icount -= 2` for istep. These are real cycle counts.
4. **ALWAYS omit** the icount <= 0 check blocks, access_to_be_redone, inst_substate, debugger hooks. These are MAME scheduler artifacts.
5. **The `next_state` field is packed**: `(interrupt_level << 24) | state_number`. When assigned to `inst_state` (u16), truncation extracts the state. When shifted right, it extracts the level.
6. **`m_t` uses C boolean semantics**: non-zero = true, zero = false. The `!` operator in C is logical NOT (returns 0 or 1), NOT bitwise NOT.

## Testing Strategy

1. Generate code, compile. Any unhandled IR opcode → `compile_error!`.
2. Run NOP test first (simplest instruction, no addressing modes).
3. Run MOVE.q (simple register operation).
4. Run ADD.b/w/l (ALU with flags).
5. Run Bcc (branch conditions — tests all `=t` translations).
6. Run LEA/JMP/JSR (addressing modes with gotos).
7. Run MOVEM (complex goto handler with loops).
8. Run MUL/DIV (backward gotos, multi-cycle loops).
9. Run full suite.

At each step: if tests fail, compare generated Rust output against C++ output for the same handler to find the divergence.
