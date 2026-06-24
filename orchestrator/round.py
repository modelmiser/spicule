"""One adversarial design question, driven to a verdict (Section 5 of the seed).

Termination (faithful to the seed): a decision is settled only when it survives
`survivals` CONSECUTIVE cold gauntlets without revision — each gauntlet re-runs
five fresh cold attackers + judge against the SAME proposal, so robustness is
tested by independent sampling, not a single lucky pass. Any FAILED gauntlet
feeds the landed attacks back to the PROPOSER and resets the streak.

On revision exhaustion the SYNTHESIZER distills the convergent failure into a
recorded property-incompatibility — the Section-8 payload — so a question that
never settles still produces the most valuable kind of output.

Coldness alarm: a gauntlet where every attacker reports SURVIVES/none is flagged
(per the seed, attackers that never find anything mean the prompts aren't cold).
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
        candidates = re.findall(r"(\{(?:[^{}]|\{[^{}]*\})*\})", text, re.DOTALL)
    for c in reversed(candidates):
        try:
            return json.loads(c)
        except json.JSONDecodeError:
            continue
    return None


def _run_attackers(proposal_text: str, runs_dir: Path, tag: str,
                   prior_art_web: bool) -> list[dict]:
    def one(role: str) -> dict:
        web = prior_art_web and role == "attacker-prior-art"
        r = call(
            roles.build_attacker_prompt(proposal_text, web=web),
            roles.ROLE_SYSTEM[role],
            label=f"{tag} {role}{' +web' if web else ''}",
            allow_web=web,
        )
        parsed = extract_json_block(r.text) or {}
        (runs_dir / f"{tag}_{role}.md").write_text(r.text)
        return {
            "role": role, "text": r.text,
            "verdict": parsed.get("verdict", "?"),
            "severity": parsed.get("severity", "?"),
            "summary": parsed.get("summary", ""),
            "strongest_point": parsed.get("strongest_point", ""),
            "cost_usd": r.cost_usd,
        }

    with ThreadPoolExecutor(max_workers=len(roles.ATTACKER_ROLES)) as ex:
        return list(ex.map(one, roles.ATTACKER_ROLES))


def _gauntlet(hist: Historian, question: str, proposal_text: str, revision: int,
              runs_dir: Path, prior_art_web: bool) -> dict:
    """One attack+judge pass on a fixed proposal; records and returns the round."""
    round_no = len(hist.state["rounds"]) + 1
    tag = f"r{round_no}"
    attacks = _run_attackers(proposal_text, runs_dir, tag, prior_art_web)

    if all(a["verdict"] == "SURVIVES" and a["severity"] in ("none", "?")
           for a in attacks):
        note = ("all 5 attackers reported SURVIVES/none — prompts may not be cold "
                "enough, or a correlated blind spot")
        hist.flag_blind_spot(note, round_no)
        _log(f"  ⚠️  COLDNESS ALARM: {note}")

    judge = call(
        roles.build_judge_prompt(question, proposal_text, attacks),
        roles.ROLE_SYSTEM["judge"], label=f"{tag} judge",
    )
    verdict = extract_json_block(judge.text) or {}
    (runs_dir / f"{tag}_judge.md").write_text(judge.text)
    decision = (verdict.get("decision") or "FAILED").upper()
    cost = judge.cost_usd + sum(a["cost_usd"] for a in attacks)

    rec = {
        "round": round_no, "question": question, "revision": revision,
        "proposal_excerpt": proposal_text[:1200],
        "attacks": [{k: a[k] for k in
                     ("role", "verdict", "severity", "summary", "strongest_point")}
                    for a in attacks],
        "judge": {"per_attack": verdict.get("per_attack", {}), "decision": decision,
                  "tradeoffs": verdict.get("tradeoffs", []) or [],
                  "headline": verdict.get("headline", ""),
                  "rationale": verdict.get("rationale", "")},
        "outcome": decision,
        "cost_usd": round(cost, 4),
    }
    hist.record_round(rec)
    _log(f"  JUDGE → {decision}  ({verdict.get('rationale','')[:150]})")
    return rec


def _reconcile(new_decision: dict, hist: Historian, runs_dir: Path) -> list[dict]:
    found = []
    for prior in hist.settled():
        if prior["question"] == new_decision["question"]:
            continue
        r = call(roles.build_reconciler_prompt(prior, new_decision),
                 roles.ROLE_SYSTEM["reconciler"],
                 label=f"reconcile [{prior['question'][:30]}]")
        parsed = extract_json_block(r.text) or {}
        slug = re.sub(r"\W+", "-", prior["question"])[:30]
        (runs_dir / f"reconcile_{slug}.md").write_text(r.text)
        if parsed.get("incompatible") and parsed.get("severity") in ("hard", "soft"):
            found.append({
                "between": [prior["question"], new_decision["question"]],
                "severity": parsed.get("severity"),
                "property_at_risk": parsed.get("property_at_risk", "?"),
                "description": parsed.get("description", ""),
                "source": "reconcile",
            })
    return found


def _synthesize_failure(hist: Historian, question: str, runs_dir: Path) -> dict | None:
    """Distill a revision-exhausted question into a payload incompatibility."""
    failed = [r for r in hist.state["rounds"]
              if r["question"] == question and r["outcome"] == "FAILED"]
    if not failed:
        return None
    r = call(roles.build_synthesizer_prompt(question, failed),
             roles.ROLE_SYSTEM["synthesizer"], label="synthesizer")
    slug = re.sub(r"\W+", "-", question)[:30]
    (runs_dir / f"synth_{slug}.md").write_text(r.text)
    parsed = extract_json_block(r.text) or {}
    if not parsed.get("converged"):
        _log("  synthesizer: failures did not converge — nothing recorded.")
        return None
    props = parsed.get("properties_in_tension", []) or ["?", "?"]
    incomp = {
        "between": [props[0], props[1] if len(props) > 1 else "(see description)"],
        "severity": parsed.get("severity", "hard"),
        "property_at_risk": ", ".join(props),
        "description": parsed.get("headline", "") + " — " + parsed.get("description", ""),
        "source": "revision-exhaustion",
        "from_question": question,
        "properties_in_tension": props,
    }
    hist.record_incompatibility(incomp)
    _log(f"  🔗 SYNTHESIZED INCOMPATIBILITY [{incomp['severity']}]: "
         f"{parsed.get('headline','')}")
    return incomp


def synthesize_failed_question(hist: Historian, question: str,
                               runs_dir: Path) -> dict | None:
    """Backfill: run the synthesizer over an already-failed question, then save."""
    runs_dir.mkdir(parents=True, exist_ok=True)
    incomp = _synthesize_failure(hist, question, runs_dir)
    hist.save()
    return incomp


def run_round(hist: Historian, question: str, runs_dir: Path,
              max_revisions: int = 3, survivals: int = 3,
              prior_art_web: bool = False, proposer_shots: int = 1) -> dict:
    """Drive one question: propose → (survive `survivals` cold gauntlets) → settle,
    revising on any FAILED up to `max_revisions`; synthesize on exhaustion.

    proposer_shots > 1 runs that many self-strengthening passes before each gauntlet
    — a stronger proposer, to test whether one-shot weakness (not structure) caused
    a question to fail."""
    runs_dir.mkdir(parents=True, exist_ok=True)
    prior_attacks: list[dict] | None = None
    revision = hist.revisions_for(question)
    last_rec: dict = {}

    while True:
        _log(f"\n=== [{question[:48]}…] revision {revision} ===")
        prop = call(
            roles.build_proposer_prompt(question, hist.settled(), prior_attacks),
            roles.ROLE_SYSTEM["proposer"],
            label=f"proposer (rev{revision})",
        )
        proposal_text = prop.text
        for shot in range(1, proposer_shots):
            _log(f"  proposer self-strengthen pass {shot}/{proposer_shots-1}")
            prop = call(
                roles.build_proposer_strengthen_prompt(question, proposal_text),
                roles.ROLE_SYSTEM["proposer"],
                label=f"proposer strengthen {shot} (rev{revision})",
            )
            proposal_text = prop.text
        (runs_dir / f"proposal_rev{revision}_r{len(hist.state['rounds'])+1}.md"
         ).write_text(proposal_text)

        # Survive `survivals` consecutive cold gauntlets on this same proposal.
        streak = 0
        carried_tradeoffs: list[str] = []
        failed_rec: dict | None = None
        last_pass: dict = {}
        while streak < survivals:
            rec = _gauntlet(hist, question, proposal_text, revision, runs_dir,
                            prior_art_web)
            last_rec = rec
            if rec["outcome"] == "FAILED":
                failed_rec = rec
                break
            streak += 1
            last_pass = rec
            carried_tradeoffs += rec["judge"].get("tradeoffs", [])
            _log(f"  survival {streak}/{survivals}")

        if failed_rec is not None:
            revision += 1
            if revision > max_revisions:
                _log(f"  ✋ max revisions ({max_revisions}) hit; "
                     f"[{question[:40]}…] unsettled. Synthesizing failure…")
                _synthesize_failure(hist, question, runs_dir)
                hist.save()
                return last_rec
            pa = failed_rec["judge"].get("per_attack", {})
            prior_attacks = [a for a in failed_rec["attacks"]
                             if pa.get(a["role"]) == "genuine-flaw"] \
                or failed_rec["attacks"]
            continue

        # Survived `survivals` in a row → settle, then reconcile against the design.
        decision_rec = {
            "question": question,
            "headline": last_pass["judge"].get("headline", ""),
            "proposal_text": proposal_text,
            "tradeoffs": sorted(set(carried_tradeoffs)),
            "round": last_pass["round"], "revisions": revision,
            "verdict": last_pass["outcome"], "survivals": survivals,
        }
        for ic in _reconcile(decision_rec, hist, runs_dir):
            hist.record_incompatibility(ic)
            _log(f"  🔗 INCOMPATIBILITY [{ic['severity']}]: "
                 f"{ic['between'][0][:34]} ✕ {ic['between'][1][:34]} "
                 f"→ {ic['property_at_risk']}")
        hist.settle(decision_rec)
        _log(f"  ✅ SETTLED after {survivals} consecutive survivals.")
        hist.save()
        return last_pass
