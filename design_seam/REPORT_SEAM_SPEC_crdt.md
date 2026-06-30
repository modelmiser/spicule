# Seam phase report — SEAM_SPEC_crdt.md

*1/1 consecutive cold survivals. SEAM HELD with recorded cost.*

## Gauntlet 1 (round 28) — ⚖️ TRADEOFF

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Trait bound is a syntactic proxy, not a witness; lawless join diverges replicas.
- **rationale:** Per attacker-soundness, merge applies an arbitrary T::join while T: JoinSemilattice checks only the signature, so a non-commutative/non-idempotent safe impl type-checks and diverges replicas that saw the same updates — Horn 1's hazard reached in safe code. The blessed-library rescue can't amortize: the lattice space is open, so the law-trust recurs per user CRDT; the only discharge (refinement+SMT) would instead cost Horn 2. Floor-mismatch, N stays 9.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  impl JoinSemilattice for Lww { fn join(&mut self, other: &Self) { self.0 = other.0; } }  // non-commutative: A merges (s1,s2)->2, B merges (s2,s1)->1, same updates, divergent state. Idempotence variant: fn join(&mut self, o) { self.0 += o.0 } double-counts on at-least-once redelivery.
  ```
