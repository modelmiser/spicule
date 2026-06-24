"""Adversarial language-design orchestrator.

Implements Section 5/6 of LANG_DESIGN_SEED.md using the Claude Code CLI
(`claude -p`) as the role-instance engine instead of the raw Anthropic API.
Each role call is a cold, stateless subprocess: full context is reconstructed
from the HISTORIAN every round, so ATTACKER instances receive only the proposal.
"""
