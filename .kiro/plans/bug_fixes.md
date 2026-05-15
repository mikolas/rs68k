# Bug Fix Plan

## Remaining Failures: 262 tests (0.03% of 830,695)

Fix in order of easiest/most impactful first.

---

## Bug 1: ADDQ/SUBQ.l An timing (+2 cycles) — 8 failures

**Symptom:** `SUB.l Q, A6` etc: cycles got 8 want 6.
**Opcodes:** ADDQ.l #n,An and SUBQ.l #n,An (address register direct, long).
**Expected:** 8 cycles for ADDQ/SUBQ to data register, but only 6 for address register.
**Root cause:** The 68000 skips the flag computation step for address register destinations.
The microcode likely has a shorter path for An targets.
**Investigation:** Compare C++ handler for `addq_l_ds` vs `addq_l_as` (or similar naming).
Check if there's a separate handler or if the cycle difference is in the microcode.

---

## Bug 2: MOVE.l V/N flags — 4 failures

**Symptom:** `MOVE.l D4, (d8, A3, Xn)`: SR XOR = 0x02 (V), 0x08 (N), or 0x0e (V|N|Z).
**Expected:** MOVE.l sets N and Z based on the moved value, clears V and C.
**Root cause:** Likely the flag update is using wrong operand or wrong size for comparison.
**Investigation:** Check the generated handler for MOVE.l with indexed addressing.
The ALU operation that sets flags might be using a 16-bit comparison instead of 32-bit.

---

## Bug 3: DBcc loop counter — 7 failures

**Symptom:** Wrong D register value after DBcc.
**Expected:** DBcc decrements Dn.w (word only, preserving high word) when condition is false.
**Root cause:** Possibly decrementing the full 32-bit register instead of just the low word,
or the condition evaluation is wrong for specific cases.
**Investigation:** Check the DBcc handler. Look at how the decrement is applied (set_16l vs
full assignment). Check the specific failing condition codes.

---

## Bug 4: CHK timing (-2 cycles) — 8 failures

**Symptom:** cycles got 46 want 48, got 38 want 40, etc. Always 2 short.
**Expected:** CHK exception takes N cycles; we produce N-2.
**Root cause:** Missing an internal step (istep) in the CHK exception path.
**Investigation:** Compare C++ CHK handler exception path. Look for a missing `icount -= 2`.

---

## Bug 5: TRAPV exception — 15 failures

**Symptom:** PC, SSP, prefetch, cycles all wrong. Exception not firing or firing wrong.
**Expected:** When V flag is set, TRAPV takes exception vector 7.
**Root cause:** The TRAPV handler's condition check or exception path is broken.
**Investigation:** Check the generated TRAPV handler. The `=t` condition for V flag check
might be wrong, or the exception path (goto target) is incorrect.

---

## Bug 6: DIVS/DIVU SR flags — 13 failures

**Symptom:** N flag (0x08) or Z flag (0x04) wrong after division.
**Expected:** Flags set based on quotient (N = bit 15 of quotient, Z = quotient == 0).
**Root cause:** Flags might be set based on wrong intermediate value, or overflow case
handling is incorrect.
**Investigation:** Check the division handler's flag-setting code at the end.
Compare with C++ for the specific cases (overflow vs normal completion).

---

## Bug 7: DIVS early exit — 3 failures

**Symptom:** cycles got 148 want 24. Massive overcount (124 cycles too many).
**Expected:** Division by zero or overflow should exit early (few cycles).
**Root cause:** The division loop runs to completion instead of detecting the early-exit
condition. The `cgoto` or `=t` condition for the overflow/zero check is wrong.
**Investigation:** Check the DIVS handler's initial checks. The C++ uses `m_t` to branch
out of the division loop early on overflow. Our `=t` translation might be wrong for
the specific condition used.

---

## Bug 8: ASL.b overflow edge case — 3 failures

**Symptom:** Wrong D register value and SR for ASL.b with immediate count.
**Opcode:** `e502 [ASL.b Q, D2]` — ASL.b #2, D2 (Q=2 means count from FTU).
**Root cause:** Likely the shift count extraction from FTU is wrong, or the V flag
accumulation across multiple shifts is incorrect.
**Investigation:** Check what FTU value is used. ASL.b #2 should shift by 2.
The V flag for ASL should be set if the sign bit changes during ANY shift in the sequence.

---

## Bug 9: ASR X/C flags (shift count > size) — 49 failures [MAME-INHERITED]

**Symptom:** X and C flags set when they should be clear (count > operand size).
**Real 68000 behavior:** After shifting all bits out, further shifts produce the same
result (all sign bits) but X/C should reflect the LAST bit that was actually shifted
out of the original value, not the repeated sign bit.
**Root cause:** The microcode loop calls alu_asr for each count. After `size` iterations,
the value is all-sign-bits and each further alu_asr shifts out a 1 (for negative values),
setting X/C. Real hardware apparently stops updating X/C after the value stabilizes.
**Fix approach:** In `alu_asr8`/`alu_asr`: detect when input == output (value stabilized)
and don't set X/C in that case. Or: check if input is 0x00 or 0xFF before setting flags.
**Note:** This diverges from MAME C++ behavior. Document as intentional improvement.

---

## Fix Order

1. Bug 5 (TRAPV) — likely a simple condition or goto issue, 15 failures
2. Bug 1 (ADDQ/SUBQ.l An) — timing issue, 8 failures
3. Bug 3 (DBcc) — counter issue, 7 failures
4. Bug 4 (CHK timing) — missing step, 8 failures
5. Bug 2 (MOVE.l flags) — flag computation, 4 failures
6. Bug 8 (ASL.b) — edge case, 3 failures
7. Bug 6 (DIVS/DIVU flags) — flag computation, 13 failures
8. Bug 7 (DIVS early exit) — loop condition, 3 failures
9. Bug 9 (ASR X/C) — MAME improvement, 49 failures

Total fixable: ~113 failures (excluding 90 address-error state + 49 ASR if deferred)
