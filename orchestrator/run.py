"""Entry point: drive the adversarial design process.

Usage (from the newlang/ directory):
  python3 -m orchestrator.run                 # run the next unsettled question
  python3 -m orchestrator.run --question N    # run question index N (0-based)
  python3 -m orchestrator.run --all           # run every question once, in order
  python3 -m orchestrator.run --status        # print state summary, run nothing
  python3 -m orchestrator.run --max-revisions 2

Each question runs PROPOSER → cold ATTACKERs → JUDGE, revising on FAILED up to
--max-revisions, then RECONCILES survivors against the existing design. Output
lands in design/state.json + design/DESIGN.md; raw transcripts in runs/.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

from .historian import Historian
from .round import run_round

# The first design questions to settle, in dependency order (Section 7 of seed).
QUESTIONS = [
    "Memory model — what replaces Rust's borrow checker (linear types, region "
    "inference, or something derived from the proof layer)?",
    "Decidability boundary — precisely which fragment of dependent types is "
    "admitted, with the formal argument that it is decidable?",
    "Proof erasure — how are proof witnesses eliminated at compile time with a "
    "formal guarantee they leave no runtime trace?",
    "Unsafe boundary — what does 'unsafe' mean here, and which invariants does "
    "crossing it break and then restore?",
    "Weak memory interface — how does the type system express happens-before and "
    "memory-ordering constraints under ARM/POWER/RISC-V?",
]

ROOT = Path(__file__).resolve().parent.parent
DESIGN_DIR = ROOT / "design"
RUNS_DIR = ROOT / "runs"


def next_unsettled(hist: Historian) -> int | None:
    for i, q in enumerate(QUESTIONS):
        if not hist.is_settled(q):
            return i
    return None


def print_status(hist: Historian) -> None:
    s = hist.state
    print(f"\nDesign state: {len(s['settled'])}/{len(QUESTIONS)} questions settled, "
          f"{len(s['rounds'])} rounds, {len(s['incompatibilities'])} "
          f"incompatibilities recorded, notional cost ${s['total_cost_usd']:.2f}.")
    for i, q in enumerate(QUESTIONS):
        mark = "✅" if hist.is_settled(q) else "⬚"
        print(f"  {mark} [{i}] {q[:80]}")
    if s["incompatibilities"]:
        print("\nForced incompatibilities (the payload):")
        for ic in s["incompatibilities"]:
            print(f"  🔗 [{ic['severity']}] {ic['between'][0][:40]} ✕ "
                  f"{ic['between'][1][:40]} → {ic['property_at_risk']}")
    if s["blind_spot_flags"]:
        print(f"\n⚠️  {len(s['blind_spot_flags'])} coldness/blind-spot flag(s) — "
              f"see DESIGN.md")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Adversarial language-design driver")
    ap.add_argument("--question", type=int, default=None,
                    help="run a specific question by 0-based index")
    ap.add_argument("--all", action="store_true",
                    help="run every unsettled question once, in order")
    ap.add_argument("--status", action="store_true",
                    help="print state and exit")
    ap.add_argument("--max-revisions", type=int, default=3)
    args = ap.parse_args(argv)

    hist = Historian(DESIGN_DIR)

    if args.status:
        print_status(hist)
        return 0

    if args.all:
        targets = [i for i, q in enumerate(QUESTIONS) if not hist.is_settled(q)]
    elif args.question is not None:
        if not (0 <= args.question < len(QUESTIONS)):
            ap.error(f"--question must be in [0, {len(QUESTIONS)-1}]")
        targets = [args.question]
    else:
        nxt = next_unsettled(hist)
        if nxt is None:
            print("All questions settled. Use --question N to re-open one.")
            print_status(hist)
            return 0
        targets = [nxt]

    for idx in targets:
        q = QUESTIONS[idx]
        print(f"\n>>> Driving question [{idx}]: {q}", file=sys.stderr)
        run_round(hist, q, RUNS_DIR, max_revisions=args.max_revisions)

    print_status(hist)
    print(f"\nDesign document: {DESIGN_DIR / 'DESIGN.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
