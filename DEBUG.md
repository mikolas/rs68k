# Debugging Facilities — Specification

## Principle

The CPU core contains no debug code. All debugging is external — implemented as
wrappers, observers, and hooks that sit between the user and the core. Zero cost
when not used.

## Components

### 1. Debugger State

```rust
pub struct Debugger {
    breakpoints: Vec<Breakpoint>,
    watchpoints: Vec<Watchpoint>,
    trace: TraceBuffer,
    stop_reason: Option<StopReason>,
}
```

### 2. Breakpoints (execution)

Trigger when the CPU is about to execute an instruction at a given address.

```rust
pub struct Breakpoint {
    pub id: u32,
    pub addr: u32,
    pub enabled: bool,
    pub condition: Option<Box<dyn Fn(&M68000) -> bool>>,
    pub hit_count: u64,
    pub ignore_count: u64,  // skip first N hits
}
```

Checked before each instruction dispatch by comparing `cpu.pc.wrapping_sub(2)`
(the instruction address) against the breakpoint list.

### 3. Watchpoints (memory access)

Trigger when a specific address range is read or written during execution.

```rust
pub enum WatchKind { Read, Write, ReadWrite }

pub struct Watchpoint {
    pub id: u32,
    pub addr_start: u32,
    pub addr_end: u32,     // inclusive
    pub kind: WatchKind,
    pub enabled: bool,
    pub condition: Option<Box<dyn Fn(&M68000, u32, u16) -> bool>>,  // addr, data
    pub hit_count: u64,
}
```

Implemented via a Bus wrapper that checks every access against the watchpoint list.

### 4. Trace Buffer

Ring buffer of executed instructions for post-mortem analysis.

```rust
pub struct TraceEntry {
    pub pc: u32,           // instruction address
    pub opcode: u16,       // first word (IRD)
    pub sr: u16,           // status register before execution
    pub cycles: u32,       // cycles consumed
    pub da: [u32; 17],     // registers after execution (optional, expensive)
}

pub struct TraceBuffer {
    entries: Vec<TraceEntry>,
    head: usize,
    capacity: usize,
    enabled: bool,
    full_regs: bool,       // whether to capture all registers (slower)
}
```

### 5. Stop Reasons

```rust
pub enum StopReason {
    Breakpoint(u32),           // breakpoint id
    Watchpoint(u32, u32, u16), // watchpoint id, addr, data
    Step,                      // single-step complete
    Exception(u16),            // CPU entered exception state N
    Halted,                    // CPU executed STOP or HALT
}
```

## Bus Wrapper (Full Handler Mode)

```rust
pub struct DebugBus<'a, B: Bus> {
    inner: &'a mut B,
    debugger: &'a mut Debugger,
}

impl<B: Bus> Bus for DebugBus<'_, B> {
    fn read_program(&mut self, addr: u32) -> u16 {
        let val = self.inner.read_program(addr);
        self.debugger.check_read(addr, val);
        val
    }
    fn read_data(&mut self, addr: u32) -> u16 {
        let val = self.inner.read_data(addr);
        self.debugger.check_read(addr, val);
        val
    }
    fn write_data(&mut self, addr: u32, data: u16) {
        self.debugger.check_write(addr, data);
        self.inner.write_data(addr, data);
    }
    fn write_data_byte(&mut self, addr: u32, data: u8) {
        self.debugger.check_write(addr, data as u16);
        self.inner.write_data_byte(addr, data);
    }
    fn read_cpu(&mut self, addr: u32) -> u16 {
        self.inner.read_cpu(addr)
    }
}
```

## Execution Wrapper

```rust
pub enum RunResult {
    Continue,
    Stop(StopReason),
}

impl Debugger {
    /// Execute one instruction with debug checks.
    pub fn step(&mut self, cpu: &mut M68000, bus: &mut impl Bus) -> RunResult {
        let pc = cpu.pc.wrapping_sub(2);
        if let Some(bp) = self.check_breakpoints(pc, cpu) {
            return RunResult::Stop(StopReason::Breakpoint(bp));
        }
        let mut dbg_bus = DebugBus { inner: bus, debugger: self };
        let cycles = cpu.execute_one(&mut dbg_bus);
        if self.trace.enabled {
            self.trace.push(pc, cpu.ird, cpu.sr, cycles, cpu);
        }
        if let Some(reason) = self.stop_reason.take() {
            return RunResult::Stop(reason);
        }
        RunResult::Continue
    }

    /// Run until breakpoint, watchpoint, or N instructions.
    pub fn run(&mut self, cpu: &mut M68000, bus: &mut impl Bus, max_instructions: u64) -> StopReason {
        for _ in 0..max_instructions {
            match self.step(cpu, bus) {
                RunResult::Stop(reason) => return reason,
                RunResult::Continue => {}
            }
        }
        StopReason::Step  // budget exhausted
    }
}
```

## Pin-Level Mode

No wrapper needed. The user controls every clock and sees every bus cycle on the pins.
Debug checks happen in the user's own loop:

```rust
loop {
    cpu.clock(&inputs);

    // PC breakpoint: check at instruction boundaries
    if cpu.inst_substate == 0 {
        let pc = cpu.pc.wrapping_sub(2);
        if breakpoints.contains(&pc) { break; }
    }

    // Memory watchpoint: check during active bus cycles
    if !cpu.pins().as_n {
        let addr = cpu.pins().addr;
        if watchpoints.matches(addr, cpu.pins().rw) { break; }
    }
}
```

The debugger module provides helper functions for this but doesn't own the loop.

## File Layout

```
src/debug.rs        — Debugger struct, Breakpoint, Watchpoint, TraceBuffer, StopReason
src/debug_bus.rs    — DebugBus wrapper (impl Bus with watchpoint checks)
```

Both are optional — gated behind a `debug` feature flag so they don't affect
release builds that don't need them.

```toml
[features]
default = []
debug = []
```

## Performance Considerations

- Breakpoint check: linear scan of a small list (typically <10 entries). No overhead
  when list is empty (early return).
- Watchpoint check: called on every bus access. For large watchpoint lists, use a
  sorted array with binary search or a small hash set.
- Trace buffer: fixed-size ring buffer, no allocation during execution. Full register
  capture is optional (costs one array copy per instruction).
- When no debugger is used: call `cpu.execute_one()` directly. Zero overhead.

## API Summary

```rust
// Setup
let mut dbg = Debugger::new();
dbg.add_breakpoint(0x1000);
dbg.add_watchpoint(0xDFF000, 0xDFF1FF, WatchKind::Write);
dbg.trace.enable(1024);  // 1024-entry ring buffer

// Run
let reason = dbg.run(&mut cpu, &mut bus, 1_000_000);
match reason {
    StopReason::Breakpoint(id) => println!("Hit breakpoint {} at {:06x}", id, cpu.pc.wrapping_sub(2)),
    StopReason::Watchpoint(id, addr, data) => println!("Watch {} at {:06x} = {:04x}", id, addr, data),
    _ => {}
}

// Inspect trace
for entry in dbg.trace.iter_recent(10) {
    println!("{:06x}: {:04x} sr={:04x} cyc={}", entry.pc, entry.opcode, entry.sr, entry.cycles);
}
```
