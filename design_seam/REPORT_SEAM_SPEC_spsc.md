# Seam phase report — SEAM_SPEC_spsc.md

*3/3 consecutive cold survivals (ran the full streak; no FAILED). SEAM HELD with recorded cost — gauntlet 1 logged the decidable P5 `T: Send` boundary bound as a price; gauntlets 2–3 clean.*

## Gauntlet 1 (round 2) — ⚖️ TRADEOFF

- **horn classifier:** SEAM-COSTS [none]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Sound, decidable, concentrated — but P5 rides an explicit T:Send bound at the boundary.
- **rationale:** No safe UB path: affinity discharges P4, !Sync blocks sharing/Arc, P1–P3 trusted in-unsafe (§5); push/pop stay proof-free and once-at-split. Per the soundness attacker's decisive fact, the affine token does NOT carry P5: separating handles onto two threads requires a handle to be Send, gated on an explicit `T: Send` bound at the re-import boundary — decidable, so no horn drops, but a price to record.

## Gauntlet 2 (round 3) — ✅ SURVIVED

- **horn classifier:** SEAM-HOLDS [none]
- **obstruction sits at:** none (stayed decidable: True)
- **headline:** Sound, decidable, concentrated — the rtrb/RustBelt SPSC seam holds all three horns.
- **rationale:** The only load-bearing weakening (P4 'ever'→'one-at-a-time') is sound: every safe cross-thread transfer of an affine, !Sync, !Clone handle carries happens-before, giving SPSC's total order. P1–P3 are trusted, never compiled, so nothing new is decided; T:Send is ordinary trait resolution. Zero-copy absence is unclaimed scope, not a horn break; joint ownership defuses leakpocalypse.

## Gauntlet 3 (round 4) — ✅ SURVIVED

- **horn classifier:** SEAM-HOLDS [none]
- **obstruction sits at:** none (stayed decidable: True)
- **headline:** Affine !Sync Send handles soundly re-import SPSC; all three horns hold.
- **rationale:** No laundering path: the Send move-edge supplies the publication happens-before transitively while affinity forbids the only two-store race (attacker-soundness). The boundary verifies nothing (trust declaration), so only floor-decidable affinity+T:Send resolution runs (attacker-implementation). Seam crossed once at split. The lone landed finding—zero-copy inexpressibility—is proof-free additive, threatening no horn.
