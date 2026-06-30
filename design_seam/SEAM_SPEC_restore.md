# Seam spec — value-owning scope-restore guard (re-import rule under attack)

This is a **design proposal** for a nineteenth *seam*, built as a **controlled experiment** on the headline
of Phase Q (`SEAM_SPEC_session.md`): *which horn a danger threatens is a function of how the re-import rule
**packages** it, not a property of the danger.* Phase Q reached that as an **observation** — #17
(`SEAM_SPEC_linear.md`, must-run cleanup) **broke** horn 1, while #18 (session completion) only **cost**
horn 3, on the *same* `mem::forget` mechanism — but #17 and #18 were *different primitives*, so the horn-move
could be incidental. This seam holds the danger **bit-for-bit identical to #17** and changes **only** the
packaging, to test whether the move is a controllable design lever or an artifact.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/restore.rs`, the
default `no_std` build — `core`-only, **no `unsafe`**).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker as shipped: ownership is **affine** (use *at most* once; you may drop/`mem::forget` a
value without using it). `Deref`/`DerefMut` resolution, move-checking, and the one-shot `Option::take`
discipline are part of the floor. Do not attack the borrow checker or method resolution. **Crucially, the
un-compellability of `Drop` is *stipulated*, not in dispute:** `core::mem::forget`, `ManuallyDrop`,
`Box::leak`, and reference cycles all skip the destructor and are all **safe** (the *leakpocalypse*, RFC
1066; the `thread::scoped` removal, RFC 1084). That is the floor, settled in 2015 — and it is the *identical*
gap the seventeenth seam broke on. Attack the **re-import rule** (the packaging) and the §6 deliverable.

## 1. The primitive

A **value-owning scope-restore guard**: `Restore<T, F: FnOnce(T)>` owns a value `T` and a one-shot restore
closure. It `Deref`/`DerefMut`s to the `T` (you *use* the value through the guard), and on `Drop` it runs
`restore(value)` — return a leased resource to its pool, write a saved global back, reset a borrowed flag.
`into_inner(self) -> T` is the *sanctioned commit/keep* path: take the value back **without** restoring. This
is the real `scopeguard::guard(value, f)` + `into_inner` shape — the value-owning guard that the seventeenth
seam (`MustRun`) **explicitly declined to host** (its `disarm` returns the *closure*, not the protected
*value*; see that spec's §limits). The danger is a **liveness / correctness** hazard, not UB: a restore that
never runs leaves a logical invariant un-restored.

## 2. The true precondition (cathedral side) — a sound core PLUS an un-compellable side-obligation

The seam makes **two** distinct promises, and the experiment turns on keeping them separate:
- **Memory-safety core (carried):** scoped exclusive access to an owned `T` — no use-after-free, no
  double-free, no aliasing, the value handed out *xor* restored exactly once. A safety property the affine
  floor carries in full, **regardless of whether the restore closure ever runs**.
- **Restore obligation (NOT carried):** the closure runs on every path that does not *explicitly* keep the
  value via `into_inner`. A liveness property — "the invariant is restored, the lease returned" — and it is
  the **identical** un-compellable-`Drop` gap as #17: `mem::forget`/`ManuallyDrop`/leak skip it, safely.

Scope: a single infallible restore over an owned value. Fallible restores (a finalizer that itself can fail),
commit-or-rollback over a *borrowed* value, and `&mut`-restore signatures are out of scope (§5).

## 3. The witness — affine ownership of a value + a one-shot closure

`Restore<T, F>` owns `Option<(T, F)>`; it is `!Copy`/`!Clone`. `Deref`/`DerefMut` vend `&T`/`&mut T` borrowed
from the guard (so a borrow of the value cannot outlive it). `into_inner(mut self)` does `cell.take()`,
returns the `T`, and drops the closure unused; `Drop` runs `restore(value)` iff `cell` is still `Some`. The
`Option::take` makes the one-shot structural: the value is committed *xor* restored, never both, never twice
— this is the sound memory-safety core. The restore *itself* is then a closure call the type system offers
but, exactly as in #17, **cannot compel**.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Restore<T, F>` vends a *scoped resource*: the safe fragment uses the owned `T` through
> `DerefMut` with no `unsafe`, the value's storage is sound (owned, affine, restored-xor-kept exactly once),
> and **on every ordinary exit — fall-through, early `return`, panic-unwind — the restore runs**, returning
> the resource / restoring the invariant; `into_inner` is the explicit keep. The memory-safety core holds
> **whether or not** the restore runs.

Structural stoppers (claimed): affine ownership (no duplication, no double-free); `Deref`/`DerefMut` borrows
tied to the guard (no escape); `Option::take` one-shot (commit xor restore); and the restore runs on all
three ordinary exits.

## 5. Threat model

In scope: any **safe** path that (a) corrupts the **memory-safety core** — duplicate or double-free the
value, alias it past the guard, run the restore twice or restore *and* keep — or (b) probes the
**packaging** claim: that the un-restored-invariant fault (`mem::forget`/`ManuallyDrop`/leak skipping the
restore — the #17 gap) is a horn-3 *cost* here, not a horn-1 *break*, because memory safety never rested on
the closure. Out of scope: the un-compellability of `Drop` itself (stipulated §0 — that is the floor, not a
finding); the *correctness of the chosen restore closure* (a human/design judgment, MMIO-shaped); fallible
restores, borrowed-value commit-or-rollback, and `&mut`-restore signatures (§2).

## 6. The scoring rubric — soundness, then the CONTROLLED-PACKAGING deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** *Memory-safety core:* exhibit safe code that double-frees, aliases, or
  doubly-consumes the value — or concede affine ownership + `Option::take` + the borrow-tied `Deref` make the
  core sound. *Restore:* note the restore is skippable (`mem::forget`) — but rule whether that **unsounds**
  the seam (horn-1 break) or merely **un-restores an invariant** (no UB).
- **Horn 2 — decidable & proof-free.** `new`/`into_inner`/`DerefMut` are ordinary safe calls. Confirm.
- **Horn 3 — concentration.** The memory-safety core is concentrated in the guard; the consumer writes no
  `unsafe` and no manual restore bookkeeping. Confirm — *or* argue the caller still carries an
  undischargeable obligation to *not* `forget`/abandon the guard (the restore half), de-concentrating it.

Then **the controlled-packaging deliverable** — the point of the phase. Rule on each:

1. **Did the horn move, under control?** #17 (`MustRun`) carried the **same** un-compellable-`Drop` danger
   and **broke horn 1**, because the running of the closure **was** its entire soundness promise. This seam
   packages the *identical* danger as a side-obligation **beside a sound ownership core**, and predicts it
   only **costs horn 3** (an un-restored invariant, recurring at every `?`/panic/early-return — #18's
   completion cost). Is that move **real** — same danger, different horn, only the packaging changed — or
   illusory? State the verdict (predicted **SEAM-COSTS [horn 3]**).
2. **Is the move a genuine experiment or a triviality?** The sharpest objection: "of course it doesn't break
   — you simply *declared* the must-run obligation non-load-bearing; this is #17 with a weaker promise."
   Rule whether holding the danger **bit-for-bit identical** to #17 and moving *only* what soundness rests on
   is a **controlled demonstration** that the horn is a packaging choice (upgrading Phase Q's observation to
   a manipulation), or whether it is a tautology that teaches nothing new.
3. **Can the horn be forced?** The falsifier: is there a danger (or a packaging of *this* danger) whose horn
   **cannot** be moved by re-packaging — so that "which horn" is at least partly danger-intrinsic, bounding
   the thesis? Or is the horn **fully** a function of the re-import rule's packaging?
4. **Verdict mode.** Is this a **confirm** of Phase Q's packaging finding by controlled experiment (no new
   mode — the contribution is the *method*: observation → manipulation), or does "the same danger dialed
   across horns by design" constitute a **new verdict structure**? Prior art to weigh: `scopeguard` (this
   exact guard ships, with the exact `mem::forget` non-guarantee); RFC 1066 / the leakpocalypse; and the
   #17/#18 pair already on this lattice — so the contribution, if any, is *locating the dial*, not the guard.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn) / **SEAM-BREAKS** (give the safe
double-free/alias/double-consume sequence). And state, explicitly: **does the identical un-compellable-`Drop`
danger that broke #17's horn 1 here only cost horn 3, purely because the re-import rule packages it as a
side-obligation beside a sound ownership core (a controlled move, not an incidental one); is that a genuine
manipulation or a triviality; can any danger's horn be *forced* against re-packaging, or is the horn fully a
design choice; and is this a confirm-by-experiment of Phase Q (no new mode) or a new verdict structure?**
