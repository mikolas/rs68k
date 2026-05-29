# Bug Report: Autovectored Interrupts Not Handled in Pin-Level Mode

## Summary

When VPA is asserted during an interrupt acknowledge (IACK) cycle, the CPU
should internally generate the autovector (vector = 24 + interrupt_level).
Instead, it reads the vector number from the data bus (`edb`), which contains
stale data, causing the CPU to jump to the wrong exception handler.

## Affected Interface

Pin-level mode only (`clock()` / partial handlers). The Bus trait interface
handles autovectors correctly via `read_cpu()`.

## Reproduction

1. Assert IPL pins to request level 3 interrupt
2. CPU initiates IACK cycle: fc=7, addr=$FFFFF6
3. Respond with VPA (vpa_n=false, dtack_n stays true)
4. CPU completes VPA/E-clock synchronization correctly
5. CPU reads vector from `edb` (data_in) instead of computing 24+3=27
6. Result: vectors to address (stale_byte & 0xFF) * 4 instead of $6C

## Root Cause

`end_interrupt_vector_lookup()` in `cpu.rs` line 235-237:

```rust
pub fn end_interrupt_vector_lookup(&mut self) {
    self.int_vector = ((self.edb & 0xff) as u32) << 2;
    self.int_next_state = 0;
}
```

This unconditionally uses `edb` (the data bus value latched in S6). For
autovectored interrupts (VPA response during IACK), the real 68000 ignores
the data bus and internally generates the vector:

    vector_address = (24 + interrupt_level) * 4

The pin-level state machine in `pins.rs` correctly handles VPA timing
(E-clock sync, VMA assertion) but does not signal back to the handler
that the cycle was completed via VPA rather than DTACK.

## 68000 Hardware Behavior

During an IACK cycle, three responses are possible:

| Response | Meaning | Vector Source |
|----------|---------|---------------|
| DTACK | Vectored interrupt | Data bus bits 0-7 (device provides vector number) |
| VPA | Autovectored interrupt | Internal: 24 + level (25-31) |
| BERR | Spurious interrupt | Internal: vector 24 |

The Amiga uses autovectored interrupts exclusively (VPA for all IACK cycles).

## Proposed Fix

Option A: Add a flag `pin_state.vpa_completed` set in S4 when VPA cycle
finishes, checked in `end_interrupt_vector_lookup()`:

```rust
pub fn end_interrupt_vector_lookup(&mut self) {
    if self.pin_state.vpa_completed {
        let level = (self.int_next_state >> 24) & 7;
        self.int_vector = (24 + level) * 4;
        self.pin_state.vpa_completed = false;
    } else {
        self.int_vector = ((self.edb & 0xff) as u32) << 2;
    }
    self.int_next_state = 0;
}
```

Option B: Check if the bus cycle that just completed used VPA by examining
whether `vma_n` was asserted (VMA is only driven during VPA cycles).

## Impact

Without this fix, no autovectored interrupt works in pin-level mode. This
breaks all Amiga emulation (which uses autovectors exclusively) and any
other system using autovectored interrupts with the pin-level interface.

## Files

- `src/cpu.rs:235` — `end_interrupt_vector_lookup()`
- `src/pins.rs:225-235` — VPA handling in BusPhase::S4
- `src/handlers_partial/handlers_00.rs:618` — call site (substate 4 of interrupt handler)

## Evidence

Trace from ramiga Amiga 500 emulator booting Kickstart 1.2:
- IACK cycle at record 15003423: fc=7, addr=$FFFFF6 (level 3)
- VPA asserted, E-clock sync completed correctly
- data_in=$2F02 (stale from previous bus cycle)
- CPU computes int_vector = ($02 & $FF) << 2 = $08 (bus error!)
- PC jumps to $000008 instead of $00006C (level 3 autovector)
