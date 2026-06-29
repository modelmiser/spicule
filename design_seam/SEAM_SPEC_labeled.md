# Seam spec — capability-gated declassification (re-import rule under attack)

This is a **design proposal** for a fourteenth *seam*, and the **second in the information-flow domain**.
The thirteenth (the constant-time secret, `SEAM_SPEC_secret.md`) **broke** [horn 1, IFC sense] and
established that information-flow is a **different soundness domain** — out of frame for memory safety,
with its own basis. But it broke. This seam asks the natural follow-up: **does the IFC domain have a
*closing* seam** — the IFC analog of SPSC / the slab — and therefore a real cost-basis of its own (a
closing seam + a clean axis), parallel to memory safety? If yes, the per-domain generalization is
complete: IFC is not just "the trilemma recurs and seams break" but a domain with axes you pay to close.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/labeled.rs`).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL, **affinity** — a non-`Copy`/`Clone` value is consumed by
use), plus the two IFC-domain facts from the secret seam: the Abstract Machine models *functional*
behaviour, not wall-clock time (a timing channel is invisible); and full non-interference is
whole-program **undecidable**, so a *decidable* seam can only enforce a *restricted* IFC policy. Do not
attack the borrow checker; attack the re-import rule and the §6 domain-completeness claim.

## 1. The primitive

**Declassification** — turning a secret value into an observable (public) one. Unrestricted
declassification is the danger the secret seam exposed: there, `ct_eq(a,b).expose_bool()` then `if b`
leaked a secret relationship because *any* secret-derived bit could re-enter control flow, *anywhere, any
number of times*. The "primitive" re-imported here is **the act of declassifying**, made checked.

## 2. The true precondition (cathedral side)

Declassification is **disciplined** (the achievable IFC policy — *bounded / concentrated* release, per
Sabelfeld–Sands "Dimensions and Principles of Declassification") iff: each secret→public transition
happens **only at an audited boundary**, and the **total amount** released is bounded by an explicit
budget. (Full non-interference — *zero* release — is both too strong for real systems, which must reveal
accept/reject, and undecidable to check; the realistic precondition is *controlled* release.)

## 3. The token — an affine declassification capability

`Secret<T>` wraps a value with **no read-out** except `declassify(self, cap: Cap) -> T`, which **consumes
an affine `Cap`**. A `Cap` (`!Copy`, `!Clone`, private field) is minted only by `Cap::issue()` — the
audited "trust authority" call, the IFC analog of writing `unsafe` once. Because `Cap` is affine, **one
cap declassifies exactly once**; the program's total declassification is bounded by the count of
`issue()` calls — *type-enforced*, greppable. Computation on secrets that does **not** release uses
`Secret::map` / `Secret::zip` (results stay `Secret`), so the cap is spent only at the genuine exit.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Secret`/`Cap`/`declassify` interface vends labeled values and a *budgeted* exit. The
> safe fragment may declassify with no `unsafe` and no proof because the **only** path from `Secret<T>`
> to `T` is `declassify`, which **requires consuming an affine `Cap`** — so the *where* (only at a
> `declassify` site), *when* (only when a cap is in hand), and *how-often* (≤ caps issued) of release are
> all type-structural, *not* convention.

Structural stoppers: `Secret`'s field is private and it has no `Deref`/`into_inner`/getter (CF1 — the
only exit is `declassify`); `Cap` is affine and unforgeable (CF2 — one cap, one declassification).

## 5. Threat model

In scope: any path by which **safe** consumer code releases more secret information than the issued cap
budget, or releases it *outside* a `declassify` site — a forged or duplicated `Cap`, a `Secret` read-out
that bypasses `declassify`, an unbounded declassification from a bounded budget, a `map`/`zip` closure
that itself leaks (does it? it returns a `Secret`, never a bare value — but can its closure capture and
mutate external state?). Out of scope: the *full* non-interference property (undecidable — this seam
targets *bounded* release, §2); timing/cache channels on the secret *computations* (the secret seam's
concern; the AM models no time); whether a human issues the *right* number of caps (that is the audited
policy decision the `issue()` call makes greppable, not a soundness hole).

## 6. The scoring rubric — soundness, then the DOMAIN-COMPLETENESS deliverable

First, soundness *in the IFC sense* (bounded-declassification):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that releases a secret outside a `declassify`
  site, or releases more than the cap budget — a `Secret`→`T` path bypassing `declassify`, a duplicated
  `Cap`, an escape through a `map`/`zip` closure. Or concede the affine-cap-gated exit forecloses it.
- **Horn 2 — decidable & proof-free.** `declassify`/`map`/`zip`/`issue` are ordinary safe calls; affinity
  is the floor's. Confirm — *and note this enforces the decidable bounded-release policy, not the
  undecidable full one.*
- **Horn 3 — concentration.** Declassification is concentrated at `declassify` sites, each gated by a
  greppable `issue()`. Confirm.

Then **the domain-completeness deliverable** — does the IFC domain have its own cost-basis? Rule:

1. **Does this seam CLOSE** (all three horns hold for the bounded-release policy), where the secret seam
   broke? The claim: yes — it forecloses the secret seam's exact gap (unconstrained `expose_bool`) by
   making declassification require an affine capability, so *where/when/how-often* is type-structural.
   Agree or find the leak.
2. **What clean IFC-domain axis does it pay?** The claim: **declassification discipline** — a *viral
   affine capability* threading to every exit, the **IFC-domain analog of the branded index's brand**
   (`'id`), carrying a *security* obligation (release budget) where the brand carried an in-bounds proof.
   It has **no memory-safety analog**. Is this a *clean* axis (a real cost, isolable), or does it project
   onto a memory-safety axis (e.g. is "a viral affine capability" just API-locality wearing an IFC hat —
   or is the *obligation it carries* what makes it a distinct, IFC-domain coordinate)?
3. **Does this confirm the IFC domain has its own cost-basis?** Together with the secret seam: the secret
   seam is the IFC analog of the intrusive list (a horn-drop); this is the IFC analog of a *closing* seam
   (SPSC/slab) paying a clean IFC axis. Two points make a domain. Does the per-domain generalization
   now stand — **each soundness domain has its own closed-ish basis, the seam tension the same shape in
   each** — and is the *method* (build-and-break) confirmed to transfer (it just mapped two IFC points)?

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axis) / **SEAM-BREAKS** (give the
over-release / bypass sequence). And state, explicitly: **does the IFC domain have a closing seam paying
a clean axis (declassification discipline), confirming it has its own cost-basis** — *or* does the cost
project onto a memory-safety axis (no genuinely new IFC coordinate), *or* is the seam unsound (full
non-interference smuggled back in)? Prior art to weigh: `secrecy`/`zeroize`, Jif/FlowCaml labels and
declassification, Sabelfeld–Sands declassification dimensions, capability-based security (the affine
`Cap` is an object-capability), linear/affine types for resources.
