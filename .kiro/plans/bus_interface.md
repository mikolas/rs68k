# Bus Interface Specification (Final)

## Design

We provide a complete 68000 with all pins modeled. One method advances the CPU by one
CLK period. The emulator wires it up however it wants.

## Interface

```rust
/// CPU output pins — updated after each clock()
pub struct Pins68k {
    // Address/data bus
    pub addr: u32,      // A1-A23 (active when as_n=false)
    pub data_out: u16,  // D0-D15 output (active during write, when as_n=false and rw=false)
    pub fc: u8,         // FC0-FC2 (function code)

    // Bus control (active low)
    pub as_n: bool,     // Address strobe
    pub uds_n: bool,    // Upper data strobe (even byte, active low)
    pub lds_n: bool,    // Lower data strobe (odd byte, active low)
    pub rw: bool,       // Read=true, Write=false

    // Bus arbitration
    pub bg_n: bool,     // Bus grant (active low)

    // Peripheral control
    pub e: bool,        // E clock output (CLK÷10, 6-high/4-low)
    pub vma_n: bool,    // Valid memory address (response to VPA, active low)
}

/// CPU input pins — provided by the system each clock()
pub struct Inputs68k {
    // Data bus
    pub data_in: u16,   // D0-D15 input (active during read)

    // Bus control (active low)
    pub dtack_n: bool,  // Data transfer acknowledge
    pub berr_n: bool,   // Bus error
    pub vpa_n: bool,    // Valid peripheral address (triggers E-clock sync)

    // Interrupts (active low, accent: active low encoding means 7=no irq, 1=highest)
    pub ipl0_n: bool,   // Interrupt priority level bit 0
    pub ipl1_n: bool,   // Interrupt priority level bit 1
    pub ipl2_n: bool,   // Interrupt priority level bit 2

    // Bus arbitration (active low)
    pub br_n: bool,     // Bus request
    pub bgack_n: bool,  // Bus grant acknowledge

    // System control (active low)
    pub halt_n: bool,   // Halt
    pub reset_n: bool,  // Reset
}

impl M68000 {
    /// Advance CPU by one CLK period. Call at 7.09 MHz (or system clock rate).
    /// Reads inputs, updates internal state, updates output pins.
    pub fn clock(&mut self, inputs: &Inputs68k);

    /// Read current output pin state (no state change).
    pub fn pins(&self) -> &Pins68k;

    /// Convenience: run N clocks. For batch execution without per-clock interaction.
    /// Calls clock() internally N times with the provided inputs.
    pub fn run_clocks(&mut self, inputs: &Inputs68k, n: u32);

    /// High-level: execute one instruction (existing API, for testing/fast path).
    /// Uses the simple Bus trait internally.
    pub fn execute_one(&mut self, bus: &mut impl Bus) -> u32;
}
```

## Clock Timing

Single CLK input. One `clock()` call = one CLK period = ~141 ns at 7.09 MHz.

Internally the CPU tracks bus cycle states (S0-S7, two per CLK period) but this is
not exposed. External behavior is fully defined at CLK boundaries.

### Bus Cycle (minimum 4 CLK)

```
CLK:    ‾\_/‾\_/‾\_/‾\_/
State:  S0 S1 S2 S3 S4 S5 S6 S7
        |     |     |     |     |
        addr  AS    data  DTACK latch
        out   assert setup sample result
```

- S0-S1: Address and FC driven
- S2-S3: AS, UDS/LDS asserted; R/W valid
- S4-S5: Data driven (write) or sampled (read); wait for DTACK
- S6-S7: Data latched; AS/strobes negated

If DTACK not asserted by S4: wait states inserted (S4-S5 repeated).

### E Clock (generated internally)

```
CLK count:  0  1  2  3  4  5  6  7  8  9  0  1  2 ...
E output:   H  H  H  H  H  H  L  L  L  L  H  H  H ...
```

Free-running, ÷10 from CLK. 6 clocks high, 4 clocks low.
The CPU tracks this with an internal counter (0-9).

When VPA is asserted during a bus cycle, the CPU synchronizes to E:
- Waits until E falls (end of high phase)
- Asserts VMA
- Completes cycle when E rises again
- Total: 4 + 0..9 extra clocks depending on E phase at VPA assertion

### Internal Cycles (2 CLK)

ALU operations, register transfers, and other internal processing take 2 CLK
with no bus activity (AS stays negated).

## What We Generate

| Output | Description |
|--------|-------------|
| addr | 24-bit address (active during bus cycle) |
| data_out | 16-bit write data |
| fc | Function code: 1=user data, 2=user program, 5=super data, 6=super program, 7=interrupt ack |
| as_n | Bus cycle active |
| uds_n/lds_n | Byte lane selection |
| rw | Direction |
| bg_n | Bus granted to DMA |
| e | E clock (free-running) |
| vma_n | Peripheral cycle acknowledged |

## What We Consume

| Input | Description |
|-------|-------------|
| data_in | 16-bit read data (active when as_n=false and rw=true) |
| dtack_n | Cycle complete (false=acknowledged) |
| berr_n | Bus error (triggers exception) |
| vpa_n | Peripheral address (triggers E-sync) |
| ipl0-2_n | Interrupt level (active low, sampled at instruction boundaries) |
| br_n | DMA requests bus |
| bgack_n | DMA has taken bus |
| halt_n | Freeze CPU |
| reset_n | Reset CPU |

## Implementation Layers

```
┌─────────────────────────────────────────────┐
│  Emulator (Amiga, etc.)                     │
│  - Calls clock() at 7.09 MHz               │
│  - Provides inputs based on address decode  │
│  - Reads outputs for bus monitoring         │
└──────────────────┬──────────────────────────┘
                   │ Pins68k / Inputs68k
┌──────────────────▼──────────────────────────┐
│  Pin-level interface (clock method)          │
│  - Tracks S0-S7 bus states                  │
│  - Generates E clock                        │
│  - Handles VPA synchronization              │
│  - Handles BR/BG/BGACK arbitration          │
│  - Handles HALT/RESET                       │
└──────────────────┬──────────────────────────┘
                   │ Internal Bus trait
┌──────────────────▼──────────────────────────┐
│  Execution core (existing, unchanged)        │
│  - dispatch() / handlers                    │
│  - ALU, decode table                        │
│  - icount-based cycle tracking              │
└─────────────────────────────────────────────┘
```

The pin-level interface is a wrapper around the execution core. It translates between
the clock-by-clock pin interface and the instruction-at-a-time execution model.
The execution core is never modified.
