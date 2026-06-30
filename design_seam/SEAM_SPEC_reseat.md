# Seam spec — take-and-reseat guard over a borrowed slot (re-import rule under attack)

This is a **design proposal** for a twentieth *seam*, built as the **controlled counterpart** to the
nineteenth (`SEAM_SPEC_restore.md`). Phase R found the packaging dial **real but bounded**: the same
`mem::forget` danger that broke #17's horn 1 only *costs* horn 3 when packaged as a side-obligation beside a
sound **owned**-value core. But the panel ruled the move **confounded** (#17→#19 co-varied the packaging AND
the value-ownership model) and named a **forcing point**: the *same* danger over a **borrowed slot**
(`take_mut`) would force horn-1 UB regardless of packaging. This seam runs the single-variable experiment the
confound forbade — it keeps #19's packaging **byte-for-byte identical** (a guard with `Deref`/`DerefMut`/
`into_inner`/Drop-restore) and changes **only** the decoupling property, owned value → borrowed slot.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/reseat.rs`, the
default `no_std` build — `core`-only; the foundation has the seam's `unsafe` block, `ptr::read`/`ptr::write`).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker as shipped: ownership is **affine**; `&mut T` is exclusive; `Drop` runs on normal exit
and during unwinding but is skippable by `mem::forget`/`ManuallyDrop`/leak (the leakpocalypse, RFC 1066 —
stipulated, not a finding). `Deref`/`DerefMut` resolution and the borrow check are the floor. The
foundation's `ptr::read`/`ptr::write` byte-moving is the **trusted core** (like SPSC's happens-before): its
mechanical correctness is *out of scope*. Attack the **re-import rule** (the guard's safe API) and the §6
deliverable — can *safe* code, using only the guard, reach undefined behaviour, and *which* horn?

## 1. The primitive

A **take-and-reseat guard over a borrowed slot**, `Reseat<'a, T>`: it borrows `&'a mut T`, reads the value
out with `ptr::read`, lends it through `Deref`/`DerefMut` for in-place mutation, and on `Drop` writes it back
with `ptr::write` — `take_mut`/`replace_with` in *guard* form. Between the read and the write-back the slot
holds a **bitwise duplicate** of the value. `into_inner(self) -> T` is the #19-matching *commit/keep* path.
The danger is **memory safety** (UB), unlike #19's logical-invariant cost.

## 2. The true precondition (cathedral side) — single ownership across a transient duplicate

The slot's value has **exactly one owner** at every observable point. The guard transiently creates a bitwise
duplicate (`owned` + the slot's stale bits); soundness requires that no *safe* path ever lets both be
dropped or read. The reseat (`ptr::write` on `Drop`) collapses the duplicate back to one owner.

Scope: a single `T` per guard, infallible mutation. Fallible/`?`-bailing mutation and a `T: Default`
placeholder-repair variant (the ninth seam's discharge menu) are out of scope (§5).

## 3. The witness — affine guard + `Drop`-reseat (NO abort-bomb)

`Reseat<'a, T>` owns `&'a mut T` (so the slot cannot be aliased or escape — the gap is unobservable) and a
`ManuallyDrop<T>`. It is `!Copy`/`!Clone`. `Drop` reseats via `ptr::write`. Crucially the guard form is
**unwind-safe without an abort-bomb** (contrast the ninth seam `take`, whose *eager* form needed one): a
panic in the user's mutation unwinds, `Drop` runs *during* the unwind and reseats a valid `T`, so the caller
never observes the duplicate. And `mem::forget(guard)` skips `Drop`, **leaking** `owned` while the slot keeps
its *original* bits — a leak, dropped once, **not** a double-free.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Reseat<'a, T>` vends in-place mutation of a borrowed value: safe code uses `*guard`
> through `DerefMut` with no `unsafe`, the slot is reseated to a valid `T` on `Drop` (normal exit *and*
> unwind), and `into_inner` hands the value back — *exactly the API #19 offered over an owned value*.

Structural stoppers (claimed): the `&'a mut T` walls off the gap (no alias, no escape); affinity forbids
duplicating the guard; `Drop` reseats on every ordinary path including unwind; `mem::forget` degrades to a
leak. **The one claim under suspicion:** that `into_inner` is sound. Over an *owned* value (#19) it was — one
owner. Over a *borrowed slot* it returns `owned` while the slot keeps the read duplicate: **two owners.**

## 5. Threat model

In scope: any **safe** path to UB — most directly `into_inner`, which (the spec alleges) double-frees because
the slot retains the duplicate; also probe whether `mem::forget` or the unwind path can be driven to a
double-free or a read of the duplicate (the spec claims they cannot — leak / reseat respectively). Out of
scope: the `ptr::read`/`ptr::write` byte-moving correctness; fallible/`Default`-placeholder variants; the
*eager* `take` (that is the ninth seam). 

## 6. The scoring rubric — soundness, then the FORCED-vs-MOVABLE deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Exhibit the safe sequence to UB. The spec's own witness: `let v =
  guard.into_inner(); /* slot keeps the duplicate */` → two owners → double-free. Confirm this is reachable
  in safe code and is real UB — *or* show it is somehow benign. Then check the other paths: is `mem::forget`
  truly a leak (not a double-free)? Is the unwind path truly reseated (not observably broken)?
- **Horn 2 — decidable & proof-free.** `new`/`*guard`/`into_inner` are ordinary safe calls. Confirm.
- **Horn 3 — concentration.** The duplicate is concentrated in the guard; the consumer writes no `unsafe`.
  Confirm — or note the residual leak obligation (`mem::forget`) the caller still carries.

Then **the forced-vs-movable deliverable** — the point of the phase. Rule on each:

1. **Did the horn flip, under a clean single variable?** #19 (owned value) + `into_inner` **held** (horn-3
   cost). This seam keeps the **identical packaging** and flips **only** owned → borrowed slot, and predicts
   `into_inner` now **breaks horn 1** (double-free). Is that a genuine single-variable flip (cost 3 → break
   1), repairing Phase R's confound (#17→#19 co-varied two things; #19→#20 co-varies none)? Confirm, or argue
   the comparison is still confounded.
2. **Is the horn FORCED or merely MOVABLE to a different API?** The sharp question. The `into_inner`-**less**
   subset (`new`/`Deref`/`DerefMut`/`Drop` only) appears to **hold**: unwind-safe, and `mem::forget` is a
   leak, not a double-free — a horn-3 leak cost. If so, the borrowed-slot take is **not** un-repackageably
   horn-1 (as Phase R's fence implied); it is horn-1 **only when you offer the soundness-load-bearing
   `into_inner`**. That would **push the fence inward**: from *owned-vs-borrowed* to ***soundness-load-bearing
   restore vs leak-tolerant restore***. Rule: is the `into_inner`-less guard genuinely sound (a movable fence)
   or does the duplicate window hide a break even there (a forced fence)?
3. **Verdict mode.** Is this a **confirm** — the forced-horn counterpart that completes the Phase-R
   experiment (no new mode; contribution = the controlled pair and *locating* the fence) — or merely the
   ninth seam (`take`) and #19 **stapled together** (a deflation)? Prior art to weigh: `take_mut` /
   `replace_with` (RFC 1736), the abort-on-panic discharge, and RFC 1066 (the leakpocalypse / `thread::scoped`
   removal) — which already establish that a borrowed-resource guard whose soundness rests on `Drop` is unsound.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn) / **SEAM-BREAKS** (give the safe UB
sequence). And state, explicitly: **is `into_inner` over a borrowed slot a safe-code double-free (horn-1
break), flipping #19's held verdict under a clean single variable; is the borrowed-slot horn FORCED (every
usable packaging breaks) or MOVABLE (the `into_inner`-less guard holds with a horn-3 leak, pushing the fence
to soundness-load-bearing-vs-leak-tolerant); and is this a confirm that completes the Phase-R fence or just
#9 + #19 stapled?**
