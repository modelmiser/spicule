"""Calibration controls — does the process recognize good AND bad designs?

The sweep settled 0/5. A 0/N result is only trustworthy if the instrument is shown
to be calibrated in BOTH directions:

  --case good  (true negative): a known decidable+sound+shipping design, hard
               constraints waived (Rust NLL). A calibrated process SETTLES it.
               -> ran 2026-06-24: SETTLED (rev0 caught a real dropck UAF, rev1 ×3).

  --case bad   (true-positive rejection): an obviously unsound non-design. A
               calibrated process FAILS it. If it SURVIVES, the process rubber-stamps
               garbage and the whole map is worthless.

Both run the identical gauntlet on a throwaway Historian.
"""
from __future__ import annotations

import argparse
import tempfile
from pathlib import Path

from .historian import Historian
from .round import run_round

CASES = {
    "good": (
        "should SETTLE",
        "NEGATIVE CONTROL (calibration) — Propose Rust's borrow checker exactly as "
        "shipped in rustc (Non-Lexical Lifetimes / Polonius-style region inference) for "
        "the SINGLE-THREADED fragment: moves, shared vs. unique borrows, lifetimes as "
        "region constraints solved by dataflow. "
        "FOR THIS QUESTION ONLY, these target properties are WAIVED and MUST NOT be held "
        "against the proposal: the unified no-seam proof/systems layer; ownership being "
        "*derived* from a proof layer (a HARDCODED compiler analysis is explicitly "
        "acceptable and expected here); zero-cost *proof* erasure; and full-spec AI "
        "reconstruction. The ONLY properties in scope are: memory safety without GC, "
        "DECIDABLE checking that needs no user-written proofs, ergonomics no worse than "
        "Rust (it IS Rust), and bare-metal targeting. This is a known, production-proven, "
        "decidable, sound design that hundreds of thousands of programmers use daily; its "
        "few gaps (self-referential structs, intrusive lists) are exactly where Rust uses "
        "`unsafe`, i.e. co-located with unsafe. A correctly-calibrated adversarial process "
        "should find that it SURVIVES."
    ),
    "bad": (
        "should FAIL",
        "POSITIVE-REJECTION CONTROL (calibration) — Propose the following memory model "
        "and defend it as sound: there is NO borrow checker, NO ownership tracking, and "
        "NO runtime checks. Any reference may be freely copied, aliased, mutated, and "
        "freed at any time; use-after-free and double-free are 'the programmer's "
        "responsibility.' Memory safety is achieved by the convention that programmers "
        "simply do not write bugs. Claim this satisfies memory safety without GC, "
        "decidable checking (the checker is the empty function, trivially decidable), "
        "zero cost, and bare-metal targeting. "
        "This design is OBVIOUSLY unsound. A correctly-calibrated adversarial process "
        "MUST find that it FAILS. If it survives, the process rubber-stamps garbage."
    ),
}


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Calibration controls")
    ap.add_argument("--case", choices=list(CASES), default="good")
    args = ap.parse_args(argv)
    expect, question = CASES[args.case]

    design = Path(tempfile.mkdtemp(prefix=f"spicule_control_{args.case}_design_"))
    runs = Path(tempfile.mkdtemp(prefix=f"spicule_control_{args.case}_runs_"))
    hist = Historian(design)
    run_round(hist, question, runs, max_revisions=3, survivals=3, prior_art_web=True)
    settled = hist.is_settled(question)

    print("\n" + "=" * 64)
    print(f"CONTROL case={args.case} (expected: {expect})")
    if args.case == "good":
        ok = settled
        verdict = ("✅ SETTLED — not biased to fail; map trustworthy" if settled
                   else "❌ FAILED — instrument too harsh; map partly artifact")
    else:  # bad
        ok = not settled
        verdict = ("✅ FAILED as required — rejects garbage" if not settled
                   else "❌ SETTLED — instrument rubber-stamps garbage; map worthless")
    print(f"{'PASS' if ok else 'FAIL'}: {verdict}")
    print(f"transcripts: {runs}")
    print("=" * 64)
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
