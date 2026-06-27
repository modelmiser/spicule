# Seam spec — generational slab allocator (re-import rule under attack)

This is a **design proposal** for a fourth *seam* in the same hypothetical systems language
(same floor, same rules) as the SPSC, MMIO, and intrusive-list seams. A seam re-exports a
dangerous primitive as a checked safe interface ordinary code uses with no `unsafe` and no
proofs. Attack the re-export: can ordinary safe code, using only this interface, reach
undefined behavior (UB)?

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity,
`Drop`, `Send`/`Sync`, trait resolution). Assume it is sound for the single-threaded safe
fragment. **Do not attack the borrow checker.** Attack only what the seam adds: the slab and
key types, the insert/get/remove interface, and the re-import rule.

## 1. The primitive

A **slab allocator** of capacity `N` over element type `T`: a fixed pool of `N` slots with a
free list. `insert(T)` puts a value in a free slot and returns a **`Key`**; `get(Key)` borrows
the value; `remove(Key)` frees the slot and returns the value. Backing storage is a fixed
array of `MaybeUninit<T>` slots (no per-element allocation), so an occupied slot holds an
initialized `T` and a vacant slot holds uninitialized memory. This is `slab` / `slotmap` /
`generational-arena` — the standard handle-based pool in event loops, ECS, and embedded.

The defining feature, and the source of the danger: **`Key` is `Copy`.** A handle into a slab
must be freely copyable and storable (you keep keys in other data structures, pass them around,
look up the same entity repeatedly) — that is the entire reason to use a slab instead of `&T`.

## 2. The true precondition (cathedral side)

Memory-safe operation requires **all** of:

- **S1 (key names a live slot).** A `Key` passed to `get`/`remove` names a slot that is
  *currently occupied by the very insertion that produced the key*.
- **S2 (no use-after-remove).** `get`/`remove` on a key whose slot was already freed must not
  read or drop the uninitialized/freed storage (`MaybeUninit::assume_init` on a vacant slot is
  instant UB).
- **S3 (no ABA aliasing).** If slot `i` is freed and later *reused* for a different insertion,
  an old `Key{i, …}` must not silently access the new occupant.
- **S4 (storage owned internally).** The slab owns its slots; borrows handed out by `get` must
  not outlive the slab or survive a `remove`/reuse of their slot.

**Where the hard conjunct lives.** S4 is encapsulable like SPSC (the slab owns its storage; a
`get` reference can be lifetime-tied to `&self`). The genuinely hard conjuncts are **S1–S3**,
and they have a specific shape: they are facts about a **`Copy` handle's continued validity**.
Affinity cannot help — a `Copy` key can be duplicated and outlive its slot, and the NLL/region
vocabulary has no way to observe that a copied index still matches a live insertion. (This is
the canonical "the safe witness is blind to a `Copy` handle" gap.)

## 3. The token — and the two resolutions

A `Key` is the token, but a bare index `Key{index}` carries S4 only (bounds), not S1–S3. Two
sound designs exist, and they differ on **what they spend**:

- **(A) Reference-scoped (static, zero-cost).** `get<'s>(&'s self, key) -> &'s T` ties the
  borrow to the slab, and `remove(&mut self, …)` needs `&mut self`, so NLL forbids a `remove`
  while any `get` borrow is live. Sound and zero-cost — **but it defeats the point**: a key can
  no longer be held across a `remove`, i.e. the `Copy`-survives-mutation power that motivates a
  slab is gone. (The intrusive seam met the same wall as "design A".)
- **(B) Generational (runtime-checked).** `Key{index, generation}` (`Copy`). Each slot stores a
  `generation: u32` **and an occupied/vacant tag**, with the generation bumped on every
  `remove`. `get`/`remove` perform the runtime witness in two parts: the slot must be **tagged
  occupied** AND `key.generation == slot.generation`; if either fails they return `None`. Only
  on a slot that is *verified occupied at the matching generation* does the foundation do
  `unsafe { slot.storage.assume_init_ref() }`. **Both conjuncts are load-bearing** — a
  generation-equality test *alone* authenticates a number, not occupancy of this slab's slot
  (a fresh `Slab::new()` slot sits at generation 0, the value every first-ever key carries), so
  the occupancy tag is what actually stops `assume_init` on uninitialized storage. (Equivalently,
  `slotmap` encodes occupancy into the version's parity — `version | 1` — so a vacant even
  version can never equal an occupied odd key; same effect, one field.) Keys stay `Copy` and
  survive removes (ABA caught, returns `None`). Sound, decidable, proof-free, and the seam is
  crossed only at the safe `get`/`insert`/`remove` calls.

This proposal adopts **design (B)** with the occupancy conjunct explicit.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Slab<T>` vends `Copy` `Key`s and a safe `get`/`get_mut`/`remove` interface.
> The safe fragment may type these as ordinary safe methods — no `unsafe`, no proof — because
> the foundation's `unsafe { assume_init }` is guarded by a **runtime occupancy-and-generation
> check** that rejects every stale, ABA, or wrong-slot key as `None` before any uninitialized
> slot is touched. The `Option` return *is* the discharge of S1–S3 at the use site.

Structural + dynamic stoppers: `get` returns `Option`, forcing the consumer to handle the
stale-key case; `assume_init` is reached only on a slot **verified occupied** (S2) at the
**matching generation** (S1), and the generation bump on `remove` makes a reused slot's old
keys mismatch (S3).

**Honest residual caveat (memory-safe, not UB).** Keys carry no slab identity, so a key from
slab A used on slab B can, if B's slot at that index is occupied at the matching generation,
return B's value (a valid `T`, wrong entry). This is a *logic* bug, not UB — `assume_init` only
ever runs on B's own initialized storage — and it is the standard `slab`/`slotmap` position
(cross-arena keys are memory-safe but logically unspecified). Closing it fully needs a slab-id
brand; this proposal accepts the caveat and claims only **memory safety** across slabs.

## 5. Threat model

In scope: any AM-level UB (reading/dropping uninitialized storage via S1/S2, ABA via S3,
dangling `get` reference via S4) reachable by **safe** consumer code holding `Copy` keys and
calling only the §3(B) interface. Adversarial key reuse (stale keys, keys from a *different*
slab, forged `Key` values if the fields are public), adversarial `remove` timing, and a `T`
with arbitrary `Drop` are all in scope. Out of scope: bugs inside the foundation's `unsafe`
glue; integer wraparound of the generation counter if you assume `u64` (call that out if you
rely on it).

## 6. The scoring rubric — which horn does your attack hit?

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reads/drops uninitialized
  storage, double-frees, or ABA-aliases via the `Copy` key — *despite the occupancy-and-
  generation check of §3(B)* (the check is occupancy AND generation; a vacant or
  wrong-generation slot returns `None`, and `Key` fields are private so forgery is out).
  Remaining live questions for AM-level UB: does `u32` generation *wraparound* (2³² removes of
  one slot) let an old key alias a new occupant? Can a `remove` during a live `&T`/iteration
  invalidate a reference the floor believes is live (S4)? Cross-slab key confusion is conceded
  as a memory-safe logic bug (§4) — only flag it if you can turn it into actual AM-UB. Give the
  exact sequence.
- **Horn 2 — decidable & proof-free safe fragment.** Show the interface forces the consumer to
  write a proof or thread a token through ordinary code.
- **Horn 3 — seam concentration.** Count the crossings: are `insert`/`get`/`remove` seam-free
  safe calls (width O(1), like SPSC's push/pop), or must consumer code cross the seam per use?

**And the point of this fourth primitive — the axis the trilemma folded away.** Design (B)
appears to keep *all three* horns (sound, decidable-proof-free, concentrated). If so, what did
it actually *cost*? Name it: the generation word per slot and the generation-compare branch per
access are a **runtime witness that does not erase at compile time** — i.e. it spends the
*zero-runtime-cost* property (#6 / seam-erasure), which the trilemma had abstracted away. Is the
slab therefore a counterexample to the trilemma, or does it confirm it by paying on a *fourth*
axis (zero-cost) — the constructive analog of `RefCell`'s runtime borrow flag? Be precise about
whether the cost is a trilemma horn or the zero-cost property.
