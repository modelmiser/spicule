# Seam phase report — SEAM_SPEC_spin_mutex.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 43) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Missing guard Sync bound launders a data race; horn 1 drops, tally rises to 10.
- **rationale:** attacker-soundness (judge-confirmed fatal): the discharge proves single-guard-liveness but not guard-exclusion-across-threads. Omitting std's stricter unsafe impl<T: Sync> Sync for the guard lets SpinGuard: Sync auto-derive at merely T: Send, so &g is Send; with T=Cell<i32> (Send+!Sync, past CF1's Rc filter) two threads share one live guard and race &Cell — UB from safe code. Decidability (horn 2) held, but soundness broke.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let m = SpinMutex::new(core::cell::Cell::new(0i32)); let g = m.lock(); thread::scope(|s| { s.spawn(|| g.set(1)); s.spawn(|| { g.get(); }); }); — SpinGuard has no manual Sync impl, so SpinGuard: Sync ⟺ &SpinMutex<Cell<i32>>: Sync ⟺ SpinMutex<Cell<i32>>: Sync ⟺ Cell<i32>: Send (true); thus &g: Send, both closures are Send, and the two threads deref &g → &Cell<i32> and race a non-atomic write against a read = data race UB, with zero unsafe in the consumer and the lock never contended.
  ```
