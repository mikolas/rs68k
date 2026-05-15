# Bus Interface Specification

## Design Principles

1. The execution core (`dispatch`, handlers, ALU) remains untouched
2. All bus interaction happens through a single trait
3. The trait models the real 68000 bus signals, not abstract memory operations
4. The caller (emulator) controls timing — the CPU never blocks

## Real 68000 Bus Signals

### Outputs (CPU → System)
| Signal | Width | Description |
|--------|-------|-------------|
| A1-A23 | 23 bits | Address bus (A0 encoded in UDS/LDS) |
| D0-D15 | 16 bits | Data bus (bidirectional) |
| FC0-FC2 | 3 bits | Function code (space + supervisor) |
| AS | 1 | Address strobe (bus cycle active) |
| UDS | 1 | Upper data strobe (even byte) |
| LDS | 1 | Lower data strobe (odd byte) |
| R/W | 1 | Read (1) / Write (0) |

### Inputs (System → CPU)
| Signal | Width | Description |
|--------|-------|-------------|
| D0-D15 | 16 bits | Data bus (read data) |
| DTACK | 1 | Data transfer acknowledge (cycle complete) |
| BERR | 1 | Bus error |
| IPL0-IPL2 | 3 bits | Interrupt priority level |
| BR | 1 | Bus request (DMA wants bus) |
| BGACK | 1 | Bus grant acknowledge (DMA has bus) |
| VPA | 1 | Valid peripheral address (autovector) |
| HALT | 1 | Halt |
| RESET | 1 | Reset input |

## Proposed Rust Interface

```rust
/// Bus cycle request from CPU
pub struct BusCycle {
    pub addr: u32,       // A1-A23 (bit 0 = 0 for word, encodes byte lane)
    pub fc: u8,          // FC0-FC2 (1=user data, 2=user program, 5=super data, 6=super program, 7=cpu/interrupt ack)
    pub rw: bool,        // true = read, false = write
    pub uds: bool,       // upper data strobe
    pub lds: bool,       // lower data strobe
    pub data: u16,       // write data (only valid when rw=false)
}

/// Bus cycle response from system
pub struct BusResponse {
    pub data: u16,       // read data (only valid when rw=true)
    pub dtack: bool,     // cycle acknowledged (false = insert wait state)
    pub berr: bool,      // bus error
}

/// System state inputs (active between bus cycles)
pub struct BusInputs {
    pub ipl: u8,         // interrupt priority level (0-7, active low inverted)
    pub br: bool,        // bus request
    pub halt: bool,      // halt
    pub reset: bool,     // reset
}

/// The bus trait — one method per bus cycle, one for between-cycle state
pub trait Bus68k {
    /// Execute a bus cycle. Called once per 4-clock bus cycle.
    /// Returns response. If dtack=false, CPU calls again next cycle (wait state).
    fn cycle(&mut self, cycle: &BusCycle) -> BusResponse;

    /// Query system inputs (called between bus cycles / during internal cycles).
    /// Used for interrupt level, bus request, halt.
    fn inputs(&self) -> BusInputs;
}
```

## Execution Model

```rust
impl M68000 {
    /// Run one bus cycle (4 clocks). Returns number of clocks consumed (always 4).
    /// The emulator calls this at the 68000 clock rate.
    /// Between calls, the emulator advances other hardware (DMA, video, etc).
    pub fn tick(&mut self, bus: &mut impl Bus68k) -> u32;

    /// Run until N clocks consumed. For batch execution when no events pending.
    pub fn run(&mut self, bus: &mut impl Bus68k, clocks: u32) -> u32;
}
```

## How It Maps to Current Code

The current `execute_one` does multiple bus accesses internally (each handler calls
`bus.read_program()` etc. multiple times). To support the cycle-level interface:

**Option A: Wrapper adapter** (non-intrusive)
```rust
/// Adapts Bus68k to the current Bus trait for the execution core
struct BusAdapter<'a, B: Bus68k> {
    bus: &'a mut B,
    fc: u8,
}

impl<B: Bus68k> Bus for BusAdapter<'_, B> {
    fn read_program(&mut self, addr: u32) -> u16 {
        let cycle = BusCycle { addr, fc: self.fc, rw: true, uds: true, lds: true, data: 0 };
        self.bus.cycle(&cycle).data
    }
    // ... etc
}
```

The CPU sets `self.fc` before each bus access (already does via `base_ssw`).
The adapter translates between the abstract Bus trait and the signal-level Bus68k.

**Option B: DMA integration**

For DMA cycle stealing, the emulator doesn't call `execute_one` during DMA slots.
The CPU is effectively frozen. This matches real hardware where BR/BG handshake
pauses the CPU between bus cycles.

For the Amiga specifically:
- Each 68000 clock = 1 CCK (color clock) / 2
- DMA slots are allocated per scan line
- CPU gets remaining slots
- The emulator schedules: `if cpu_slot { cpu.tick(bus); } else { dma_access(); }`

## Wait States

If `BusResponse.dtack = false`, the CPU inserts a wait state (2 extra clocks).
In the adapter, this means calling `bus.cycle()` again until dtack is asserted.
The Amiga has no wait states for normal memory, but CIA access uses VPA
(which adds ~6 cycles for synchronization).

## Interrupt Handling

The emulator updates `bus.inputs().ipl` whenever interrupt lines change.
The CPU checks IPL at instruction boundaries (already implemented via
`update_interrupt` / `int_next_state`). The adapter calls `bus.inputs()`
and feeds IPL to `cpu.set_irq()` at appropriate times.

## Summary

The current `Bus` trait stays as the internal interface for the execution core.
A new `Bus68k` trait provides the signal-level interface for emulator integration.
An adapter bridges them. The execution core is never modified.

```
Emulator ←→ Bus68k trait ←→ BusAdapter ←→ Bus trait ←→ M68000 execution core
```
