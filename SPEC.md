# MAME m68000 → Rust Port Specification

## 1. Goal

Produce a cycle-accurate, functionally perfect Rust port of the MAME Olivier Galibert m68000 core
(`m68000/m68000-sdf.cpp` and supporting infrastructure). The port targets embedding in a Rust-based
Amiga emulator. Verification uses the ProcessorTests suite (680x0/68000/v1/).

## 2. Source Material

| File | Role |
|------|------|
| `m68000/m68000.h` | State struct, ALU methods, helpers, enums |
| `m68000/m68000.cpp` | execute_run, device_start/reset, interrupt logic |
| `m68000/m68000-sdf.cpp` | Generated instruction handlers (direct, full) — ~4MB |
| `m68000/m68000-decode.cpp` | Packed decode table (opcode → state) |
| `m68000/m68000-head.h` | Handler function declarations |
| `m68000/m68000.lst` | Instruction list (opcode, mask, mnemonic, addressing modes) |
| `m68000/m68000gen.py` | Code generator (reference for understanding semantics) |

We port **only** the "direct full" (sdf) variant. The "partial" (sdp) variant exists for MAME's
interruptible scheduler and is not needed. The "indirect" (sif/sip) variants are for MMU dispatch
and are not needed for a 68000.

## 3. Architecture Overview

### 3.1 Execution Model

The CPU is a state machine. Each instruction is assigned a numeric state. The main loop:

```
loop {
    state = decode_table[ird]  (or special state from interrupt/exception)
    call handler for state
    handler executes micro-steps, each consuming 2 or 4 cycles
    handler sets next m_inst_state at end
}
```

Each handler is a linear sequence of operations. Bus accesses cost 4 cycles. Internal operations
cost 2 cycles. There is no branching within a handler (except for address error checks and the
MAME-specific `access_to_be_redone` / `m_icount` preemption which we strip).

### 3.2 Bus Interface

The C++ code calls:
- `m_opcodes.read_interruptible(addr)` — program space read (prefetch)
- `m_program.read_interruptible(addr)` — data space read
- `m_program.write_interruptible(addr, data)` — data space write
- `m_cpu_space.read_interruptible(addr)` — CPU space read (interrupt ack)

Each access also sets `m_base_ssw` with function code bits before the access.

### 3.3 Registers

```
da[17]: u32     — D0-D7 (0-7), A0-A6 (8-14), USP (15), SSP (16)
sp: u32         — index into da[] (15 or 16)
pc: u32         — program counter (points to word after current opcode)
ipc: u32        — instruction PC (start of current instruction)
au: u32         — address unit (general-purpose address register)
at: u32         — address temporary
aob: u32        — address output buffer (drives address bus)
dt: u32         — data temporary
int_vector: u32 — interrupt vector address

irc: u16        — instruction register capture (next prefetch word)
ir: u16         — instruction register
ird: u16        — instruction register decode (current executing opcode)
ftu: u16        — function transfer unit
dbin: u16       — data bus input
dbout: u16      — data bus output
edb: u16        — external data bus (result of last read)
aluo: u16       — ALU output
alue: u16       — ALU extension (for 32-bit ops)
alub: u16       — ALU buffer
movemr: u16     — MOVEM register mask
irdi: u16       — saved IRD for bus error frame
isr: u16        — internal status register (ALU condition codes)
sr: u16         — status register
new_sr: u16     — new SR (for RTE)
base_ssw: u16   — base special status word
ssw: u16        — special status word (for bus error frame)

dcr: u8         — divide counter register

movems: u32     — MOVEM current register index (mapped through sp)
t: i32          — temporary counter (for MUL/DIV loops)

inst_state: u16 — current handler state
next_state: u32 — next state after current instruction
```

### 3.4 ALU

~40 inline methods that operate on `m_aluo`, `m_alue`, `m_isr`. Each takes 1-2 operands and
computes result + flags. These are pure functions of their inputs + `m_sr` (for X flag) + `m_alue`
(for 32-bit ops). They are trivial to port 1:1.

### 3.5 Decode Table

A packed table of `(value, mask, state)` entries expanded at init into a 64K lookup:
`decode_table[opcode] → state_number`. State numbers 0-9 are special (RESET, BUS_ERROR,
ADDRESS_ERROR, DOUBLE_FAULT, INTERRUPT, TRACE, ILLEGAL, PRIVILEGE, LINEA, LINEF). Instruction
states start at 10.

## 4. Rust Structure

### 4.1 Crate Layout

```
rs68k_mame/
├── Cargo.toml
├── src/
│   ├── lib.rs              — public API
│   ├── cpu.rs              — M68000 struct, execute, reset, interrupt
│   ├── alu.rs              — ALU methods (impl M68000)
│   ├── decode.rs           — decode table init + packed data
│   ├── handlers.rs         — mod declaration, re-exports
│   ├── handlers/           — split generated handlers into multiple files
│   │   ├── special.rs      — reset, bus_error, address_error, etc.
│   │   ├── handlers_0.rs   — states 10..N
│   │   ├── handlers_1.rs   — states N..M
│   │   └── ...             — (split to keep files <100KB for compile times)
│   └── types.rs            — Bus trait, enums, constants
├── tests/
│   └── hartes.rs           — ProcessorTests harness
└── external/
    └── ProcessorTests/     — symlink or copy
```

### 4.2 Core Struct

```rust
pub struct M68000 {
    // Registers — exact mirror of C++ layout
    pub da: [u32; 17],
    pub sp: u32,
    pub pc: u32,
    pub ipc: u32,
    pub au: u32,
    pub at: u32,
    pub aob: u32,
    pub dt: u32,
    pub int_vector: u32,
    pub movems: u32,
    pub t: i32,

    pub irc: u16,
    pub ir: u16,
    pub ird: u16,
    pub ftu: u16,
    pub dbin: u16,
    pub dbout: u16,
    pub edb: u16,
    pub aluo: u16,
    pub alue: u16,
    pub alub: u16,
    pub movemr: u16,
    pub irdi: u16,
    pub isr: u16,
    pub sr: u16,
    pub new_sr: u16,
    pub base_ssw: u16,
    pub ssw: u16,

    pub dcr: u8,

    // State machine
    pub inst_state: u16,
    pub next_state: u32,

    // Interrupt state
    pub int_level: u32,
    pub int_next_state: u32,
    pub virq_state: u32,
    pub nmi_pending: bool,

    // Decode table
    decode_table: Vec<u16>,

    // Cycle counter for current execution
    pub cycles_used: u32,
}
```

### 4.3 Bus Trait

```rust
pub trait Bus {
    /// Read word from program space. addr is already word-aligned by caller.
    fn read_program(&mut self, addr: u32, fc: u8) -> u16;
    /// Read word from data space.
    fn read_data(&mut self, addr: u32, fc: u8) -> u16;
    /// Write word to data space.
    fn write_data(&mut self, addr: u32, data: u16, fc: u8);
    /// Read from CPU space (interrupt acknowledge).
    fn read_cpu(&mut self, addr: u32, fc: u8) -> u16;
}
```

The function code (`fc`) is derived from `base_ssw`:
- bits 0-1: space (01=data, 10=program, 11=cpu)
- bit 2: supervisor

This gives the standard m68k function codes (1-7).

### 4.4 Execution API

```rust
impl M68000 {
    pub fn new() -> Self { /* zero-init + decode table */ }
    pub fn reset(&mut self, bus: &mut impl Bus) { /* run reset sequence */ }
    pub fn execute_one(&mut self, bus: &mut impl Bus) -> u32 { /* returns cycles consumed */ }
    pub fn set_irq(&mut self, level: u8) { /* update interrupt state */ }
}
```

`execute_one` runs exactly one instruction (from the current `inst_state`) and returns the number
of cycles consumed. This is the natural unit for an Amiga emulator that needs to interleave CPU
with custom chips at specific cycle boundaries.

## 5. Porting Strategy

### 5.1 Code Generation

The `m68000-sdf.cpp` file is 4MB of generated C++. We will write a **Rust code generator** (Python
script) that:

1. Parses `m68000-sdf.cpp` (the generated C++ handlers)
2. Translates each handler function to a Rust function
3. Outputs Rust source files

The translation is mechanical because each handler is a linear sequence of:
- Register assignments (`m_au = m_au + 2` → `self.au = self.au.wrapping_add(2)`)
- ALU calls (`alu_add(a, b)` → `self.alu_add(a, b)`)
- Bus reads (`m_edb = m_opcodes.read_interruptible(m_aob & ~1)` → `self.edb = bus.read_program(self.aob & !1, fc)`)
- Bus writes (`m_program.write_interruptible(m_aob & ~1, m_dbout)` → `bus.write_data(self.aob & !1, self.dbout, fc)`)
- Cycle accounting (`m_icount -= 4` → `self.cycles_used += 4`)
- State transitions (`m_inst_state = ...` → `self.inst_state = ...`)

### 5.2 What Gets Stripped

These MAME-specific constructs are removed during translation:

| C++ construct | Reason | Replacement |
|---------------|--------|-------------|
| `m_icount` decrement + check | MAME scheduler preemption | Simple `cycles_used += N` |
| `access_to_be_redone()` | MAME bus contention retry | Removed entirely |
| `m_inst_substate` | Mid-instruction resume | Removed entirely |
| `if(m_icount <= 0) { ... return; }` | Preemption exit | Removed entirely |
| `debugger_exception_hook()` | MAME debugger | Removed (or optional callback) |
| `m_opcodes` vs `m_program` distinction | MAME address spaces | Mapped to Bus trait methods |
| `update_user_super()` | Switches MAME memory spaces | Simplified to `sp` index update |

### 5.3 What Gets Preserved Exactly

- All register assignments in exact order
- All ALU operations with exact same operands
- All bus access ordering and addresses
- Cycle counts (2 per internal step, 4 per bus access)
- Address error detection (`if(m_aob & 1)`)
- State transitions (`m_inst_state = ...`)
- `set_ftu_const()` logic
- `step_movem()` / `step_movem_predec()` logic
- Interrupt checking at instruction boundaries
- SR/CCR flag propagation from `m_isr` to `m_sr`
- The decode table (identical bit patterns)

### 5.4 Translation Rules

| C++ | Rust |
|-----|------|
| `m_da[N]` | `self.da[N]` |
| `m_da[map_sp(r)]` | `self.da[self.map_sp(r)]` |
| `m_au = m_au + 2` | `self.au = self.au.wrapping_add(2)` |
| `m_au = m_au - 2` | `self.au = self.au.wrapping_sub(2)` |
| `set_16h(m_da[N], v)` | `set_16h(&mut self.da[N], v)` |
| `set_16l(m_da[N], v)` | `set_16l(&mut self.da[N], v)` |
| `high16(m_pc)` | `high16(self.pc)` |
| `ext32(m_ftu)` | `ext32(self.ftu)` |
| `merge_16_32(h, l)` | `merge_16_32(h, l)` |
| `m_edb = m_opcodes.read_interruptible(m_aob & ~1)` | `self.edb = bus.read_program(self.aob & !1, self.fc())` |
| `m_edb = m_program.read_interruptible(m_aob & ~1)` | `self.edb = bus.read_data(self.aob & !1, self.fc())` |
| `m_program.write_interruptible(m_aob & ~1, m_dbout)` | `bus.write_data(self.aob & !1, self.dbout, self.fc())` |
| `m_icount -= 4` (after bus access) | `self.cycles_used += 4` |
| `m_icount -= 2` (internal) | `self.cycles_used += 2` |
| `if(m_aob & 1) { m_icount -= 4; m_inst_state = S_ADDRESS_ERROR; return; }` | `if self.aob & 1 != 0 { self.cycles_used += 4; self.inst_state = S_ADDRESS_ERROR; return; }` |
| `m_inst_state = m_next_state ? m_next_state : m_decode_table[m_ird]` | `self.inst_state = if self.next_state != 0 { self.next_state as u16 } else { self.decode_table[self.ird as usize] }` |

### 5.5 Handler Dispatch

C++ uses function pointer table. Rust equivalent:

```rust
impl M68000 {
    pub fn execute_one(&mut self, bus: &mut impl Bus) -> u32 {
        self.cycles_used = 0;
        self.ipc = self.pc.wrapping_sub(2);
        self.irdi = self.ird;
        self.dispatch(bus);
        self.cycles_used
    }

    fn dispatch(&mut self, bus: &mut impl Bus) {
        match self.inst_state {
            S_RESET => self.state_reset(bus),
            S_BUS_ERROR => self.state_bus_error(bus),
            S_ADDRESS_ERROR => self.state_address_error(bus),
            S_DOUBLE_FAULT => self.state_double_fault(bus),
            S_INTERRUPT => self.state_interrupt(bus),
            S_TRACE => self.state_trace(bus),
            S_ILLEGAL => self.state_illegal(bus),
            S_PRIVILEGE => self.state_privilege(bus),
            S_LINEA => self.state_linea(bus),
            S_LINEF => self.state_linef(bus),
            10 => self.ori_b_imm8_ds(bus),
            11 => self.ori_b_imm8_ais(bus),
            // ... ~900 more handlers
            _ => self.state_illegal(bus),
        }
    }
}
```

## 6. MAME-Specific Removal Details

### 6.1 The `m_icount` / Preemption System

In MAME, `execute_run()` runs until `m_icount <= 0`. Each bus access checks this and can exit
mid-instruction, saving `m_inst_substate` for later resumption via the "partial" handler.

We remove this entirely. Our `execute_one` runs a complete instruction and returns. The cycle
count is accumulated in `cycles_used` and returned to the caller.

### 6.2 `access_to_be_redone()`

This is MAME's mechanism for handling bus contention (another device holds the bus). In our
emulator, all bus accesses complete immediately. This entire mechanism is removed.

### 6.3 Address Spaces

MAME has separate `m_opcodes` and `m_program` address spaces (and user/supervisor variants).
We collapse these into the Bus trait. The function code passed to the trait method tells the
bus implementation which space is being accessed:

- FC 1 = user data
- FC 2 = user program
- FC 5 = supervisor data
- FC 6 = supervisor program
- FC 7 = CPU space (interrupt ack)

The bus implementation can use this to implement Amiga-specific behavior (e.g., CIA chip select
depends on FC).

## 7. Test Harness

### 7.1 Test Format Mapping

ProcessorTests provide:
- `initial.pc` → formal PC (address of the instruction)
- `initial.prefetch[0]` → IR = IRD (the opcode being executed)
- `initial.prefetch[1]` → IRC (next prefetched word)
- `initial.ram` → memory contents (byte-addressed)
- `final.*` → expected state after execution
- `length` → total cycles consumed

### 7.2 State Setup

```rust
fn setup_cpu(ps: &ProcessorState) -> M68000 {
    let mut cpu = M68000::new();

    // D0-D7
    cpu.da[0..8] = [ps.d0, ps.d1, ..., ps.d7];
    // A0-A6
    cpu.da[8..15] = [ps.a0, ps.a1, ..., ps.a6];
    // USP, SSP
    cpu.da[15] = ps.usp;
    cpu.da[16] = ps.ssp;

    // SR
    cpu.sr = ps.sr;
    cpu.sp = if ps.sr & 0x2000 != 0 { 16 } else { 15 };

    // Prefetch pipeline
    cpu.ir = ps.prefetch[0];
    cpu.ird = ps.prefetch[0];
    cpu.irc = ps.prefetch[1];
    cpu.dbin = ps.prefetch[1];

    // PC: internal PC = formal_pc + 2 (points past opcode)
    cpu.pc = ps.pc + 2;
    cpu.au = ps.pc + 4;
    cpu.aob = ps.pc + 4;  // or ps.pc + 2, needs verification

    // FTU from IRD
    cpu.set_ftu_const();

    // Decode
    cpu.inst_state = cpu.decode_table[cpu.ird as usize];

    cpu
}
```

### 7.3 Verification

Compare after execution:
- All 17 registers (D0-D7, A0-A6, USP, SSP)
- SR
- PC (formal PC = `cpu.pc - 2` at instruction boundary, or derived from `cpu.aob`/`cpu.au`)
- Prefetch: IR/IRD and IRC
- RAM contents
- Cycle count (`cpu.cycles_used == test.length`)

### 7.4 Bus Transaction Verification (Optional)

The test suite provides per-cycle bus transactions. We can optionally record these:

```rust
pub struct BusTransaction {
    pub rw: char,       // 'r' or 'w'
    pub cycles: u32,    // 4 for word, 4 for byte
    pub fc: u8,         // function code
    pub addr: u32,      // address
    pub size: char,     // 'w' or 'b'
    pub data: u16,      // value
}
```

This is secondary to register/memory verification but useful for debugging timing issues.

## 8. Implementation Phases

### Phase 1: Infrastructure
- Create crate skeleton
- Port `M68000` struct with all fields
- Port all ALU methods (trivial 1:1 translation)
- Port helper functions (set_16h, ext32, merge_16_32, etc.)
- Port decode table initialization
- Implement Bus trait
- Implement `set_ftu_const()`, `step_movem()`, `update_user_super()`, `update_interrupt()`

### Phase 2: Code Generator
- Write Python script to parse `m68000-sdf.cpp`
- Generate Rust handler functions
- Handle the translation rules from §5.4
- Split output into multiple files for compile time

### Phase 3: Integration
- Wire up `execute_one` dispatch
- Implement `reset` (calls the reset handler)
- Implement interrupt injection

### Phase 4: Test Harness
- Copy ProcessorTests to `external/`
- Adapt hartes.rs from rs68k for the new CPU struct
- Run tests, fix discrepancies

### Phase 5: Bus Transaction Verification
- Add optional transaction recording
- Compare against test suite transactions
- Fix any timing discrepancies

## 9. Design Options (Deviations from C++)

### OPTION A: Handler Dispatch via Match vs Function Pointer Table

**C++ approach:** Array of function pointers indexed by state number.
**Rust match:** A giant `match self.inst_state { ... }` statement.
**Rust fn pointer table:** `static HANDLERS: [fn(&mut M68000, &mut dyn Bus); N]`

**Recommendation:** Use `match`. The compiler will generate a jump table for dense ranges anyway.
Function pointer tables in Rust require either `dyn Bus` (vtable overhead) or monomorphization
tricks that complicate the code. A match statement is idiomatic, verifiable, and the compiler
optimizes it identically.

**Risk:** None. The generated assembly is equivalent.

---

### OPTION B: Bus Trait Generic vs Dynamic Dispatch

**Generic (`bus: &mut impl Bus`):** Monomorphized — zero-cost, but the entire 4MB of handlers gets
compiled per Bus implementation. Compile times will be significant.

**Dynamic (`bus: &mut dyn Bus`):** Single compilation, vtable call overhead per bus access.

**Recommendation:** Use generics. The Amiga emulator will have exactly one Bus implementation.
Compile time is a one-time cost. Runtime performance matters for a cycle-accurate emulator running
at 7+ MHz.

**Risk:** Compile times of 30-60s for the handlers. Acceptable for a release build.

---

### OPTION C: Generated Handlers vs Hand-Written Interpreter

**Generated (port m68000-sdf.cpp):** Mechanical translation, easy to verify against C++, guaranteed
cycle-accurate because it's the same code.

**Hand-written interpreter:** Could be more compact, potentially faster (shared addressing mode
code), but extremely difficult to verify cycle-for-cycle against the MAME core.

**Recommendation:** Generated. Verification is the #1 priority. You've hit the wall with deviations
before. A mechanical port of the exact same generated code eliminates semantic drift.

**Risk:** Large binary size (~4MB of handlers). Acceptable for a desktop emulator.

---

### OPTION D: Single execute_one vs Run-N-Cycles

**execute_one:** Returns after each instruction. Caller manages cycle budget.
**run_n_cycles:** Runs until cycle budget exhausted (like MAME's execute_run).

**Recommendation:** Provide both. `execute_one` for precise interleaving, `execute_n` for
batch execution when the caller knows no events are pending.

```rust
pub fn execute_n(&mut self, bus: &mut impl Bus, cycles: u32) -> u32 {
    let mut total = 0;
    while total < cycles {
        total += self.execute_one(bus);
    }
    total
}
```

**Risk:** None. `execute_n` is trivially built on `execute_one`.

---

### OPTION E: Address Error Handling

**C++ approach:** Sets `m_inst_state = S_ADDRESS_ERROR` and returns from the handler. The next
dispatch call runs the address error handler.

**Rust approach:** Same. When we detect `aob & 1` on a word access, set state and return. The
caller's next `execute_one` (or the same call if we loop internally) runs the address error
handler.

**Recommendation:** Match C++ exactly. Address error is a state transition, not an exception in
the Rust sense.

**Risk:** None.

---

### OPTION F: Wrapping Arithmetic

**C++ approach:** Relies on unsigned overflow wrapping (defined in C++ for unsigned types).
**Rust approach:** Must use `wrapping_add`, `wrapping_sub`, `wrapping_mul` explicitly.

**Recommendation:** Use wrapping ops everywhere. The code generator handles this automatically.
All address arithmetic and ALU operations use wrapping semantics.

**Risk:** Missing a wrapping op causes panic in debug builds. The test suite will catch these
immediately.

---

### OPTION G: Code Generator Source

**Option G1:** Parse `m68000-sdf.cpp` (the generated C++ output).
**Option G2:** Port `m68000gen.py` to generate Rust directly from `m68000.lst` + microcode tables.

**Recommendation:** G1 (parse the C++ output). Reasons:
1. The C++ output is the ground truth that passes MAME's own tests
2. Parsing linear C++ handlers is straightforward (regex + simple state machine)
3. Porting the generator (3000+ lines of Python with microcode/nanocode tables) is a much larger
   task with more room for subtle errors
4. If MAME updates, we can re-run our translator on the new C++ output

**Risk:** If MAME changes the output format, the parser needs updating. Low risk — the format has
been stable.

---

### OPTION H: Internal Cycle Counter Type

**C++ approach:** `m_icount` is `int` (signed), decremented. Negative means overrun.
**Rust approach:** `cycles_used: u32`, incremented. Simpler mental model.

**Recommendation:** Use `u32` incremented. The sign semantics of `m_icount` are purely for MAME's
scheduler and don't affect instruction behavior.

**Risk:** None. The cycle count is just accumulated differently.

## 10. File Size Management

The `m68000-sdf.cpp` is ~4MB with ~900 handler functions. In Rust:

- Split into ~10 files of ~400KB each (by state number ranges)
- Each file is a separate module with `impl M68000` blocks
- This keeps incremental compile times reasonable
- The match dispatch in `cpu.rs` references all handlers

## 11. Verification Strategy

### 11.1 Primary: ProcessorTests

Run all ~1,000,000 tests from the ProcessorTests suite. Each test verifies:
- All registers
- SR flags
- Memory contents
- Cycle count
- Prefetch state

Target: 100% pass rate.

### 11.2 Secondary: Bus Transactions

For debugging, compare bus transaction logs against the test suite's `transactions` field.
This catches:
- Wrong access order
- Wrong function codes
- Wrong addresses
- Wrong data values on writes

### 11.3 Tertiary: Differential Testing

Run the same instruction sequence through both the C++ MAME core and our Rust port, compare
all state after each instruction. This catches edge cases not covered by ProcessorTests.

## 12. Amiga Integration Notes

The Bus trait implementation for the Amiga will:
- Route accesses through the Amiga memory map (chip RAM, fast RAM, ROM, custom chips)
- Handle CIA access (directly memory-mapped, no FC-based decode needed for basic Amiga)
- Handle custom chip registers (DMA controller, Blitter, etc.)
- The FC value enables proper supervisor/user space separation if needed

The `execute_one` API integrates naturally with an event-driven emulator:
```rust
loop {
    let cycles = cpu.execute_one(&mut amiga_bus);
    advance_custom_chips(cycles);
    if dma_steal_cycle { /* skip CPU */ }
}
```

## 13. Summary of Non-Negotiables

1. **Cycle count must match** — every instruction must consume exactly the same number of cycles
   as the C++ original (verified by ProcessorTests `length` field)
2. **Bus access order must match** — reads and writes happen in the same sequence with the same
   addresses and function codes
3. **Register state must match** — all registers including internal pipeline state (IR, IRC, etc.)
   must be identical after each instruction
4. **Flag computation must match** — ALU operations produce identical condition codes
5. **Address error behavior must match** — detected at the same point, same state transition
6. **Prefetch pipeline must match** — IRC/IR/IRD transitions are part of the observable state

Anything that doesn't affect these six properties can be changed freely (code organization,
naming, API shape, etc.).
