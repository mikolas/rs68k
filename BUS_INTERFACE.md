# Bus Interface Specification

## Two Interfaces, One Core

rs68k exposes two bus interfaces. Both execute the same verified microcode — they differ
only in how bus cycles are performed.

### Interface A: Simple Bus Trait (Full Handlers)

The CPU calls your code to perform bus accesses. One function call = one complete instruction.

```rust
cpu.execute_one(&mut my_bus) -> u32  // returns cycles consumed
```

**Use when:** you don't need per-cycle accuracy. The emulator advances other hardware
by the returned cycle count after each instruction.

### Interface B: Pin-Level (Partial Handlers)

Your code calls the CPU. One function call = one clock period. The CPU signals what it
needs on its output pins; you respond on the input pins.

```rust
cpu.clock(&inputs) // call at 7.09 MHz (or system clock)
```

**Use when:** you need true parallel hardware emulation — CPU, DMA controller, and
peripherals all advance together, one clock at a time.

---

## Interface A: Simple Bus Trait

### The Trait

```rust
pub trait Bus {
    fn read_program(&mut self, addr: u32) -> u16;      // instruction fetch
    fn read_data(&mut self, addr: u32) -> u16;         // data read
    fn write_data(&mut self, addr: u32, data: u16);    // word write
    fn write_data_byte(&mut self, addr: u32, data: u8); // byte write
    fn read_cpu(&mut self, addr: u32) -> u16;          // interrupt vector
}
```

### Semantics

- `addr` is always the physical address. For word accesses, bit 0 is always 0.
- `write_data_byte`: `addr` bit 0 selects the byte lane (0=high byte, 1=low byte).
- `read_cpu`: called during interrupt acknowledge. Address encodes the vector.
- All accesses are instantaneous from the CPU's perspective.
- The CPU handles cycle counting internally (`icount`).

### Usage

```rust
let cycles = cpu.execute_one(&mut bus);
// CPU has completed one instruction. `cycles` is the exact clock count.
// Advance your system by `cycles` clocks.
```

### Function Codes

The CPU sets `cpu.base_ssw` before each access. If you need the function code:
```rust
let fc = cpu.fc();  // returns u8: 1-7
// 1=user data, 2=user program, 5=supervisor data, 6=supervisor program, 7=CPU space
```

---

## Interface B: Pin-Level

### Overview

The CPU is a black box with pins. You clock it. It drives address/data/control pins.
You respond with data and acknowledge signals. All internal complexity is hidden.

### Output Pins (read after each `clock()`)

| Pin | Type | Description |
|-----|------|-------------|
| `addr` | u32 | A1-A23 address bus (active when `as_n` is false) |
| `data_out` | u16 | D0-D15 write data (valid when `as_n` false and `rw` false) |
| `fc` | u8 | FC0-FC2 function code |
| `as_n` | bool | Address strobe — false = bus cycle active |
| `uds_n` | bool | Upper data strobe (even byte / high byte) |
| `lds_n` | bool | Lower data strobe (odd byte / low byte) |
| `rw` | bool | true = read, false = write |
| `bg_n` | bool | Bus grant (active low) |
| `e` | bool | E clock output (CLK÷10) |
| `vma_n` | bool | Valid memory address (VPA response) |

### Input Pins (provide before each `clock()`)

| Pin | Type | Description |
|-----|------|-------------|
| `data_in` | u16 | D0-D15 read data (active during read cycles) |
| `dtack_n` | bool | Data transfer acknowledge — false = cycle complete |
| `berr_n` | bool | Bus error — false = abort cycle |
| `vpa_n` | bool | Valid peripheral address — false = use E-clock sync |
| `ipl0_n` | bool | Interrupt priority bit 0 (active low) |
| `ipl1_n` | bool | Interrupt priority bit 1 (active low) |
| `ipl2_n` | bool | Interrupt priority bit 2 (active low) |
| `br_n` | bool | Bus request — false = DMA wants bus |
| `bgack_n` | bool | Bus grant acknowledge — false = DMA has bus |
| `halt_n` | bool | Halt — false = freeze CPU |
| `reset_n` | bool | Reset — false = reset CPU |

### Tracing Exceptions (Bus Sniffing)

The `rs68k` core is designed for maximum performance and does not expose internal software hooks for exceptions or traps. To detect when the CPU takes an exception (e.g., Bus Error, Address Error, or a TRAP instruction), the host system must passively "sniff" the bus.

When the 68000 processes an exception, it always fetches the target PC from the exception vector table located at the bottom of memory (`0x000000` to `0x0003FC`).

To detect an exception in your host:
1. Observe a bus read cycle (`as_n` is false, `rw` is true).
2. Check if the address is within the vector table (`addr < 0x400`).
3. Confirm the CPU is in Supervisor mode accessing Data or Program space (`fc == 5` or `fc == 6`).
4. The vector number is simply `addr / 4`.

### Bus Cycle Timing

A minimum bus cycle takes 4 CLK periods. Read and write cycles differ in when
the data strobes (UDS/LDS) are asserted — this matches real 68000 silicon.

**Read cycle (4 clocks):**
```
Clock:  1     2     3     4
Phase:  S0    S2    S4    S6
        │     │     │     │
        │     │     │     └─ Data latched from bus, AS/UDS/LDS negated
        │     │     └─────── DTACK sampled (wait if not asserted)
        │     └───────────── AS asserted, UDS/LDS asserted (simultaneous)
        └─────────────────── Address and FC driven, R/W = high
```

**Write cycle (4 clocks):**
```
Clock:  1     2     3     4
Phase:  S0    S2    S4    S6
        │     │     │     │
        │     │     │     └─ AS/UDS/LDS negated (memory latches on rising edge)
        │     │     └─────── UDS/LDS asserted (DELAYED), DTACK sampled
        │     └───────────── AS asserted, write data driven on bus
        └─────────────────── Address and FC driven, R/W = low
```

The write strobe delay is critical: UDS/LDS are asserted one clock after AS,
giving the CPU time to stabilize write data before signaling memory to latch it.
External logic must use UDS/LDS (not just AS) to qualify write-enable signals.

If DTACK is not asserted at S4, the CPU inserts wait states (stays in S4) until
DTACK arrives. This is how slow memory and peripherals work.

### How to Respond to a Bus Cycle

**For reads:**
1. Observe `as_n` going false — a bus cycle has started
2. Decode `addr` and `fc` to determine the target device
3. Assert `dtack_n = false` and put data on `data_in` when ready
4. After `as_n` goes true again (cycle complete), negate `dtack_n`

**For writes:**
1. Observe `as_n` going false and `rw` = false
2. Read `data_out` — this is the write data
3. Decode `addr` and `fc`, write to target
4. Assert `dtack_n = false` to acknowledge
5. After `as_n` goes true, negate `dtack_n`

**For instant response (zero wait states):**
```rust
// Simplest possible: always acknowledge immediately
let mut inputs = Inputs68k::default();
inputs.dtack_n = false;  // always ready
inputs.data_in = memory[cpu.pins().addr];  // always provide data
cpu.clock(&inputs);
```

### DMA / Bus Arbitration

To steal bus cycles (e.g., Amiga Agnus DMA):

1. Assert `br_n = false` — request the bus
2. Wait for `bg_n` to go false — CPU grants bus
3. Assert `bgack_n = false` — acknowledge grant
4. CPU releases bus (tri-states pins). You own the bus.
5. When done: negate `bgack_n = true`
6. CPU resumes on next clock

The CPU only grants the bus between bus cycles (never mid-cycle). During internal
operations, the bus is idle and can be granted immediately.

### VPA / E-Clock Synchronization (6800 Peripherals)

For slow peripherals (CIAs on Amiga):

1. CPU starts a bus cycle (AS asserted)
2. Instead of DTACK, assert `vpa_n = false`
3. CPU waits for E clock to fall, asserts VMA
4. CPU completes cycle on next E rise
5. Total: 4 + 0..9 extra clocks depending on E phase

### Interrupts

Directly wire IPL pins:
```rust
// Level 2 interrupt (active low encoding: 2 = ~101)
inputs.ipl0_n = false;  // bit 0 = 0 → inverted = 1... 
// Actually: IPL is active-low level. Level 2 = binary 010.
// ipl2_n = true (bit2=0), ipl1_n = false (bit1=1), ipl0_n = true (bit0=0)
```

The CPU samples IPL at instruction boundaries and initiates an interrupt acknowledge
cycle (FC=7) if the level exceeds the SR interrupt mask.

### Reset

Assert `reset_n = false` for at least 10 clocks. The CPU will:
- Abort any current bus cycle
- Load SSP from address 0x000000
- Load PC from address 0x000004
- Begin execution

### Halt

Assert `halt_n = false`. The CPU freezes — no bus cycles, no internal processing.
Release to continue from where it stopped.

---

## Internal Architecture (Hidden from User)

The user never sees these details. They are documented here for maintainers.

### How clock() Drives Partial Handlers

```
clock() called
  │
  ├─ E clock: always advances (free-running ÷10)
  ├─ Reset/Halt/Arbitration: handled first
  │
  └─ Bus state machine:
       │
       ├─ Idle + no pending request:
       │    Call dispatch_partial()
       │    Handler runs internal ops (icount -= 2 each)
       │    Handler hits bus_end: sets aob/ssw/dbout/pending_byte, returns
       │    Compute internal_clocks from icount delta
       │    If bus request: start S0
       │    If instruction done: wait for next clock
       │
       ├─ Idle + internal_clocks > 0:
       │    Decrement counter, do nothing (bus idle)
       │
       ├─ S0: Drive addr/FC on pins → advance to S2
       ├─ S2: Assert AS/UDS/LDS, drive write data → advance to S4
       ├─ S4: Sample DTACK/VPA/BERR → advance to S6 or wait
       └─ S6: Latch data_in into edb, charge icount-=4,
              negate strobes → Idle, call dispatch_partial() again
```

### How the Handler Communicates Bus Requests

The partial handler sets these CPU registers before returning:
- `self.aob`: address for the bus cycle
- `self.base_ssw`: encodes space (program/data/cpu) and direction (read/write)
- `self.dbout`: write data (for writes)
- `self.pin_state.pending_byte`: byte vs word access
- `self.inst_substate`: non-zero = bus access pending, 0 = instruction complete

The pin state machine reads these to configure the bus cycle.

### Function Code Derivation

```rust
let space = self.base_ssw & 3;  // 1=data, 2=program, 3=cpu
let supervisor = self.sr & 0x2000 != 0;
let fc = match space {
    1 => if supervisor { 5 } else { 1 },  // data
    2 => if supervisor { 6 } else { 2 },  // program
    3 => 7,                                 // CPU space (interrupt ack)
    _ => 0,
};
```

### Byte Lane Selection

For byte accesses (`pending_byte = true`):
- Even address (bit 0 = 0): UDS asserted, LDS negated → data on D8-D15
- Odd address (bit 0 = 1): LDS asserted, UDS negated → data on D0-D7

For word accesses: both UDS and LDS asserted.

The address driven on the bus always has bit 0 = 0 (A0 doesn't exist on 68000).
The byte lane strobes select which half of the data bus is active.
