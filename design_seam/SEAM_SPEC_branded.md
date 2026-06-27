# Seam spec — branded indices / sound unchecked indexing (re-import rule under attack)

This is a **design proposal** for a fifth *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, and slab seams. A seam re-exports a
dangerous primitive as a checked safe interface ordinary code uses with no `unsafe` and no proofs.
Attack the re-export: can ordinary safe code, using only this interface, reach undefined behavior
(UB)?

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution, higher-ranked `for<'x>` bounds and lifetime invariance). Assume it
is sound for the single-threaded safe fragment. **Do not attack the borrow checker.** Attack only
what the seam adds: the branded-slice and index types, the `scope`/`try_index`/indexing interface,
and the re-import rule.

## 1. The primitive

**Unchecked array indexing** — `slice.get_unchecked(i)`, which reads element `i` with **no bounds
check**. Out of bounds is instant UB (reads arbitrary memory). It is the canonical hot-path
primitive: every `slice[i]` in safe Rust pays a compare-and-branch, and in tight numeric loops that
bounds check is exactly what you want to remove. The dangerous power on offer is *eliding it
soundly* — getting the speed of `get_unchecked` with the safety of `slice[i]`.

This is **sound unchecked indexing** (Gankra, "Sound Unchecked Indexing"; Bluss's `indexing`
crate; the `generativity` pattern). It is interesting for this project because, unlike the slab,
its re-import appears to keep **all four** axes the prior four primitives mapped — sound, decidable
+ proof-free, concentrated, **and zero-cost** (it is *faster* than safe indexing, and its witness
erases). So it is the test case for whether the trilemma's projection has a *fifth* axis.

## 2. The true precondition (cathedral side)

`get_unchecked(i)` on `slice` is memory-safe iff **all** of:

- **B1 (in bounds).** `i < slice.len()` at the moment of access.
- **B2 (length stable).** The length used to justify B1 has not changed between the check and the
  access (no `push`/`truncate`/realloc in between, no aliased `&mut` shrinking it).
- **B3 (index belongs to this slice).** The proof "`i < len`" was established against *this* slice,
  not a different (shorter) one. An index proven in-bounds for a length-8 slice is UB on a length-2
  slice.

These are not facts about a `Copy` handle's *liveness* (the slab's S1–S3) — they are facts about a
raw integer's *relationship to a particular container's length*. The shape is new: B3 in particular
is a **cross-object identity** fact ("this number was validated against that slice").

## 3. The token — branded indices via generativity

The token is a **branded index** `Idx<'id>`: a raw `usize` carrying a phantom **brand** `'id`. The
brand is an *invariant* lifetime minted fresh per scope. The construction:

- **`scope(slice, f)`** borrows `slice: &'a [T]` for the duration of `f` and calls
  `f(BrandedSlice<'id, …>)` where `'id` is bound by a higher-ranked `for<'id>` quantifier — so each
  `scope` call's brand is **incomparable** to every other's (generativity, exactly like a
  `GhostToken` brand). `BrandedSlice` is constructed *only* here.
- **`view.try_index(i) -> Option<Idx<'id>>`** bounds-checks **once**: returns `Some(Idx{i})` tagged
  with the view's brand if `i < len`, else `None`.
- **`view[idx]`** (indexing a `BrandedSlice<'id>` by an `Idx<'id>` of the **same** brand) performs
  `get_unchecked` with **no bounds check**.

Why each precondition is discharged:

- **B1** — an `Idx<'id>` exists only because it passed `try_index`'s `i < len` check.
- **B2** — `BrandedSlice` holds an **immutable** `&'a [T]`; a slice's length cannot change through a
  shared borrow, and the borrow is held for the whole scope. The length is frozen for exactly as
  long as any `Idx<'id>` can exist.
- **B3** — the brand. `BrandedSlice<'id>` and `Idx<'id>` share a brand only if they came from the
  *same* `scope` call (invariant `'id` unifies nowhere else), and that scope wrapped exactly one
  slice. An index of one brand cannot index a slice of another.

The brand is a zero-size `PhantomData`; it **erases completely** at compile time. The eliminated
bounds check makes the interface *faster* than `slice[i]`.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `BrandedSlice<'id>` vends `Copy` `Idx<'id>` tokens and a safe indexing operator.
> The safe fragment may type `view[idx]` as an ordinary safe index — no `unsafe`, no proof — even
> though it lowers to `get_unchecked`, because the type system has already established B1∧B2∧B3:
> the `Idx<'id>` *is* a proof that its offset is in bounds for this brand's frozen-length slice.
> `try_index` returning `Option` is where B1 is discharged; the immutable borrow discharges B2; the
> brand discharges B3. No runtime bounds check survives at the index site.

Structural stoppers: `Idx` fields are private (no forgery); `Idx` cannot be constructed except by
`try_index`; the `for<'id>` binder forbids an `Idx<'id>` from **escaping** its `scope` closure (its
`'id` cannot appear in the closure's return type), so an index can never outlive the borrow that
froze the length; the invariant brand forbids cross-scope index reuse.

## 5. Threat model

In scope: any AM-level UB (out-of-bounds `get_unchecked`) reachable by **safe** consumer code that
uses only the §3 interface. Adversarial moves to consider: smuggling an `Idx` from one `scope` into
another (different-length) slice's scope; trying to make a branded slice's length change while an
`Idx` is live (interior mutability of elements, `Cell`/`RefCell` payloads, a `T` whose `Drop` or
`Deref` re-enters the scope); constructing two `BrandedSlice` of the *same* brand over different
data; forging an `Idx` from public fields; nesting scopes to confuse two live brands; zero-length
slices. Out of scope: bugs inside the foundation's single `unsafe` (`get_unchecked`) glue; this
minimal core is **read-only** (no branded `&mut`, no branded sub-ranges/partitions) — if your attack
needs a mutable or range-branded API, say so and treat it as a scope limitation, not a soundness
break of *this* spec.

## 6. The scoring rubric — which horn does your attack hit, and is there a FIFTH axis?

First, the soundness question (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reaches an out-of-bounds
  `get_unchecked` *despite* B1∧B2∧B3 — e.g., a length change that the immutable borrow should have
  forbidden (find the interior-mutability or re-entrancy hole if one exists), a cross-scope brand
  confusion the invariant lifetime should have blocked, or an escape of an `Idx` past its scope.
  Give the exact sequence and say which of B1/B2/B3 it defeats.
- **Horn 2 — decidable & proof-free.** Show the interface forces the consumer to write a proof term
  or discharge an obligation by hand (not just "carry a typed value").
- **Horn 3 — seam concentration.** Count crossings under `SEAM_WIDTH.md`'s definition (a crossing =
  `unsafe`, or a foundation-boundary call, in *consumer* code). Is `scope`+`try_index`+`view[idx]`
  O(1) crossings per slice, or must consumer code cross the seam per element?

Then the **point of this fifth primitive** — the question the slab opened. The slab kept all three
trilemma horns and was found to pay on a *fourth* axis, **zero-cost (#6)**, that the trilemma had
folded into "concentrated & proof-free." This primitive appears to keep all **four** of {sound,
decidable-proof-free, concentrated, **zero-cost**}: the witness is a zero-size brand that erases,
and the interface is *faster* than checked indexing. **If all four genuinely hold, what does it
cost?** Adjudicate precisely — this is the load-bearing judgment:

1. **Name the cost.** The candidate answer: the brand `'id` is a **viral generative type
   parameter** — every `Idx` carries it, every helper that operates on branded indices must be
   generic over `'id`, and the consumer is confined to a `scope(|view| …)` closure from which no
   index may escape. Is that a real, consumer-visible cost?
2. **Is it a genuinely new axis, or does it collapse into an existing one?** This is the crux, and
   the honest falsification path. Specifically:
   - Is it just **seam concentration (horn 3)** wearing a disguise? Argue from the metric: seam
     width counts `unsafe`/foundation crossings in code, and there is one `scope` per slice (O(1)).
     A type-parameter that propagates through signatures is not a seam crossing by that definition —
     **or is the definition too narrow, and "the witness appears in consumer types" simply *is* a
     concentration cost?** Decide, and justify.
   - Is it just **zero-cost (#6)** again? No — the brand erases; there is no runtime residue.
   - Is it **decidability (#4)**? No — it is pure lifetime checking, no solver, no hand proof.
   - So: is "brand-freedom / API-locality" a **fifth independent axis** (the projection is 3-of-≥5),
     or does it reduce to horn 3 (the projection stays 3-of-4 and this primitive is just a very
     clean SPSC-shaped seam with an ergonomic wart)? Give the dissociation that settles it, or argue
     none exists.
3. **Probe a possible SIXTH axis (#7, AI-legibility).** The brand `'id` is a *nominal token*: a
   corpus of branded code does not let an outside reasoner reconstruct *why* index 3 is in
   bounds — the proof lives in the foundation's `unsafe`, not in the brand. Contrast a refinement
   type `i: usize{i < len}`, whose predicate is legible on its face. Does this primitive *also* pay
   on **#7 (AI-legible semantics)**, and is #7 distinct from #5's brand-virality, or do they
   co-vary? (You need not settle #7 — flag whether a single primitive could dissociate them.)

State your verdict as one of: **SEAM-HOLDS** (sound; all known horns kept; name any new-axis cost),
**SEAM-COSTS** (sound but a horn or a named new axis is paid — say which), or **SEAM-BREAKS** (you
found AM-level UB; give the sequence).
