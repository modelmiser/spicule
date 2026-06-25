# Calibration — does the process settle a known-good design?

The sweep settled **0 of 5** questions and recorded **6 hard incompatibilities**.
A process that settles nothing is, from its outputs alone, indistinguishable
between *"the target is genuinely impossible"* and *"the instrument is rigged to
fail."* This negative control discriminates the two.

## Method

Run the **identical** machinery (PROPOSER → 5 cold ATTACKERs → JUDGE, 3 consecutive
survivals to settle, prior-art web on) against a design known to be decidable,
sound, ergonomic, and shipping: **Rust's Non-Lexical-Lifetimes borrow checker for
the single-threaded fragment**, with the seed's hardest constraints — #2 no-seam,
#3 derived-from-proof-layer, #6 zero-cost proof erasure, #7 spec reconstruction —
**explicitly waived**. (`orchestrator/control.py`)

## Result — both directions

### Direction 1 — true negative (good design must SETTLE): ✅ SETTLED

| Revision | Verdict | Why |
|---|---|---|
| rev0 | FAILED | Soundness attacker exhibited a real single-threaded UAF — a `Drop` impl reading borrowed data (the genuine `dropck`/`#[may_dangle]` hole). The process caught an actual bug in the first writeup. |
| rev1 | SURVIVED ×3 | "Production-proven sound, proof-free, GC-free, bare-metal." Remaining attacks ruled to "puncture the author's reconstruction, not the design." Settled after 3 consecutive cold survivals. |

The process (1) settled a known-good design — so it is **not** biased to fail;
(2) still required correctness — rev0 caught a real soundness hole; (3) judged the
**design**, not the proposer's overclaims; (4) held the 3-survival rigor.

### Direction 2 — true-positive rejection (garbage must FAIL): ✅ FAILED

An obviously-unsound non-design (no checker, no ownership tracking, "programmers
just don't write bugs") run through the identical gauntlet. FAILED all 4 revisions;
the soundness attacker immediately refuted `check=⊤ ⟹ ∀e.safe(e)` with a well-typed
use-after-free. The proposer's clever defenses (trap-on-bad-deref "makes it total and
safe") were seen through and killed. The process does **not** rubber-stamp garbage.

### Conclusion

Settles the good, rejects the bad, fails the subject (0/5). The instrument measures a
property of the **subject**, not of itself — so the six incompatibilities and the
unifying theorem are trustworthy.

## Consequence — a controlled-variable result

The ONLY difference between the control (settles) and every hard question (fails) is
properties **#2 + #3 + #6**. Rust survives *because it is permitted to be a hardcoded
checker.* The moment ownership must be **derived from a unified, decidable, no-seam
proof layer with erasable proofs**, the design becomes unsound-or-undecidable — in
all five subsystems independently.

**The six incompatibilities are trustworthy signal, not artifacts.** The seed's
novel ask — derive ownership/safety from one decidable seamless proof layer — is the
specific part that does not exist, and the process has now mapped where and why it
breaks.
