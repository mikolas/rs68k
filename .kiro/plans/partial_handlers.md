# Partial Handler Generation Plan

## What the C++ Partial Handler Is

The partial handler (`sdp`) is the SAME code as the full handler (`sdf`) with these additions:

1. **`switch(m_inst_substate) { case 0:`** at the top of the function
2. **`[[fallthrough]]; case N:`** before each bus access (read/write)
3. **`[[fallthrough]]; case N+1:`** after each bus access (before address error check)
4. **`m_inst_substate = 0;`** at address error transitions and instruction end
5. **`}`** closing the switch at the end

The `cid` (case ID) is assigned by `propagate_bus_access()` in the IR — it's already
in the `bus_end` CI as `ci[1]`. Each bus access gets two consecutive IDs: one before
the access (resume point for retry) and one after (resume point for post-access check).

## How It Works

When called with `inst_substate = 0`: runs from the beginning.
When called with `inst_substate = N`: jumps directly to `case N` (C++ fallthrough).

The handler runs linearly until it hits a bus access. At that point:
- It performs the bus access
- Decrements icount by 4
- Checks if icount <= 0 (time slice exhausted)
- If yes: saves `inst_substate` to the resume point and returns
- If no: falls through to the next case and continues

For our pin-level use: we DON'T use the icount check. Instead, the handler always
returns after each bus access. The pin state machine then drives the bus signals for
4 clocks. When complete, it calls the handler again with the next substate.

## Differences from Full Handler in gen_rust.py

The full handler (`generate_rust_source_from_code` with `GEN.full`):
- No switch/match at top
- No case labels at bus accesses
- No substate save/return at bus accesses
- No substate reset at end

The partial handler (without `GEN.full`):
- `match self.inst_substate {` at top
- Each bus_end emits: save substate, return (suspend point)
- After bus_end: resume point (next match arm)
- Address error: `self.inst_substate = 0; return;`
- Instruction end: `self.inst_substate = 0;`

## Implementation in gen_rust.py

### Option A: Separate generation function

Add `generate_rust_partial_source_from_code(code)` that emits the partial variant.
This is a copy of `generate_rust_source_from_code` with the partial-mode additions.

### Option B: Flag parameter

Add a `partial: bool` parameter to `generate_rust_source_from_code`. When true,
emit the substate scaffolding.

**Choose Option B** — less code duplication.

## Exact Changes Needed

### 1. Function signature
```python
def generate_rust_source_from_code(code, partial=False):
```

### 2. Top of function (after rx/ry declarations)
```python
if partial:
    source.append("        match self.inst_substate {")
    source.append("        0 => {")
```

### 3. In `emit_bus_end` — before the bus access
```python
if partial:
    lines.append("} // end current substate")
    lines.append("%d => { // resume: bus access" % cid)
```

Actually no — the C++ uses `[[fallthrough]]` which means the code flows linearly.
In Rust, we can't fallthrough in match. We need a different approach.

### Rust Approach: Linear Code with Labeled Resume Points

Instead of a match, use a jump-to-label approach. In Rust, the cleanest way:

```rust
pub fn handler_xxx_partial(&mut self, bus: &mut impl Bus) {
    if self.inst_substate >= 1 { /* skip to substate 1 */ }
    if self.inst_substate >= 2 { /* skip to substate 2 */ }
    // etc.
}
```

No — that's terrible. The C++ uses switch fallthrough which is essentially goto.

### Rust Approach: Match with Explicit Fallthrough

```rust
pub fn handler_xxx_partial(&mut self, bus: &mut impl Bus) {
    let mut substate = self.inst_substate;
    loop {
        match substate {
            0 => {
                // code before first bus access
                substate = 1;
            }
            1 => {
                // bus access
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 2;
                return; // SUSPEND — bus cycle in progress
            }
            2 => {
                // after bus access (address error check)
                if self.aob & 1 != 0 {
                    self.icount -= 4;
                    self.inst_state = S_ADDRESS_ERROR;
                    self.inst_substate = 0;
                    return;
                }
                self.dbin = self.edb;
                // code until next bus access
                substate = 3;
            }
            3 => {
                // next bus access
                self.edb = bus.read_program(self.aob & !1u32);
                self.icount -= 4;
                self.inst_substate = 4;
                return; // SUSPEND
            }
            4 => {
                // after second bus access
                // ... final code ...
                self.inst_state = ...;
                self.inst_substate = 0;
                return; // DONE
            }
            _ => return,
        }
    }
}
```

Each bus access is a suspend point. The handler returns after initiating the access.
The pin-level `clock()` drives the bus for 4 clocks, then calls the handler again.
The handler resumes at the next substate.

### Key Observations from C++

1. **Substate IDs come from `ci[1]` (the `bid` field in bus_end)**. They're assigned
   by `propagate_bus_access()` and are always sequential odd numbers: 1, 3, 5, 7...
   (bus access at cid, post-check at cid+1).

2. **Goto-based handlers still use the same substate mechanism**. The gotos are within
   a single substate block — they don't cross bus access boundaries. So the partial
   handler's match arms align with bus accesses, not with goto labels.

3. **Internal cycles (istep) are NOT suspend points**. Only bus accesses suspend.
   Internal cycles just decrement icount and continue.

4. **The handler always returns at bus accesses in partial mode**. In MAME, it returns
   only when icount <= 0. For our pin-level interface, it ALWAYS returns (every bus
   access is a suspend point regardless of icount).

## Substate Assignment

From the IR, each `bus_end` CI has `ci[1]` = bid. The substates are:
- `bid`: resume point BEFORE the bus access (for retry after access_to_be_redone)
- `bid+1`: resume point AFTER the bus access (for post-access code)

For our pin-level interface, we only need `bid+1` (resume after access completes).
The "retry" case (`access_to_be_redone`) doesn't apply — we always complete the access.

Simplified for pin-level:
- Substate 0: start of instruction
- Substate N (odd): bus access initiated, waiting for completion
- Substate N+1 (even): bus access complete, continue execution

## Integration with clock()

```
clock() called:
  if bus_phase == Idle:
    call partial handler
    handler runs until bus access, sets inst_substate, returns
    pin state machine starts bus cycle (S0)
  if bus_phase == S0/S2/S4/S6:
    advance bus state machine
    when S6 complete (data latched):
      bus_phase = Idle
      call partial handler again (resumes from inst_substate)
```

## Files to Generate

- `src/handlers_partial/` — partial handler files (same structure as `src/handlers/`)
- `src/handlers_partial/dispatch.rs` — dispatch for partial handlers

The M68000 struct gets a `dispatch_partial()` method that calls the partial handlers.
The `clock()` method uses `dispatch_partial()`. The `execute_one()` method continues
using the full handlers (fast path).

## Summary

1. Add `partial` flag to `generate_rust_source_from_code`
2. When partial: emit `loop { match substate { ... } }` structure
3. Each `bus_end` becomes a suspend/resume point (two match arms)
4. Internal cycles and ALU ops stay inline (no suspend)
5. Gotos stay within match arms (same as full handler's loop/match)
6. Generate into `src/handlers_partial/`
7. Wire `clock()` to use partial dispatch
