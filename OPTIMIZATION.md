# Partial Handler Optimization Plan

## Current Performance

- Full handlers: 110.3 MIPS (985 MHz effective)
- Pin-level (clock()): 74.4 MIPS (670 MHz effective)
- Overhead: 32.5% slower than full handlers
- Dispatch overhead: ~20–30% of partial handler runtime
- Per-clock overhead (E clock, IPL, arbitration, phase match): ~40–50 instructions before useful work

---

## Tier 1: Low-Effort, High-Impact (pins.rs hot path)

### 1. E clock: replace `% 10` with comparison

The `% 10` compiles to a multiply-shift-subtract sequence (7 instructions) every clock() call.

```rust
// Before:
self.pin_state.e_clock = (self.pin_state.e_clock + 1) % 10;

// After:
self.pin_state.e_clock += 1;
if self.pin_state.e_clock >= 10 { self.pin_state.e_clock = 0; }
```

Saves ~5 instructions per clock.

### 2. Hoist BusPhase::Idle as direct branch

The 5-arm match on bus_phase generates an indirect jump (jump table) every clock.
The most common path is Idle. Hoist it before the match:

```rust
if self.pin_state.bus_phase == BusPhase::Idle {
    // fast path (direct branch, predictable)
} else {
    self.handle_bus_phase(inputs); // cold path with match
}
```

Eliminates indirect jump on the most-common state.

### 3. Defer IPL sampling to S4 or instruction boundaries

Currently computes IPL from 3 bools and calls set_irq() every clock (~10 instructions).
Real 68000 samples IPL at instruction boundaries. Move to `inst_substate == 0` check
or `bus_phase == S4` only. Eliminates work on ~75% of clocks.

### 4. Reorder arbitration: check bus_granted first

```rust
// Before: 4 conditionals every clock
// After:
if unlikely(self.pin_state.bus_granted) { return; }
if unlikely(!inputs.br_n || !inputs.bgack_n) { self.handle_arbitration(inputs); }
```

Better branch prediction on the common path (no arbitration active).

**Combined Tier 1 estimate: ~20–30 fewer instructions per clock() call.**

---

## Tier 2: Medium-Effort, Medium-Impact (generated code + cpu.rs)

### 5. Eliminate dead `alu_and(x, 0xffff)` calls

1,965 occurrences of `alu_and(value, 0xffff)` — a no-op AND that only sets isr/aluo
as side effects. In many cases these are overwritten before being read. The generator
can track liveness and skip dead ALU ops.

Requires: gen_rust.py change + regeneration.

### 6. `set_ftu_const()` → 16-entry lookup table

Called in every instruction epilogue. The 5-arm match on `ird >> 12` is unpredictable.
Replace with a 16-entry precomputed table for the common cases:

```rust
static FTU_TABLE: [u16; 16] = [0, 0, 0, 0, 0x80, 0, 0, 0, 0x0f, 0, 0, 0, 0x0f, 0, 0, 0];
// Special handling for 0x5/0x6/0x7/0xe (shift count, displacement)
```

Requires: cpu.rs change only.

### 7. Struct field reorder: hot fields in first cache line

Current layout spans 3-4 cache lines for hot fields. Reorder M68000 struct:

First 64 bytes (cache line 1): aob, au, pc, sr, isr, aluo, dbin, edb, irc,
inst_substate, icount, inst_state, base_ssw, pin_state.bus_phase

Requires: cpu.rs reorder + verify no layout-dependent code.

### 8. Extract instruction epilogue to `#[inline(always)]` helper

The 7-line epilogue pattern is duplicated 1,779 times across handlers:

```rust
self.ird = self.ir;
if self.next_state != S_TRACE as u32 { self.next_state = self.int_next_state; }
self.au = self.pc.wrapping_add(2);
self.set_ftu_const();
self.icount -= 2;
self.inst_state = if self.next_state != 0 { ... } else { self.decode_table[...] };
if self.sr & SR_T != 0 { self.next_state = S_TRACE as u32; }
self.inst_substate = 0;
```

Extract to helper → reduces code size by ~12K lines → better I-cache utilization.

Requires: gen_rust.py change + regeneration.

---

## Tier 3: High-Effort, Structural

### 9. Profile-Guided Optimization (PGO)

Build with instrumentation, run benchmark, rebuild with profile data.
Expect 10–15% improvement from better branch prediction and code layout.

```sh
RUSTFLAGS="-Cprofile-generate=/tmp/pgo" cargo build --release
cargo run --release --example bench_partial
rustup run nightly cargo pgo optimize
```

### 10. Reduce dispatch frequency

Currently dispatch_partial is called 2–5× per instruction (once per bus cycle).
For substates that don't need a bus cycle (pure internal work), the handler could
continue executing the next substate in the same call instead of returning.

The generator already does this for some cases (inner `loop { match __state }`)
but could be more aggressive — batch all internal-only substates into a single
dispatch call, only returning when a bus_end is reached.

Requires: gen_rust.py architectural change.

### 11. Dead store elimination for `self.at`

451 occurrences of `self.at = M68000::ext32(...)` where `at` is never read before
being overwritten. Making `at` non-pub or using a local would let LLVM eliminate
these. Alternatively, the generator can track `at` liveness.

Requires: gen_rust.py analysis + cpu.rs visibility change.

### 12. Hot handler inlining

Top-20 handlers (MOVE variants, ADD, SUB, BRA, NOP, CMP) could be inlined directly
into dispatch_partial to avoid function call overhead. These dominate real workloads.

Requires: gen_rust.py change to emit inline code for hot handlers.

---

## Execution Order

1. Tier 1 (items 1–4): pure pins.rs, no regeneration, immediate gain
2. Item 6 (set_ftu_const): trivial cpu.rs change
3. Item 7 (struct reorder): cpu.rs field reorder + test verification
4. Item 9 (PGO): build system only
5. Items 5, 8, 10–12: gen_rust.py changes + full regeneration + test suite

---

## Verification

After each change, run:
```sh
cargo test --release --test sst           # 127 instructions, 317,500 vectors
cargo test --release --test partial       # partial vs full equivalence
cargo run --release --example bench_partial  # performance measurement
```
