# gen_rust.py Implementation Plan

## Overview

Fork `m68000/m68000gen.py` → `gen_rust.py`. Keep all microcode/nanocode decoding intact.
Replace only the output layer to emit Rust instead of C++.

## What stays unchanged (copy verbatim)

- Lines 1–804: microcode_base_table, nanocode_base_table, expansion, a1_pal, a23_pal,
  priv_test, R enum, regname, regdep, ALU enum, ALUInfo, GEN enum, alu_ops, alu_masks,
  alu_opnames, alu_single_param
- Lines 804–812: instruction list loading from m68000.lst
- Lines 813–824: nanox() helper
- Lines 825–932: generate_forward_graph()
- Lines 885–932: code_find_deps()
- Lines 934–1810: generate_base_code_for_microcode() (the big one — microcode→IR)
- Lines 1811–1870: generate_blocks()
- Lines 1879–1912: generate_code_from_blocks()
- Lines 1913–1958: propagate_bus_access(), detect_reset_toggling()
- Lines 1959–2001: generate_intermediate_code(), generate_intermediate_code_from_instruction(),
  generate_intermediate_code_from_state(), handler_name_for_state(), handler_name_for_instruction()
- Lines 2355–2370: states[] list

Total: ~2350 lines copied unchanged.

## What gets replaced (~400 lines new code)

### 1. `make_rust_expression(ci)` → returns `(str, ExprType)`

ExprType enum: U8, U16, U32.

Handles:
- None → ("0", U16)
- R.dcro → ("(1u16 << (self.dcr & 15))", U16)
- R.dcro8 → ("(1u16 << (self.dcr & 7))", U16)
- "ftu-i" → ("0xfff0 | ((self.next_state >> 23) as u16 & 0xe)", U16)
- "ftu-ssw" → ("(self.ftu & !0x1f) | self.ssw", U16)
- "trap-tvn" → ("0x80 | ((self.ird & 0xf) << 2)", U16)
- "int-tvn" → ("self.int_vector", U32)
- Plain register (not high16): (rust_regname[ci], reg_type[ci])
- Plain register (high16): ("M68000::high16({rust_regname[ci]})", U16)
- ["c", val] → ("0x{val:04x}u16", U16) or ("0x{val:04x}", U16)
- ["+c", expr, N] → wrapping_add/wrapping_sub with type propagation
- ["+r", e1, e2] → wrapping_add with type promotion (U16+U16=U16, any U32=U32)
- ["+1/2", expr, reg] → "{expr}.wrapping_add(if {reg} < 15 { 1 } else { 2 })"
- ["-1/2", expr, reg] → "{expr}.wrapping_sub(if {reg} < 15 { 1 } else { 2 })"
- ["merge", hi, lo] → ("M68000::merge_16_32({hi}, {lo})", U32)
- ["ext32", e] → ("M68000::ext32({e} as u16)", U32) if e is U32, else ("M68000::ext32({e})", U32)
- ["ext32h", e] → ("M68000::ext32h({e} as u16)", U16) if e is U32, else ("M68000::ext32h({e})", U16)
- ["q8"] → dead code, panic

### 2. `rust_regname[]` and `reg_type[]` tables

Parallel to `regname[]` but with Rust syntax:
- "self.da[rx]", "self.da[ry]", "self.da[self.map_sp((self.irc >> 12) as usize)]",
  "self.da[self.sp as usize]", "self.da[16]", "self.da[15]", "self.dt", "self.au",
  "self.at", "self.pc", "self.aob", "self.da[self.movems as usize]"
- Types: indices 1-14 → U32, indices 15-28 → U16 (high16 wrapped), indices 29+ → U16/U8

### 3. `cast_expr(code, from_type, to_type)` helper

- Same type → no-op
- U16 → U32 → "(({code}) as u32)"
- U32 → U16 → "(({code}) as u16)"
- U16 → U8 → "(({code}) as u8)"
- U32 → U8 → "(({code}) as u8)"

### 4. `generate_rust_source_from_code(code, gen_mode)` 

Walks the IR (same structure as original). For each CI opcode, emits Rust per the skill rules.

Key differences from C++ emitter:
- No icount <= 0 blocks (skip the entire scheduler section in bus_end)
- No inst_substate
- No [[fallthrough]] / case labels
- Wrapping arithmetic
- Type-aware casts
- =t lookup table (dict mapping C++ string → Rust string)
- "i" lookup table

Returns: list of Rust source lines (indented with spaces, not tabs)

### 5. `emit_handler(name, code, out_lines)`

Determines if handler has labels (scan for "label" in code).
- If no labels: emit plain function body
- If has labels: emit loop/match wrapper

### 6. `generate_rust_files(output_dir)`

Main entry point. Replaces `generate_source_file()`.

- Iterates states[] and instructions[]
- Generates IR for each (unchanged logic)
- Calls generate_rust_source_from_code()
- Splits handlers into files of ~150 each
- Emits dispatch.rs with match statement
- Emits mod.rs

### 7. Main block

```
Usage: python3 gen_rust.py m68000/m68000.lst src/handlers/
```

Replaces the multi-mode CLI of the original (we only need one mode: direct+full for m68000).

## File output structure

```
src/handlers/
├── mod.rs           — module declarations
├── dispatch.rs      — match self.inst_state { ... }
├── handlers_00.rs   — states 0-149
├── handlers_01.rs   — states 150-299
├── ...
└── handlers_10.rs   — states 1500-1534
```

Each handler file:
```rust
use crate::cpu::M68000;
use crate::types::*;

#[allow(non_snake_case, unused_variables, unused_mut, unreachable_code)]
impl M68000 {
    pub fn handler_state_reset(&mut self, bus: &mut impl crate::types::Bus) {
        ...
    }
    ...
}
```

## Verification steps

1. Run gen_rust.py → check it produces files without Python errors
2. `cargo build` → fix any compile errors (should be zero if skill rules followed)
3. Run NOP ProcessorTest → verify basic execution
4. Run full suite progressively per the skill's testing strategy

## Estimated size

- gen_rust.py: ~2750 lines (2350 unchanged + 400 new)
- Generated Rust: ~80,000 lines across 11 files (smaller than C++ due to no scheduler blocks)
