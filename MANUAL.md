# rs68k — Integration Manual

## Overview

rs68k is a cycle-exact Motorola 68000 CPU emulator ported from MAME's Olivier Galibert
microcode-based core. It provides two execution modes:

1. **Full handlers** (`execute_one`) — runs one complete instruction per call. Fast path for
   sequential emulators.
2. **Pin-level partial handlers** (`dispatch_partial`) — suspends at each bus access boundary.
   For cycle-exact parallel emulation where CPU and other hardware advance together.

## Integration

### Crate Dependency

```toml
[dependencies]
rs68k_mame = { path = "../rs68k" }
```

### Mode 1: Instruction-at-a-Time (Full Handlers)

For emulators that interleave CPU and other devices at instruction boundaries.

```rust
use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;

struct MyBus { /* your memory/IO implementation */ }

impl Bus for MyBus {
    fn read_program(&mut self, addr: u32) -> u16 { /* word read from program space */ }
    fn read_data(&mut self, addr: u32) -> u16 { /* word read from data space */ }
    fn write_data(&mut self, addr: u32, data: u16) { /* word write */ }
    fn write_data_byte(&mut self, addr: u32, data: u8) { /* byte write */ }
    fn read_cpu(&mut self, addr: u32) -> u16 { /* interrupt acknowledge */ }
}

let mut cpu = M68000::new();
let mut bus = MyBus::new();

// Reset sequence (set up initial PC/SP from vectors)
// ... set cpu.pc, cpu.sr, cpu.sp, cpu.ir, cpu.ird, cpu.irc, cpu.inst_state ...

loop {
    let cycles = cpu.execute_one(&mut bus);
    advance_other_hardware(cycles);
}
```

### Mode 2: Pin-Level (Partial Handlers)

For cycle-exact emulators where all hardware runs in parallel. The CPU never calls
bus read/write functions — it just signals what it needs, and the external system
performs the access.

```rust
use rs68k_mame::cpu::M68000;
use rs68k_mame::types::*;

let mut cpu = M68000::new();

// Main emulation loop — called once per clock cycle (7.09 MHz for Amiga)
fn tick(cpu: &mut M68000, memory: &mut Memory, agnus: &mut Agnus) {
    // 1. Advance all hardware by one clock
    agnus.clock();

    // 2. If CPU has a pending bus request and bus is available:
    if cpu.inst_substate != 0 && agnus.cpu_bus_available() {
        // Perform the bus access
        let is_read = cpu.base_ssw & SSW_R != 0;
        let is_byte = cpu.pin_state.pending_byte;
        let addr = cpu.aob;

        if is_read {
            // Read: latch data into edb
            cpu.edb = memory.read_word(addr & !1u32);
        } else {
            // Write: put dbout onto bus
            if is_byte {
                if addr & 1 != 0 { memory.write_byte(addr, cpu.dbout as u8); }
                else { memory.write_byte(addr, (cpu.dbout >> 8) as u8); }
            } else {
                memory.write_word(addr & !1u32, cpu.dbout);
            }
        }
        cpu.icount -= 4;

        // Resume the handler (processes the result, runs until next bus access)
        cpu.dispatch_partial(&mut DummyBus);
    }
    // 3. If CPU needs to start work (no pending bus request, instruction boundary):
    else if cpu.inst_substate == 0 {
        // Run handler — it will do internal ops then request a bus access
        cpu.ipc = cpu.pc.wrapping_sub(2);
        cpu.irdi = cpu.ird;
        cpu.dispatch_partial(&mut DummyBus);
    }
    // 4. If bus not available (DMA stealing): CPU stalls, do nothing
}

// DummyBus — never called, but required by the trait signature
struct DummyBus;
impl Bus for DummyBus {
    fn read_program(&mut self, _: u32) -> u16 { 0 }
    fn read_data(&mut self, _: u32) -> u16 { 0 }
    fn write_data(&mut self, _: u32, _: u16) {}
    fn write_data_byte(&mut self, _: u32, _: u8) {}
    fn read_cpu(&mut self, _: u32) -> u16 { 0 }
}
```

### Key Registers for Bus Interface

| Register | Type | Meaning |
|----------|------|---------|
| `cpu.aob` | u32 | Address output buffer — the address for the bus cycle |
| `cpu.base_ssw` | u16 | Bus cycle type: `& SSW_R` = read, `& 3` = space (1=data, 2=program, 3=cpu) |
| `cpu.dbout` | u16 | Data output — write data for the bus cycle |
| `cpu.edb` | u16 | External data bus — fill this with read data before resuming |
| `cpu.pin_state.pending_byte` | bool | True if byte access (assert only UDS or LDS) |
| `cpu.inst_substate` | u16 | 0 = at instruction boundary, non-zero = waiting for bus access |
| `cpu.icount` | i32 | Cycle counter — decrement by 4 after each bus access |

### SSW Space Decoding

```rust
let space = cpu.base_ssw & 3;
// space == 1: data space (SSW_DATA)
// space == 2: program space (SSW_PROGRAM)
// space == 3: CPU space (SSW_CPU) — interrupt acknowledge

let is_read = cpu.base_ssw & SSW_R != 0;  // SSW_R = 0x10
let is_supervisor = cpu.sr & 0x2000 != 0;

// Function code (for Amiga: FC determines chip select)
let fc = (space & 3) | if is_supervisor { 4 } else { 0 };
```

### Internal Cycles

Between bus accesses, the handler may execute internal operations (ALU, register moves).
These consume 2 clocks each and are reflected in `icount`. The handler runs all internal
ops in one burst before requesting the next bus access. During internal cycles, the bus
is idle — no pins are driven, DMA can use the bus.

To determine how many internal clocks elapsed:
```rust
let icount_before = cpu.icount;
cpu.dispatch_partial(&mut bus);
let internal_clocks = icount_before - cpu.icount;  // 2 per internal step
// If inst_substate != 0, a bus access follows (4 more clocks)
```

### Interrupts

```rust
cpu.set_irq(level);  // level 0-7, call when IPL pins change
```

Interrupts are checked at instruction boundaries (when `inst_substate` returns to 0
and `inst_state` transitions to the next instruction).

## Testing

### Prerequisites

```bash
# Clone with submodules (SingleStepTests)
git clone --recursive <repo-url>
# Or if already cloned:
git submodule update --init
```

### Run Full Handler Tests (127 instructions, 317,500 vectors)

```bash
cargo test --test sst
```

### Run Partial Handler Tests (127 instructions, 63,500 vectors)

```bash
cargo test --test partial partial_sst_all -- --nocapture
```

This runs all 127 instruction files (500 vectors each) through the pin-level partial
adapter and verifies results match the full handlers exactly.

### Run Basic Partial Tests (quick sanity check)

```bash
cargo test --test partial
```

### Run Infrastructure Tests

```bash
cargo test --test verify_infra
```

## Benchmarking

### Full Handlers

```bash
cargo run --release --example bench
```

### Partial Handlers (with full comparison)

```bash
cargo run --release --example bench_partial
```

Output shows MIPS, effective MHz, Amiga headroom multiplier, and partial/full ratio.

### Typical Results (modern x86)

| Mode | MIPS | Effective MHz | Amiga Headroom |
|------|------|---------------|----------------|
| Full handlers | ~100+ | ~900+ | ~130x |
| Partial handlers | ~70+ | ~630+ | ~90x |

(Results vary by CPU. The benchmark runs a tight ADDQ/BRA loop.)

## Regenerating Handlers

If you modify `gen_rust.py`:

```bash
# Regenerate both full and partial handlers
python3 gen_rust.py m68000/m68000.lst src/handlers/ --partial src/handlers_partial/

# Verify
cargo build
cargo test --test sst sst_nop        # quick full handler check
cargo test --test partial partial_nop  # quick partial handler check
cargo test --test partial partial_sst_all -- --nocapture  # full partial verification
```

## Architecture Notes

- All handlers are generated from the same microcode IR (same as MAME's m68000gen.py)
- Full handlers call `bus.read_*()` / `bus.write_*()` inline
- Partial handlers never call bus functions — they set up `aob`/`ssw`/`dbout` and return
- The `Bus` trait parameter in partial handlers is unused but kept for type compatibility
- Cross-substate gotos in partial handlers use unique substate IDs to avoid re-executing address error checks
- Internal cycles (`icount -= 2`) execute in a burst; only bus accesses are suspend points

## Mode 3: Pin-Level clock() Interface

For drop-in 68000 replacement with real bus signals. One call per CLK period.

```rust
use rs68k::cpu::M68000;
use rs68k::pins::{Inputs68k, Pins68k};

let mut cpu = M68000::new();
// ... set up initial state (reset vector fetch) ...

loop {
    let pins = cpu.pins();

    // Build inputs based on current pin state
    let mut inputs = Inputs68k::default();

    // Address decode: provide data for reads
    if !pins.as_n && pins.rw {
        inputs.data_in = memory_read(pins.addr);
        inputs.dtack_n = false; // acknowledge
    }

    // Capture writes
    if !pins.as_n && !pins.rw {
        memory_write(pins.addr, pins.data_out);
        inputs.dtack_n = false;
    }

    // Slow device: delay DTACK for wait states
    if is_slow_device(pins.addr) && !pins.as_n {
        inputs.dtack_n = !device_ready();
    }

    // 6800 peripheral (CIA): assert VPA instead of DTACK
    if is_cia_address(pins.addr) && !pins.as_n {
        inputs.dtack_n = true;  // no DTACK
        inputs.vpa_n = false;   // use E-clock sync
    }

    // Interrupts
    inputs.ipl0_n = !(irq_level & 1 != 0);
    inputs.ipl1_n = !(irq_level & 2 != 0);
    inputs.ipl2_n = !(irq_level & 4 != 0);

    cpu.clock(&inputs);
}
```

The `clock()` method handles all internal complexity:
- S0-S7 bus state machine (4 CLK per bus cycle minimum)
- Wait state insertion (stays in S4 until DTACK)
- E clock generation (free-running CLK÷10)
- VPA/VMA handshake for 6800 peripherals
- Bus arbitration (BR/BG/BGACK)
- Halt and reset

See `BUS_INTERFACE.md` for the complete pin specification.
