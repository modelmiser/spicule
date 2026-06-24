"""One adversarial round (Section 5 'Round Structure' of the seed).

  1. PROPOSER generates decision D (with history + any prior defeating attacks)
  2. all ATTACKERs run cold against D, in parallel
  3. JUDGE evaluates each attack -> SURVIVED / FAILED / TRADEOFF
  4. if SURVIVED: RECONCILER checks D against each already-settled decision and
     records forced incompatibilities (the payload)
  5. HISTORIAN records everything; raw transcripts dumped for audit

Coldness alarm: if every attacker self-reports SURVIVES/none, we flag the round
— per the seed, attackers that never find anything mean the prompts aren't cold.
"""
from __future__ import annotations

import json
import re
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

from . import roles
from .claude_cli import call
from .historian import Historian


def _log(msg: str) -> None:
    print(msg, file=sys.stderr, flush=True)


def extract_json_block(text: str) -> dict | None:
    """Pull the last JSON object out of a role's free-form reply, leniently."""
    fenced = re.findall(r"```(?:json)?\s*(\{.*?\})\s*```", text, re.DOTALL)
    candidates = list(fenced)
    if not candidates:
        # last brace-balanced-ish object as a fallback
        candidates = re.findall(r"(\{(?:[^{}]|\{[^{}]*\})*\})", text, re.DOTALL)
    for c in reversed(candidates):
        try:
            return json.loads(c)
        except json.JSONDecodeError:
            continue
    return None


def _run_attackers(proposal_text: str, runs_dir: Path, round_no: int) -> list[dict]:
    def one(role: str) -> dict:
        r = call(
            roles.build_attacker_prompt(proposal_text),
            roles.ROLE_SYSTEM[role],
            label=f"R{round_no} {role}",
        )
        parsed = extract_json_block(r.text) or {}
        (runs_dir / f"r{round_no}_{role}.md").write_text(r.text)
        return {
            "role": role,
            "text": r.text,
            "verdict": parsed.get("verdict", "?"),
            "severity": parsed.get("severity", "?"),
            "summary": parsed.get("summary", ""),
            "strongest_point": parsed.get("strongest_point", ""),
            "cost_usd": r.cost_usd,
        }

    # Cold attackers are independent => true parallelism (subprocess I/O bound).
    with ThreadPoolExecutor(max_workers=len(roles.ATTACKER_ROLES)) as ex:
        return list(ex.map(one, roles.ATTACKER_ROLES))


def _reconcile(new_decision: dict, hist: Historian, runs_dir: Path,
               round_no: int) -> list[dict]:
    found = []
    for prior in hist.settled():
        if prior["question"] == new_decision["question"]:
            continue
        r = call(
            roles.build_reconciler_prompt(prior, new_decision),
            roles.ROLE_SYSTEM["reconciler"],
            label=f"R{round_no} reconcile vs [{prior['question']}]",
        )
        parsed = extract_json_block(r.text) or {}
        slug = re.sub(r"\W+", "-", prior["question"])[:30]
        (runs_dir / f"r{round_no}_reconcile_{slug}.md").write_text(r.text)
        if parsed.get("incompatible") and parsed.get("severity") in ("hard", "soft"):
            found.append({
                "between": [prior["question"], new_decision["question"]],
                "severity": parsed.get("severity"),
                "property_at_risk": parsed.get("property_at_risk", "?"),
                "description": parsed.get("description", ""),
                "round_detected": round_no,
            })
    return found


def run_round(hist: Historian, question: str, runs_dir: Path,
              max_revisions: int = 3) -> dict:
    """Run one design question to a verdict, revising the PROPOSER on FAILED."""
    runs_dir.mkdir(parents=True, exist_ok=True)
    round_no = len(hist.state["rounds"]) + 1
    prior_attacks: list[dict] | None = None
    revision = hist.revisions_for(question)

    while True:
        _log(f"\n=== ROUND {round_no} · [{question}] · revision {revision} ===")

        # 1. PROPOSER
        prop = call(
            roles.build_proposer_prompt(question, hist.settled(), prior_attacks),
            roles.ROLE_SYSTEM["proposer"],
            label=f"R{round_no} proposer (rev{revision})",
        )
        proposal_text = prop.text
        (runs_dir / f"r{round_no}_proposal_rev{revision}.md").write_text(proposal_text)

        # 2. ATTACKERS (parallel, cold)
        attacks = _run_attackers(proposal_text, runs_dir, round_no)

        # coldness alarm
        if all(a["verdict"] == "SURVIVES" and a["severity"] in ("none", "?")
               for a in attacks):
            note = ("all 5 attackers reported SURVIVES/none — prompts may not be "
                    "cold enough, or a correlated blind spot")
            hist.flag_blind_spot(note, round_no)
            _log(f"  ⚠️  COLDNESS ALARM: {note}")

        # 3. JUDGE
        judge = call(
            roles.build_judge_prompt(question, proposal_text, attacks),
            roles.ROLE_SYSTEM["judge"],
            label=f"R{round_no} judge",
        )
        verdict = extract_json_block(judge.text) or {}
        (runs_dir / f"r{round_no}_judge.md").write_text(judge.text)
        decision = (verdict.get("decision") or "FAILED").upper()
        tradeoffs = verdict.get("tradeoffs", []) or []
        headline = verdict.get("headline", "")

        round_cost = (prop.cost_usd + judge.cost_usd
                      + sum(a["cost_usd"] for a in attacks))

        round_rec = {
            "round": round_no,
            "question": question,
            "revision": revision,
            "proposal_excerpt": proposal_text[:1200],
            "attacks": [
                {k: a[k] for k in
                 ("role", "verdict", "severity", "summary", "strongest_point")}
                for a in attacks
            ],
            "judge": {
                "per_attack": verdict.get("per_attack", {}),
                "decision": decision,
                "tradeoffs": tradeoffs,
                "rationale": verdict.get("rationale", ""),
            },
            "outcome": decision,
            "cost_usd": round(round_cost, 4),
        }
        hist.record_round(round_rec)
        _log(f"  JUDGE → {decision}  ({verdict.get('rationale','')[:160]})")

        if decision == "FAILED":
            revision += 1
            if revision > max_revisions:
                _log(f"  ✋ max revisions ({max_revisions}) hit; leaving "
                     f"[{question}] unsettled.")
                hist.save()
                return round_rec
            # feed the landed attacks back to the proposer
            prior_attacks = [
                a for a in attacks
                if verdict.get("per_attack", {}).get(a["role"]) == "genuine-flaw"
            ] or attacks
            round_no = len(hist.state["rounds"]) + 1
            continue

        # 4. SURVIVED or TRADEOFF -> settle, then reconcile against the design
        decision_rec = {
            "question": question,
            "headline": headline,
            "proposal_text": proposal_text,
            "tradeoffs": tradeoffs,
            "round": round_no,
            "revisions": revision,
            "verdict": decision,
        }
        incomps = _reconcile(decision_rec, hist, runs_dir, round_no)
        for ic in incomps:
            hist.record_incompatibility(ic)
            _log(f"  🔗 INCOMPATIBILITY [{ic['severity']}]: "
                 f"{ic['between'][0]} ✕ {ic['between'][1]} "
                 f"→ {ic['property_at_risk']}")
        hist.settle(decision_rec)
        hist.save()
        return round_rec
