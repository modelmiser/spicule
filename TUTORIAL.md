# newlang — a tutorial on the impossible language, and the PL theory you need to read it

**Audience.** A mid-level career developer: you write production code, you've used Rust or
C++ or Go, you know what a segfault is and roughly why it happens, but you have not formally
studied programming-language theory. By the end you will understand (a) the actual PL theory
this project rests on, from the ground up; (b) the state of the art in "safe systems
languages"; and (c) what newlang discovered and *why it matters*.

**What newlang is.** Not a language. It is a research project that asked "what would the
perfect systems language look like?", listed seven properties such a language should have,
and then **proved — adversarially — that the seven cannot coexist**, mapped exactly where the
wall is, and then **built thirty-two real things that live right up against the wall (eleven that close, plus twenty-one probes — ending by showing the closed basis is *per soundness domain*, that which seams close is *graded* by the *floor-precondition match* (full for memory safety, none for information flow, partial then none for liveness), that *upgrading the floor is non-monotone* — the granularity gap is a ladder — that a floor can be the *wrong structural logic* (affine vs linear) no upgrade fixes, and finally that the *same* danger is a soundness *break* or a mere concentration *cost* depending on how the re-import rule **packages** it — a dial that proves **real but bounded**: a controlled probe moved one identical danger across horns, yet the move only works where soundness can be *decoupled* from the un-compellable closure, and a clean single-variable follow-up confirmed the fence is *forced* — the same packaging over a borrowed slot breaks — and that a *fourth domain* (replicated convergence) need not even be one: the candidate **deflated** into the generic unchecked-typeclass-law gap, projecting onto an existing basis — and that a *second* fourth-domain try (real-time/WCET) **failed** too, its op-count an *unsound* proxy for cycles (a cost monad), so two candidates in a row open no new domain and the three confirmed domains look **special** — and finally that a *real binary session* **deepened** liveness: it held (removing #18's single-handle limit and confirming the grading transfers), but its "real session" framing deflated to *per-session-only* progress, global deadlock-freedom needing *multiparty* typing — and finally that **delegation** (channel-over-channel), built to add that very feature, *held* but had its deliverable **rejected**, opening **progress/deadlock-freedom** as a third liveness coordinate I had mislabeled as a leak — and finally that a *multiparty* session, assembled from those binary edges, made #23's asserted interleaved deadlock a *compiled* artifact (`ring()` type-checks a source-less three-party cycle the per-edge floor cannot range over), *held* (sound) yet had its deliverable **rejected a third time** when my "global progress lives only in a generative whole-protocol projection" claim proved a *false dichotomy* — per-channel priority typing (Kobayashi, Dardha–Gay) rejects the ring with no global type, restoring a three-rung taxonomy and the weaker truth that global progress is a *whole-program* (not per-call, not necessarily generative) obligation — and finally that a *priority-typed* session (Kobayashi/Dardha–Gay, the rung the multiparty rejection named), built to *confirm* that weaker thesis, instead **broke on soundness** — the public `role()` constructor lets three identically-priority-typed relays compile into a deadlocking ring (edge-priority consistency is a *trusted-constructor invariant*, not a type fact), the **eighth broken seam**, confirming the thesis by *breaking* not costing and unifying progress's lock face (ordered_locks #15/#16) and session face under one *wrong-granularity* break — and finally that the *salvage* of #26, sealing the public constructor whose hole *broke* #26 and vending only trusted builders that thread strictly-increasing `Nat`s per edge, **held soundly** (the deadlocking ring now *unconstructible* — no public mint, and a monotone chain cannot close) yet had its deliverable **rejected a fourth time**, because "deadlock-freedom by construction" is really "**schedulability** by construction": affine Rust proves a deadlock-free *schedule exists* (the static `Nat` witness) but never *forces* the consumer to follow it, so a run-all/in-order/no-drop **progress residue** stays on the caller — splitting the progress coordinate into *topology* (no cycle, which the trusted constructor carries — the #26 fix) and *completion/execution* (always carried by the consumer — the #17/#18/#23 residue at multiparty scale)) — and finally that **choice** (`offer`/`select`, the branching session connective — external `&`, internal `⊕`), the last unprobed *core* session feature, *held* (sound — worst case a starved/leaked but memory-safe session) yet had its deliverable **rejected a fifth time**, because §4's "handle every label" guarantee is **vacuous**: Rust exhaustiveness gives *scrutinee coverage*, not *run-to-End* — a safe `match offer() { …, _ => {} }` type-checks with no `unsafe` and *drops* the branch's `Chan<R>`, starving it, so "handle-all" has no content beyond the conceded completion residue and **collapses into it** (now fed by *two* safe paths — `mem::forget` and the dropped wildcard arm); branch-safety is **two-legged** (label–continuation correlation + branch-set duality), not the four §4 listed (order is the generic typestate guarantee #18, "handle-all" the false leg); and the deflation lands on **binary `Either` + exhaustiveness**, not n-ary *named* sums (labeled choice needs positional nesting that drops the names, or `offer!`/`choose!` macros — exactly what the `session_types` crate reached for), so choice opens **no new coordinate** — the branching connective opens no new ground, and the last core session feature deflates onto binary `Either` — and finally that **must-complete** (a `Session<P>` affine handle with a `Drop` *completion bomb*, cf. `take` #9 — drop a `live` mid-protocol session and it aborts, so the floor *appears* to compel run-to-`End`; built, after the session connectives were exhausted, to attack the *completion residue* itself, the leftover every session seam #17–#28 carries), *held* (sound — `mem::forget` skips the bomb and starves the peer, but no UB) yet had its deliverable **rejected a sixth time**, and the *most instructive* rejection of the arc because the gauntlet **REVERSED** my finding rather than relabelled it: my pre-registered "completion is an *irreducible* linear gap — the session face of `linear` #17's must-run break" is **refuted** by a *scoped-closure / CPS* combinator — `with_session(|s| { …; s.finish() })`, the `std::thread::scope` analogue — that **compels completion modulo divergence with no `unsafe`**, moving the obligation to the re-import/construction boundary so the consumer never holds a forgettable *free* handle ("return `End` or diverge"); the sting is the *sixth refutation from my own cited lineage* — I cited `thread::scoped`'s **removal** (RFC 1084) as proof completion can't be forced and ignored its actual **fix**, `thread::scope` (stable 1.63) — so the #17–#28 completion residue was the **price of the FREE-HANDLE API**, never an inherent gap (every prior session seam vended a forgettable `Chan`, so every one carried it), and sessions ≠ #17's cleanup: a *delimited* session **can** be scoped to force completion, an *open-ended* cleanup cannot; completion is therefore **not irreducible but *concentrable*** at a horn-3 API-locality cost (surrender the free movable handle; run the session inside a scope), the verdict landing **SEAM-COSTS** not a break (FAILED broken-seam tally **stays eight**, refuting my own 8→9 prediction), and inverting my usual error — across the prior rejections I overclaimed concentration *total*, here I overclaimed it *irreducible*, when the truth is a cost *in between*)** to show what
you *can* do. The deliverables are an impossibility result, a theorem (the *Seam Trilemma*),
a Lean proof, and a small Rust crate of worked examples.

**How to read this.** Parts 1–5 are the PL bootstrap — the prerequisites, taught from
scratch, with the state of the art woven in. Parts 6–10 are the project itself. Part 11 is
the transferable meta-lessons. Each part stands alone; skip what you know. Code references
point at real files in this repo (`seam/`, `proof/`, `design_seam/`).

> **Catch-up note (2026-06-28) — what's new since the first version.** This tutorial originally
> ended at the seven-property impossibility result and its first four worked seams. It has since
> been carried through the project's *constructive* arc, and the story got materially deeper. Two
> additions, both in Parts 9–11:
> 1. **The trilemma turned out to be a *projection* — of a *closed* object.** Its three horns are a
>    low-dimensional shadow of an object with **nine clean axes** — each an *already-named* PL concern
>    (zero-cost abstraction, generativity, refinement-legibility, the `core`/`alloc` split,
>    exception-safety, leak-freedom) — plus an *inseparable category* and two *outside-the-axes* slots.
>    Eight more primitives (5–12) probed it one at a time, so the result is a **unification** of known
>    axes under one question — *what does the safe witness cost?* — a question with **finitely many
>    answers**. That honest deflation, and the *closure* of the basis, is the spine of the new material.
> 2. **The method has *nine* verdict modes** — the fifth closes the basis; the last four step outside the
>    memory-safety frame (the ninth: the floor is the wrong *structural logic*). The probes past seven exercise all of them. (8) **rejected** — confounded. (9) **confirmed**
>    — *unwind-safety*. (10) **confirmed and ruled distinct** — *leak-freedom*. (11) **real but
>    structurally inseparable** — *move-stability*. (12, a validity read) **projects entirely onto the
>    basis** — *the cost-basis is closed*. (13, a constant-time secret) stepped **outside memory safety**:
>    information-flow is a *different soundness domain* with its own basis — closure is **per domain**.
>    (14, a capability-gated declassification) probed whether the IFC domain has a *closing* seam and found
>    it does **not** — both IFC seams break, because **Rust's affine floor carries none of the IFC
>    precondition** (it bounds resource *uses*, not information *bits*). And (15, a type-level lock ordering)
>    probed the **positive** half in a *third* domain — **liveness/deadlock-freedom** — and **broke** too,
>    but *partially*: a re-entrant root mints a second token lineage so safe code types a lock-order
>    *inversion* (wrong **granularity** — per-token, not per-thread), and even patched, the dynamic
>    fragment escapes the static floor (wrong **coverage**). And (16, a *runtime* lock registry) **upgraded
>    the floor** to fix that — a thread-local held-set of runtime ranks — and **broke too**, showing the
>    upgrade is **non-monotone**: it fixed *coverage* (dynamic orderings) but **granularity recurred one
>    rung finer** (the held-set is keyed on the *thread*, but async *tasks* are the control unit, so
>    work-stealing migration launders an undetected inversion). So the deepest result: **which seams close
>    is *graded* by the floor-domain match** — **full** (memory safety / affine floor), **none** (information
>    flow / affine floor), **partial** (liveness — and *no* floor lifts it to full, because *granularity is
>    a containment-scope ladder*, token < thread < task, and each ambient floor reaches one rung). And (17, a
>    must-run cleanup guard) went *sideways* — same liveness domain, a *different precondition* ("this
>    finalizer must run") — and **broke** in a *new mode*: a clean **none** (a safe `mem::forget` skips the
>    destructor — the affine floor cannot compel a *linear* obligation), the **wrong structural logic** (no
>    floor *upgrade* fixes it — every sound decidable fix needs linearity Rust rejected), which also sharpens
>    the unit of the match from *domain* to *precondition*. And finally (18, a *session-typed channel*) took
>    that *same* `mem::forget` linearity gap and — by **packaging** it differently — made it merely **cost**
>    instead of break: it is the **first** out-of-memory-safety probe to **hold** (SEAM-COSTS), because its
>    precondition *decomposes* and the floor matches it **component-wise** — the *order-safety* half **closes**
>    via affine typestate (no `root`-hole, unlike #15), while the *completion* half is the #17 gap again but
>    now only an undischargeable **horn-3 concentration cost** (it recurs at every `?`/early-exit) rather than
>    a soundness break, because completion is *one component beside a sound safety core* rather than the whole
>    promise. So **which horn a danger threatens is set by how the rule packages it** — the same gap unsounds
>    #17 and merely de-concentrates #18. (Honest deflations the panel forced: a single-handle typestate
>    *walk*, not a binary session; "usable" only for infallible straight-line protocols; and the split is
>    documented folklore — the contribution is *locating* it.) And finally (19, a *value-owning scope-restore
> guard* — the real `scopeguard`) turned that packaging observation into a **controlled experiment**: it
> carries #17's *identical* `mem::forget` danger but wraps it in #18's structure (a sound ownership core
> beside which the restore is a side-obligation), and **held** (SEAM-COSTS, horn 3) — the same gap that
> *broke* #17 only *costs* here. But the panel **bounded** the thesis, all accepted: the move is
> **confounded** (to package the danger as a side-obligation, #19 also adds the value-ownership core #17
> lacked, so the value-ownership model co-varies — a better-controlled probe than the incidental #17-vs-#18
> pair, not a clean single variable); and the horn is **forceable** (the *same* skippable-`Drop` danger in a
> *borrowed-slot/`take_mut`* packaging forces horn-1 UB, and `Pin`'s drop guarantee and the `thread::scope`
> restructure exhibit un-repackageable horns), so *which* horn depends on **where safety rests** — partly
> danger-intrinsic. The refined finding: the packaging dial is **real but bounded** — which horn a danger
> threatens is a design choice *within a structural envelope* set by whether soundness can be **decoupled**
> from the un-compellable closure (owned ⇒ costs; borrowed-across-unwind ⇒ forces). Not novel: `JoinGuard`
> broke horn-1 and `scopeguard` cost horn-3 already in 2015 (RFC 1066); the contribution is the method and
> *locating* the dial. And finally (20, the *borrowed-slot* counterpart — `take_mut` as a guard) ran the
> clean single-variable experiment #19's confound forbade: keep #19's packaging byte-identical, change *only*
> owned → borrowed, and it **broke** (SEAM-BREAKS, horn 1, the sixth break) — `into_inner`, sound for #19's
> owned value, is a safe-code double-free over a borrowed slot. The flip cost-3 → break-1 is now attributable
> to the decoupling property *alone*, and the panel rejected the "maybe-movable" hedge (the sound subset is
> degenerate, dominated by `&mut T`), so the fence is **forced**, at the owned-vs-borrowed line — the
> packaging trilogy closes with one law: the horn is movable **iff** the value-ownership model lets soundness
> decouple from the un-compellable closure. And finally (21, a *state-based CRDT merge*) aimed at a **fourth
> domain** — replicated convergence — and the candidate **deflated**: the supply-side break is real (a safe
> lawless `join` diverges replicas) but it is the *generic unchecked-typeclass-law* gap (`Ord`/`Eq`/`Monoid`/
> semilattice — signatures checked, laws not), projecting onto `restore`'s application-correctness basis, so
> convergence opens no new domain and the map stays at three soundness domains plus application-correctness.
> The Seam Trilemma is *domain-general*, the
>    cost-basis *per-domain*, the match *graded* along *granularity* and *coverage* and keyed to the
>    *precondition*, *upgrading the floor trades one gap for a finer one*, *a floor can be the wrong
>    structural logic*, and *the horn a danger threatens is a packaging choice*. A discovery procedure that
>    only confirms is measuring nothing; this one rejected, confirmed, distinguished, recognised the
>    inseparable, certified closure, recognised a new domain, diagnosed a floor-domain mismatch, graded a
>    partial match, showed the grade cannot be bought up by upgrading the floor, *and named a
>    wrong-structural-logic floor* — all **nine** verdict modes (the eighteenth through twenty-second seams add no new mode; they
>    *re-package* an existing one, turning #17's break into a cost — #19 then maps the *limit* of that re-packaging (the horn moves only where soundness decouples from the danger), and #20 *forces* the horn at that limit, confirming the fence under a clean single variable.)
>
> If you read the earlier version, the genuinely new content is **Primitives 5–21** (Part 9), the
> "un-bundlings" + the rejection + the inseparable ruling + the closed-basis capstone + the
> domain-generalization + the floor-domain refinement + the **graded** (full/partial/none) match + the
> **non-monotone floor-upgrade** (granularity is a ladder) + the **wrong-structural-logic** mode (affine vs
> linear) + the **packaging** finding (the same gap breaks #17 but only *costs* #18 — the first
> out-of-memory-safety probe to **hold**), and **meta-lessons 6–10** (Part 11). Everything in Parts 1–8 is
> unchanged prerequisite material.

> **One-paragraph spoiler.** A type system that is *decidable* (the compiler always halts with
> yes/no and never asks you to write a proof) fundamentally cannot also be *expressive enough*
> to derive all of memory safety, concurrency, and zero-cost abstraction from first principles.
> Something has to give. Real languages give up different things, and the *seam* — the place
> where a checked, decidable world hands off to a trusted, proven one — is unavoidable. The
> interesting engineering is in *shaping* that seam, and the shape is dictated by one question:
> can the dangerous fact be carried by the cheap checker, checked at runtime, assumed as an
> axiom, or none of the above?

---

# Part 1 — The dream: seven properties of an ideal systems language

Start with the wish list. A "systems language" is one you'd write an operating system, a
database, a trading engine, or device firmware in: it runs on bare metal, with no garbage
collector pausing it at random, and it must never corrupt memory. The dream language has all
seven of these at once:

1. **Executable specification** — the spec *is* the type system *is* the compiler. No gap
   between "what it should do" and "what the checker enforces."
2. **Unified proof/systems layer** — no FFI seam between "proof land" (where you prove things)
   and "systems land" (where you poke hardware). One language.
3. **Ownership without GC, derived from the proof layer** — memory safety with no garbage
   collector, and *not* via a hardcoded special-case checker but derived from the proof rules.
4. **Decidable type checking** — the compiler always terminates with a yes/no answer, and the
   common case needs *no user-written proofs*.
5. **Bare-metal targeting** — no runtime, no reference-count spikes, no GC pauses; usable for
   hard-real-time.
6. **Zero-cost abstractions** — the proof/safety machinery *erases* at compile time; nothing
   survives to slow the program down.
7. **AI-legible semantics** — a model trained on the corpus can reconstruct the spec (a modern
   nice-to-have).

Each one ships *somewhere* today. The conjecture newlang tested is whether they ship
*together*. They do not, and property #4 — decidability — is the antagonist in every conflict.
To see why, you need to know what a type checker actually is.

---

# Part 2 — PL bootstrap I: what a type checker actually is

### Types are a proof system

A type is a *claim*, and type checking is *checking a proof of that claim*. When you write
`fn f(x: u32) -> bool`, the type `u32 -> bool` is the proposition "given a 32-bit unsigned
integer, this produces a boolean," and the compiler verifies the body is a valid proof of it.
This is the **Curry–Howard correspondence**: types are propositions, programs are proofs. You
don't need the deep version; you need the consequence — *a type checker is a theorem prover*,
and everything that is true of theorem provers is true of type checkers.

### Soundness: the property that makes types worth anything

A type system is **sound** if "well-typed" implies "well-behaved." Formally, soundness is two
lemmas: **progress** (a well-typed program never gets stuck — it can always take the next step
or is a finished value) and **preservation** (taking a step keeps the program well-typed).
Together: a well-typed program never reaches an undefined state. Soundness is what lets you
trust `u32` actually holds a number and not a dangling pointer. An *unsound* type system is
decoration. (newlang's calibration controls test exactly this — see Part 6.)

### Decidability: the checker must halt

Here is the constraint that ends up being load-bearing for the whole project.

A *decidable* type checker is one that always terminates with yes or no. You want this — a
compiler that might loop forever on valid code is unusable. For simple type systems (Java, Go,
basic Rust), decidability is easy. But the dream's property #1 ("the spec *is* the type
system") pushes toward **dependent types** — types that contain *values*, like
`Array<T, n>` where `n` is a runtime-ish number, or `Vec<T> where len > 0`. Dependent types
let the type system express *arbitrary* correctness properties. And there's the catch:

> Once types can express arbitrary propositions, type checking *is* theorem proving in
> arbitrary logic, which is **undecidable** (you can encode the halting problem, or Hilbert's
> 10th problem on Diophantine equations, into type equality). So the checker either (a)
> restricts to a *decidable fragment* — losing expressiveness — or (b) asks the human to supply
> the proof — losing "no user-written proofs."

This is the first fork in the road, and property #4 sits right on it. "Decidable AND no user
proofs" means you must find a fragment of logic that is simultaneously **expressive enough** to
state your safety properties and **decidable enough** to check automatically. That fragment is
small. The whole negative result is the discovery that it is *too* small to also carry
ownership, concurrency, and erasure.

**Worked intuition.** Refinement types like `{x: u32 | x < 100}` are decidable when the
refinements live in a theory an SMT solver decides quickly (linear integer arithmetic,
"QF-LIA"). Push to nonlinear arithmetic (`x * y < z`) and you're at the edge of decidability;
push to "this pointer aliases no other live pointer" and you're in undecidable separation
logic. The dream wants the *power* of the second with the *automation* of the first.

State of the art for "decidable + cheap": **Liquid Haskell** and **Flux** (refinement types
for Rust) carve out a decidable refinement fragment and lean on an SMT solver. They are
excellent and they are *bounded* — they work precisely where the refinement logic stays
decidable, and they punt (to the human, or to "can't express it") elsewhere. That boundary is
the trilemma in the wild.

---

# Part 3 — PL bootstrap II: memory safety without a garbage collector

### What "undefined behavior" actually is

A systems language without a GC must guarantee memory safety *statically*, because there is no
runtime to catch you. The failure modes (collectively **undefined behavior**, UB):

- **Use-after-free (UAF):** you free memory, then read or write through a pointer to it. The
  memory may now hold something else. This is not "a crash" — it is the compiler's assumptions
  being violated, which means *anything* can happen, including silent corruption and remote
  code execution. UB is "the abstract machine has no defined meaning for this," and optimizers
  are allowed to assume it never happens.
- **Double-free:** freeing the same allocation twice — corrupts the allocator.
- **Data race:** two threads touch the same memory, one writing, with no synchronization.
  Undefined at the language level even if it "looks fine."

A GC prevents UAF by never freeing memory anyone can still reach. The dream forbids a GC
(property #5), so it needs another mechanism. That mechanism is **ownership**.

### Ownership, moves, and affine types

The core idea: every value has exactly one **owner**; when the owner goes away, the value is
freed; you cannot use a value after you've given it away. This is a **substructural type
system**. Normal logic lets you use a fact as many times as you like; substructural logics
restrict that:

- **Linear** types must be used *exactly once* (can't duplicate, can't drop).
- **Affine** types must be used *at most once* (can't duplicate, *can* drop). **Rust's
  ownership is affine.**
- **Relevant** types must be used *at least once*.

The key property affinity buys you: **no duplication.** If a value cannot be copied, then there
is only ever one of it, and "free it when the owner is done" is safe — there's no second owner
to be surprised. Remember this; it is exactly the trick that makes one of newlang's four
primitives (the SPSC queue) work, and exactly the trick that *fails* for another (the slab,
whose keys must be copyable).

### Borrowing, lifetimes, and the borrow checker

Owning-and-moving alone is too restrictive — you'd have to give a value away to let anyone look
at it. So you add **borrowing**: temporary references that don't transfer ownership. Two kinds:

- `&T` — a **shared** (immutable) borrow. Many allowed at once.
- `&mut T` — a **unique** (mutable) borrow. Exactly one at a time, and no shared borrows
  coexist with it.

The rule "many readers XOR one writer" is what prevents data races and iterator invalidation
*at compile time*. To enforce it, the compiler must know *how long* each borrow lasts — its
**lifetime**, which is really a **region** of the program. The **borrow checker** is a dataflow
analysis that solves these region constraints. Modern Rust uses **NLL (Non-Lexical Lifetimes)**:
a borrow lasts only until its last *use*, not until the end of its lexical scope, computed by
liveness analysis. (The next-gen engine, **Polonius**, reformulates this with more precision.)

The thing to internalize:

> The borrow checker is a **decidable, proof-free, hardcoded** analysis. It is *not* derived
> from a general proof layer; it is a special-purpose dataflow pass with a fixed notion of
> "region." That is exactly why it is cheap and always terminates — and exactly what property
> #3 of the dream forbids (it wants ownership *derived from the proof layer*, not hardcoded).

This is the second fork. Rust gets memory-safety-without-GC by **hardcoding** an affine borrow
checker. The dream wants the same guarantee *derived* from a unified, decidable, proof-erasing
layer. newlang's calibration control (Part 6) proves that the hardcoded version settles cleanly
and the *derived* version is precisely the impossible part.

---

# Part 4 — PL bootstrap III: the limits, and where proofs come back

The borrow checker is powerful but it cannot see everything. The places it fails are exactly
where real systems code lives, and they are where the "seam" of the project comes from.

### Copy handles defeat affinity

Affinity works because a value can't be duplicated. But some handles *must* be copyable — an
array index, a slab key, a `Copy` integer. The moment a handle is `Copy`, the borrow checker
can no longer track "is this the unique live reference?" — you can copy it freely and it has no
idea. This is why Rust cannot put **interior mutability** (mutating through a shared `&`) into
the static checker, and instead provides:

- **`Cell`/`RefCell`** — move the borrow check to **runtime** (a flag checked on each access;
  `RefCell` panics on violation). You pay a runtime cost for a guarantee the static system
  couldn't give.
- **`UnsafeCell`** — the primitive that says "I, the human, will uphold the invariant; compiler,
  trust me." Every safe interior-mutability type is built on it.

Hold onto this: **the static checker is blind to a `Copy` handle's validity, and the standard
escape is a runtime check (`RefCell`).** newlang's slab primitive is exactly this pattern, and
its lesson — that you can recover soundness at *runtime cost* — is the project's deepest finding.

### Separation logic: reasoning about the heap and aliasing

To reason about programs that *do* alias and mutate shared memory, you need a logic richer than
types: **separation logic**. Its key connective is the *separating conjunction* `P * Q` —
"the heap splits into one part satisfying P and a *disjoint* part satisfying Q." It makes
ownership and non-aliasing first-class. **Concurrent separation logic** adds reasoning about
threads and shared state. The state-of-the-art framework is **Iris** (a higher-order concurrent
separation logic, mechanized in Coq) — it is the power tool for proving that tricky concurrent,
aliasing, `unsafe` code is actually sound.

### RustBelt: why `unsafe` can be *justified* but not *checked*

Here is the deepest prerequisite. Rust's safe fragment is sound, but its standard library is
full of `unsafe` (in `Vec`, `Arc`, `Mutex`, channels…). Why is *that* sound? **RustBelt**
(Jung, Jourdan, Krebbers, Dreyer — POPL 2018) answered this: it built, in Iris, a *semantic
model* of Rust types and proved that each `unsafe` abstraction, behind its safe interface,
upholds the model. The profound consequence for newlang:

> The safety of an `unsafe` block is a theorem in undecidable concurrent separation logic. It
> can be **proven once, by a human, with a proof assistant** — but it cannot be **decided by a
> compiler**. The proof does not fit in the type system; it lives in a paper and a Coq
> development. This is *the* seam: a checked, decidable world (safe Rust) sitting on top of a
> trusted, separately-proven world (the `unsafe` core).

### The unsafe boundary as a trusted computing base

Every verified system has a **TCB** — a trusted computing base, the part you *assume* correct
rather than prove. `unsafe` is Rust's TCB boundary; an `assume` in a proof is one; in seL4,
CompCert, and F\*/Low\* the TCB is explicit and minimized. A crucial distinction the project
leans on: some facts are *unprovable in principle from inside the program* — e.g. "this memory
address really is the UART register" is a fact about *physical hardware*, an axiom you discharge
by reading a datasheet, not a theorem any checker in any language could decide. That is a
*different kind* of trust than "this is provable but we didn't mechanize it." (newlang's MMIO
primitive is exactly this case.)

### Weak memory: why concurrency is even harder

On real hardware (ARM, POWER, RISC-V), memory operations are *not* globally ordered — different
cores can observe writes in different orders. To write correct lock-free code you use
**acquire/release** atomics to establish **happens-before** edges: a `Release` store
"publishes" everything before it, and an `Acquire` load that reads it "sees" all of that. The
safety of a lock-free queue *is* an argument in this weak-memory model. (newlang's SPSC queue
puts exactly two such atomics at its core — see Part 9.)

---

# Part 5 — State of the art: the landscape of safe systems languages

Everything above sorts the field into a map. Every real system **picks a horn** of the same
trilemma the project later names. Knowing this landscape is most of being literate here.

**The decidable, cheap, proof-free end** (give up expressiveness / accept a seam):
- **Rust** — affine ownership + borrow checking, hardcoded and decidable; `unsafe` is the seam,
  and the obligation at the seam lives "in the programmer's head," not the type system.
- **Flux**, **Liquid Haskell** — refinement types over a decidable SMT fragment. Great until the
  property leaves the fragment.

**The proof-bearing end** (give up decidability / accept user proofs):
- **Verus** — Rust + SMT-checked proofs; you write `requires`/`ensures` and proof code.
- **F\* / Low\*** — dependently typed, SMT + tactics; Low\* extracts to C (used for verified
  crypto in Firefox/Linux — the HACL\* project).
- **Creusot**, **Prusti**, **Aeneas** — Rust verification via SMT (Creusot/Prusti) or a pure
  functional translation for proof assistants (Aeneas).
- **Dafny** — verification-oriented language, SMT-backed.
- **Kani** — bounded model checking for Rust (no proofs, but *bounded*, not a full guarantee).
- **VeriFast**, **CN** — separation-logic verifiers for C.

**The cautionary tales** (tried to have more, paid for it):
- **ATS / ATS2** — extremely expressive linear+dependent types; the canonical *ergonomics
  cliff*: theoretically capable, practically punishing to write. (newlang's attackers use "is
  this the ATS2 cliff?" as a standing test.)
- **Cyclone**, **Mezzo** — research languages with rich region/permission systems; influential,
  not adopted.
- **Linear Haskell**, **Austral**, **Cogent** — linear-types experiments, each conceding
  something (ergonomics, scope, or a restricted domain).

**The specific tools newlang's primitives use:**
- **GhostCell** (Yanovski, Dang, Jung, Dreyer — ICFP 2021) — a *branding* trick: an invariant
  lifetime `'brand` plus a token, letting you safely mutate aliased graph structures *if* you
  thread the token through every access. The soundness rests on RustBelt-style reasoning.
- **slotmap / generational-arena** — `Copy` keys made safe by a per-slot **generation counter**
  checked at runtime; the production answer to "how do you hand out copyable handles into a
  pool without UAF."

**The one sentence that organizes the field:** *every shipping system is decidable-and-cheap
with a trusted seam (Rust), or expressive-and-sound with human proofs (Verus/F\*), and none is
all three.* That observation, made rigorous, is the project.

---

# Part 6 — The method: adversarial design with cold AI instances

newlang's twist is *how* it established impossibility. You cannot prove "no language has all
seven" by enumeration. So it ran an **adversarial design process** and watched it fail in a
*structured, convergent* way.

### The roles

- **PROPOSER** — produces one concrete, formal design decision (a type rule, an algorithm).
- **Five cold ATTACKERs**, each with one angle: *ergonomics* (will anyone write this?),
  *soundness* (exhibit a well-typed program that does UB, or show checking is undecidable),
  *implementation* (which compiler pass blows up?), *expressiveness* (what real program can't
  be written?), *prior-art* (who tried this and what did it cost them?).
- **JUDGE** — rules each attack genuine-flaw / argued-past / tradeoff, then SURVIVED / FAILED.
- **SYNTHESIZER** — when a question fails every revision, distills the *convergent reason* into a
  recorded property-incompatibility (the payload).
- **RECONCILER**, **HISTORIAN** — check pairwise collisions of settled decisions; record state.

### "Cold" is the load-bearing trick

An attacker must reason from the proposal *alone* — no shared context, no memory, no knowledge
of who wrote it. The project runs each role as a fresh `claude -p` subprocess with all ambient
context stripped (`--system-prompt`, empty settings so no hooks, no MCP, a clean working
directory). Statelessness is *demolition's* correct property: a cold reviewer can't be charmed
by the design's backstory. (The mirror image — *construction* needs continuity — is why the
later build phase uses a stateful builder and keeps the cold instances as the red team only.)

### Calibration: why a 0/N result needs controls

If a process settles *nothing*, that is ambiguous: either the target is genuinely impossible, or
your instrument is rigged to reject everything. You break the ambiguity with **controls**:
- **Good control** — feed it a known-good, shipping design (Rust's borrow checker, with the
  impossible constraints waived). A calibrated process must *settle* it. (It did — and on the
  first try it even caught a real lifetime bug before accepting the fix.)
- **Bad control** — feed it obvious garbage ("no checker, programmers just don't write bugs").
  A calibrated process must *reject* it. (It did.)

Only a process that settles good designs and rejects bad ones makes a 0/N on the real questions
*signal*.

### Decontamination: the move that makes the result trustworthy

The first attempt to design a "seamful" language kept too many constraints, so failures mixed
"the seam is impossible" with "the proposer couldn't write a bug-free borrow checker." The fix
is the project's sharpest methodological idea: **hand the proposer the known-good sub-component
and watch whether the failure *moves* or *disappears*.** They stipulated "use Rust's borrow
checker exactly as shipped" — a concession the good control had *already proven costless* — and
the failure did not vanish. It **relocated** onto the seam itself. A failure that *moves* is a
real structural obstruction; one that *disappears* was just proposer weakness. (You will see
this exact move pay off across all eight primitives in Part 9 — where the obstruction *vanishes*,
*moves and stays decidable*, *moves onto a runtime witness*, or *moves and breaks* is the entire
verdict each time.)

---

# Part 7 — The negative result: the decidability fault line

Run the full process over the seven properties, decomposed into five design questions
(sequential ownership; the decidable fragment; proof erasure; the unsafe boundary; interior
mutability & weak memory). Result: **0 of 5 settled, 6 hard incompatibilities, and every single
one places property #4 (decidable, no user proofs) on one side of the conflict.** The dream
cracks along one fault line:

> **decidable-and-cheap  XOR  sound-derived-seamless-zerocost.**

Two readings matter:

1. **It's a covering, not an intersection.** For *each* property individually, *some* language
   has it (Rust, Lean/Coq, RustBelt). A few languages *together* cover all seven. But no single
   language has all seven, because each concedes one of #2/#3/#6. "A few languages together do
   everything" is true and is *not* the same as "one language does everything."
2. **Merging is itself the forbidden move.** "Combine them into one seamless language" *is*
   property #2 (no seam). So the act of fusion is one of the things you must give up. The
   un-fusable residue *is* the seam — and the seam is *load-bearing*, because it's where the
   undecidable obligation gets quarantined behind a trusted boundary. Erase the seam and the
   undecidability reappears at the weld.

This is the conservation law the whole project circles: you cannot delete essential difficulty,
only move it.

---

# Part 8 — The constructive flip: the Seam Trilemma

If the seam is unavoidable, stop fighting it and *design it*. Concede property #2 (a seam is
allowed) and ask: what is the best *seamful* language? Re-running the process under that framing
(with the borrow checker stipulated, per decontamination) yields a clean three-way constraint —
the **Seam Trilemma**. A seam is the boundary where a primitive whose true precondition lives in
rich, undecidable separation logic is re-exported as a checked safe interface. You want three
things; you can have any two:

1. **Sound re-import** — crossing back from the seam launders no UB into safe code.
2. **Decidable, proof-free safe fragment** — ordinary code type-checks with no user proofs, no
   SMT, no threaded token.
3. **Seam concentration** — ordinary code touches the seam *rarely* (ideally once, at
   construction), not on every operation. (The opposite is the ATS2 ergonomics cliff.)

Why you can't have all three (the mechanized core, `proof/SeamGap.lean`): the foundational
primitives' true preconditions are Iris/RustBelt-grade (undecidable, certificate-free). The safe
fragment can only *receive* a witness that is simultaneously decidable, proof-free, and erasable
— strictly weaker. So the re-import obligation is necessarily weaker than the precondition it
must certify, and the missing conjunct forces exactly one of: **drop soundness** (UB launders
in), **drop decidability** (an external prover / user proofs), or **drop concentration** (every
use crosses the seam). Every shipping language picks a horn: Rust drops concentration informally;
Verus/F\*/Creusot drop decidability; the unsound corner is the one to avoid (and the bad control
shows it's detectable).

**The productive reading:** a seamful language *is* worth building. The sweet spot is "Rust, but
`unsafe` is `proof`" — drop the decidability horn at the seam, keep the safe fragment cheap, and
concentrate the proof burden in a few foundation authors. The value of the project is knowing
you are *choosing a horn*, not escaping the trilemma — and which horn, and why the fourth option
doesn't exist.

---

# Part 9 — Build and break: thirty-two real seams (the heart of it)

Theory in hand, the project stopped arguing and *built*. Crucial design of the experiment:
**build and break are different machines.** Construction is continuity, so a *stateful builder*
(a normal, context-holding process — here, me) writes the smallest thing that compiles.
Demolition is coldness, so the cold harness is the **red team**, aimed *only* at the re-import
rule. Each seam ships as a **text spec** (what the cold attackers attack) *and* a real,
`rustc`-checked crate (`seam/`) whose consumer code is `#![forbid(unsafe_code)]` and whose UB-
laundering attempts are `compile_fail` tests. Seventeen primitives: the first four chosen because their
preconditions have four *different shapes*; the next three (5–7) because they reopened a deeper
question — *how many axes is the trilemma actually a projection of?* — each recovering one; and **ten
probes past seven** (8–17) — one *no* (rejected), two *yes* (unwind-safety at N ≥ 8, leak-freedom at N ≥ 9,
ruled *distinct*), one *"real but inseparable"* (move-stability, N stays 9), one that asked whether the
axis-list was *finished* and got back *"yes — the basis is closed"* (validity), and **five that left memory
safety entirely**: a constant-time secret (*"that is a **different domain** — closure is per-domain"*),
a capability-gated declassification (*"and no seam closes in that domain on this floor — the **floor
is mismatched** to it"*), a type-level lock ordering in a *third* domain, liveness (*"a seam half-closes
here — the floor match is **partial**, not yes-or-no"*), a *runtime* lock registry that **upgraded the
floor** to try to lift that partial match (*"the upgrade is **non-monotone** — it fixes one gap and the
granularity gap recurs a rung finer; the grade can't be bought up"*), and a must-run cleanup guard probing
a *different precondition* in that liveness domain (*"a clean **none** — and the floor is the **wrong
structural logic**, affine vs linear, no upgrade fixes"*). The verdicts together are the point: the last
six *conclude* the arc, *generalize* it, *bound* the generalization to the floor, *grade* that bound, show
*the grade is a ladder no floor tops out*, then show *the floor itself can be the wrong structural logic*.
The headline finding:

> **Whether a seam closes is a function of the precondition's shape** — specifically, *what the
> dangerous conjunct is*: can the cheap checker carry it, can a runtime check carry it, is it an
> external axiom, or does it escape to the caller entirely? **And when a seam *does* close, *which*
> of several hidden axes it spends to do so is a second, independent question** — the one the fifth
> primitive forced open.

### Primitive 1 — SPSC lock-free ring buffer → the seam CLOSES (`seam/src/lib.rs`)

A single-producer/single-consumer queue: one thread pushes, one pops, no locks, two
acquire/release atomic indices. Its true precondition is genuinely hard (per-slot exclusion,
happens-before publication, single-producer cardinality, payload thread-safety). And yet **every
conjunct that the caller could observe is carried by the floor for free**:
- *Cardinality* (only one producer) → **affinity**: the `Producer`/`Consumer` handles are
  move-only (`!Clone`), so you literally cannot make a second producer.
- *Storage lifetime* (no use-after-free) → **NLL lifetimes**: `split(&'a mut self)` hands both
  handles a `&'a` borrow of the buffer, so the borrow checker forbids the buffer from dropping
  while either handle lives. UAF becomes a *compile error*.
- *Payload safety* → a **decidable `T: Send` bound** the compiler resolves; the consumer never
  writes it.
- *Happens-before* (the genuinely undecidable part) stays **inside the `unsafe` block**, leaving
  *no caller-visible residue*.

Cold verdict: **SEAM-HOLDS** across four independent panels. This is not luck — it is literally
what `heapless::spsc` / `rtrb` ship, and RustBelt machine-checked the class. **Seam width: O(1)
per buffer** (cross once at `split`, then `push`/`pop` are free). When the dangerous fact fully
*encapsulates*, the seam closes losslessly.

### Primitive 2 — MMIO read-to-clear register → SOUND modulo an external axiom (`seam/src/mmio.rs`)

A memory-mapped device register where *reading clears* the pending bits. An affine singleton
handle + encapsulated volatile access makes the memory-safety seam close to O(1) just like SPSC.
But one conjunct of its precondition — "this address *really is* this register, with these
datasheet bit-semantics" — is **a fact about the physical world**. No checker, in this language
or any, can decide it; it is an **axiom discharged by fiat** at one `unsafe` line. The cold
harness named the category exactly right: this is a **TCB `assume` boundary** (the same kind
seL4, CompCert, Low\* rely on), **not a fourth trilemma horn**. The real recordable *cost* is a
horn-3 concentration nudge (the interrupt-handler use needs a foundation `Send` opt-in). Verdict:
**SEAM-COSTS**. Lesson: some obligations aren't "undecidable," they're *outside the formal system
entirely* — and the honest move is to *name* the axiom, not hide it.

### Primitive 3 — generational slab → the seam CLOSES, at RUNTIME cost (`seam/src/slab.rs`)

A pool that hands out `Copy` keys (`insert -> Key`, `get(Key)`, `remove(Key)`). The handle
*must* be `Copy` — storing and passing keys around is the whole point — so **affinity is
useless** (recall Part 4: a `Copy` handle defeats the static checker). The danger is stale keys
and ABA: a copied key outliving its slot. The fix is exactly `RefCell`'s move, generalized: a
per-slot **generation counter**, bumped on `remove`, checked at runtime; `get` returns `None`
for a stale key instead of reading freed memory. This keeps **all three trilemma horns** (sound,
decidable, concentrated — the whole interface is safe, seam width O(1)) — and pays on a **fourth
axis the trilemma folded away: zero-cost erasure (property #6).** The generation word per slot
and the compare-branch per access *do not erase*. Verdict: **SEAM-COSTS [zero-cost]**.

This primitive forced the **first** of four un-bundlings:

> **The Seam Trilemma is a 3-of-4 projection.** "Concentrated & proof-free" had quietly bundled
> away *zero runtime cost*. The slab prises them apart: a *concentrated* seam can still be
> *non-erasable*. Seam width (horn 3) and zero-cost (#6) are independent axes — the slab aces the
> first and spends the second. And #6 was one of the casualties back in Part 7's unified result;
> the constructive phase puts it back on the map. *(The fifth, sixth, and seventh primitives, below,
> force three more un-bundlings — API-locality, then legibility/solver-freedom, then bare-metal —
> turning "3-of-4" into "3-of-≥7". Then **two eighth-axis probes**: the eighth is **rejected** (its
> axis confounds with expressiveness), the ninth **confirmed** (unwind-safety, a clean off-diagonal) —
> "3-of-≥8". That the method rejects one and confirms the other is itself a result.)*

*(A teaching aside from the real run: the slab's **first** cold gauntlet FAILED on a genuine UB —
but the bug was in the **spec** (a generation-only check authenticates a fresh pool's vacant
slot 0), while the **implementation** checked occupancy first and was sound. A regression test
proved it. That spec↔implementation divergence is exactly what the build-attacks-text /
code-validated-by-compilation separation exists to catch. The fix is slotmap's real
`version | 1` parity trick.)*

### Primitive 4 — intrusive doubly-linked list → the seam BREAKS (`seam/src/intrusive.rs`)

Nodes contain their own `next`/`prev` links and are owned *elsewhere* (an arena, the stack). Now
the danger **escapes to the caller**: nodes alias each other while the caller still owns them.
Affinity is *defined* as the absence of aliasing, so the SPSC recipe is structurally
inapplicable. The sound tool is **GhostCell branding** — but the cold harness proved something
sharper than expected: **a brand is an *access* capability, not a *storage* lifetime.** The token
discharges aliasing (who may form `&mut`) but carries *no* relation between a node's membership
and its allocation — so the rule type-accepts a **use-after-free** (a still-linked node drops,
a later traversal reads freed memory). Verdict: **SEAM-BREAKS [horn 1]**. Every repair just
relocates the horn: tie nodes to one lifetime → lose early reclaim (drop *usability*, the
primitive's purpose); full separation-logic proof → drop *decidability*. The trilemma bites
hardest here.

### Primitive 5 — branded index → the seam CLOSES, and the *dimension count grows* (`seam/src/branded.rs`)

This one was chosen as a deliberate probe. The slab proved zero-cost is a hidden fourth axis; the
test for whether there is a *fifth* is to find a primitive that aces **all four** known axes —
sound, decidable, concentrated, *and* zero-cost — yet still pays. **Sound unchecked indexing**
(Beingessner/Gankra's thesis Ch. 6.3; bluss's `indexing` crate) is exactly that. A *branded* index
is a raw `usize` tagged with a generative lifetime `'id` minted fresh per `scope(slice, |view| …)`;
because `'id` is unique to one scope over one immutable (length-frozen) slice, an index that passed
*one* bounds check is provably in-bounds forever — so indexing elides the check. It is the first
seam here that is **faster than safe code**, and the brand is a zero-size `PhantomData` that erases
completely. All three horns hold; zero-cost is aced. So what's left to pay?

The cold panel — grounding the answer in four decades of literature — found a real cost *outside
the three horns*: the brand `'id` is a **viral generative type parameter**. It never appears in
consumer *code* as a seam crossing (seam width is O(1), one `scope` per slice), but it appears in
every consumer *type*, and traps every index inside the `scope` closure (an `Idx<'id>` literally
cannot be returned). That is **API-locality**, and it is provably *not* seam width: Launchbury &
Peyton Jones's `runST` (1994) and Kiselyov & Shan's branded indices (2006) pay the identical tax in
Haskell — a language with *no `unsafe` and no seam at all*. A cost that exists where seam width is
undefined cannot *be* seam width. Verdict: **SEAM-COSTS [API-locality]**. (The panel also caught a
builder overclaim — naive single-index `try_index` re-checks per element, so the zero-cost win
needs the *range*-branding the toy scoped out — and noted that AI-legibility (#7) is yet another
axis: a brand is a nominal token, whereas a refinement type `{v:Int | v < len}` is brand-free *and*
legible, so #5 and #7 dissociate.)

This forced the **second** un-bundling, and the project's deepest structural result:

> **The Seam Trilemma is a 3-of-N projection, N ≥ 7.** The slab un-bundled zero-cost from
> "concentrated & proof-free"; the branded index un-bundles **"concentration" itself** into a
> *code* face (seam width) and a *type* face (API-locality). Beyond the three horns the constructive
> primitives expose more independent axes — zero-cost (#6), API-locality, AI-legibility (#7), and
> bare-metal (#5) — each proven real by a *witness that two trilemma-identical configurations
> differ*. They even have structure: within the branded index, API-locality and #6 **trade**
> (range-branding buys erasure by deepening the brand tax). The full face lattice is
> `design_seam/REALIZABILITY.md`. *(Two claims are still open here — that #7 is independent of
> API-locality, and that bare-metal is a separate axis. The sixth and seventh primitives witness them.)*

### Primitive 6 — refinement-typed bounds → CLOSES the cell #5⟂#7 (`seam/src/refined.rs`)

The branded index left a gap: it pays #5 *and* #7 together (a brand is viral *and* a nominal token),
so it cannot prove they are *separate* axes — for that you need an **off-diagonal**, a primitive that
agrees with another on #5 but differs on #7. That is what a **refinement type** gives. Instead of a
brand, the witness is a **legible predicate**: `i: usize{i < len}` is "a `usize` the checker has
*proven* less than `len`," discharged by an SMT solver over a decidable arithmetic fragment (this is
**Flux** for Rust, Lehmann et al. 2023, and **Liquid Haskell**, Vazou et al. 2014). It is the exact
opposite corner from the brand:

- **brand-free** (#5 aced): a `Refined` is a plain value — you can store it, return it from a
  function, collect it into a `Vec`. The keystone test does exactly that, and it is a *compile error*
  for the branded `Idx<'id>` (whose `'id` cannot escape its scope). No brand, no closure.
- **legible** (#7 the opposite of the brand): the predicate `value < bound` *is* the type's contract;
  an outside reasoner reconstructs the safety invariant from the signature alone.

So pair it against the **slab**, reclassified **(brand-free, *illegible*)** — a `Copy` `Key`'s
live-slot validity is *not* reconstructible from its type (the earlier table's "legible" was too
generous, and two cold attackers independently endorsed the correction). Refined-vs-slab now holds
#5 fixed (both brand-free) and **flips #7**; branded-vs-slab holds #7 fixed and flips #5. The cold
judge ruled it outright: **"#5⟂#7 close? — YES."** AI-legibility is a *confirmed* independent axis.

And the refined index forced a **third** un-bundling — of **horn 2** itself. A refinement type is
*decidable* (the fragment terminates) but not *solver-free*: it needs an SMT solver in the trusted
toolchain. So "decidable & proof-free" was a bundle of **decidability** and **solver-freedom**, and
the refined index pays the second. (Honest guard: that solver-in-the-TCB cost is *not* a new seventh
axis — it is the original `unified` result's "keep the invariant faithfully → external prover" horn,
the documented founding tradeoff of refinement types since DML in 1998. The map gains *resolution*,
not a dimension.) Two more cold findings sharpened the artifact: the **R2 trade** (you can have
brand-free *or* zero-cost, not both — the #5↔#6 exchange again, from the other side), and a real
expressiveness bug (the model checked `bound == len` where safety needs only `bound ≤ len` — fixed).
Verdict: **SEAM-COSTS [horn 2]**.

### Primitive 7 — reference-counted shared ownership → recovers BARE-METAL (`seam/src/shared.rs`)

The first six seams share a hidden uniformity: none needs a heap. They are `core`-only — fixed
buffers, fixed pools, borrowed slices — so they run on bare metal (an OS kernel, a microcontroller
with no allocator). The seventh breaks it, and recovers seed property **#5 (bare-metal)** as an axis.

The new precondition shape is **dynamic shared ownership** (a minimal `Rc`): a value with multiple
owners, where the live-owner set is decided *at run time* by `clone`/`drop`, and the value is freed
exactly when the *last* owner leaves. That instant is not statically knowable — affinity gives you
*one* owner, NLL gives you *static* lifetimes, and the slab's generation check still has a single
owner (the pool). The only sound witness is a **runtime reference count**, and a refcount needs
storage that outlives every owner and is reclaimed on the last drop — i.e. a **runtime manager**,
here the global heap. The cost is made *compile-enforced*: the crate is `#![no_std]`, and `shared` is
the sole reason it pulls in `extern crate alloc` — delete it and the other six still build. A no-heap
kernel author simply cannot link this seam. That is the bare-metal axis.

It is genuinely independent of zero-cost (#6) — the **off-diagonal vs the slab** proves it: the slab
pays a runtime cost (#6 ✗) yet is bare-metal (a fixed pool, no heap); `Shared` *also* pays a runtime
cost (the refcount) *and* needs the heap. They agree on #6 and differ on bare-metal — so a runtime
*check* and a runtime *manager* are not the same thing. Verdict: **SEAM-COSTS [bare-metal]**.

But here the cold panel did its most useful work yet — it **deflated the novelty**. Yes, bare-metal
is a real, independent seventh axis. But it is not a *discovery*: it is precisely Rust's own
`core`/`alloc`/`std` layering (RFC 2480), and `heapless::pool`'s core-only refcounted pointer already
proves the trade (shared ownership doesn't *force* the heap — *unbounded dynamic count* does; a
bounded pool stays bare-metal). The honest name is *unbounded-dynamic-allocation vs
caller-provided-bounded-storage*. **Keep the axis; drop the "undiscovered dimension" framing.**

That deflation — the second of two (solver-freedom was the first) — is the project's most honest
result. *Every* axis the method recovers turns out to be an **already-named PL concern**: zero-cost
abstraction, generativity, refinement-legibility, the `core`/`alloc` split. newlang did not discover
new physics. It found that the trilemma's three horns silently *bundle a set of independently-known PL
design axes*, and that one question — **"what does the safe witness cost?"** — unifies them. The
contribution is the unification and the recovery method, not the dimensions.

### Primitive 8 — type-level-proof index → the eighth probe FAILS (`seam/src/typed_nat.rs`)

The first seven primitives each recovered (or, for MMIO/intrusive, located) something. The eighth was
built to *push the count to eight* — and the gauntlet **rejected it**. That rejection is the most
instructive moment in the project, so it gets a worked example of its own.

The idea: prove an index in bounds *entirely at compile time*, with no runtime check and no external
solver. Encode index and length as type-level naturals (`Z`, `S<Z>`, …); the in-bounds fact `I < Len`
is the trait bound `I: Lt<Len>`, discharged by two recursive `impl`s the trait resolver chases. The
proof carries no data, needs no Z3, and erases to a bare `get_unchecked`. It is **sound** — and the
panel conceded that unanimously: `Nat` is a *sealed* trait, so a consumer can impl the public `Lt` for
their own local type (orphan rule) but that type can never be `Nat`, and `get`'s `I: Nat + Lt<Len>`
bound rejects every forgery. No UB path exists.

The builder pitched an eighth axis — **checker-tractability**: the refined index needs a solver but
that solver halts *cheaply*; this one needs no solver but its checker halts only under an arbitrary
`#![recursion_limit]` (resolving `I: Lt<Len>` costs `I::VALUE` steps, so a deep-but-in-bounds index
fails with E0275 "overflow" until you raise the knob — out of fuel, not out of bounds). Held against
SPSC, the claim was: flips *only* checker-tractability.

Three independent attackers killed the axis (the seam stays sound; only the *deliverable* failed):
- **The off-diagonal is confounded.** `get::<I>` takes a *type*, never a runtime `usize` — so this
  seam can only index compile-time *constant* positions. It is a strict expressive subset of the
  branded/refined indices, whose entire reason to exist (`get_unchecked` on ring/arena/slab) is
  *data-dependent* indexing this seam **cannot express**. The "one-coordinate flip" silently also
  flips expressiveness — and you cannot read an independence off two primitives that don't accept the
  same programs. The confound is *intrinsic*: a runtime index carried by a compile-time proof is a
  contradiction.
- **The cost is encoding-relative.** The E0275 wall is an artifact of *unary* Peano. *Binary*
  type-level naturals (typenum's `UInt<U,B>`) make resolution O(log N) — they never hit the limit for
  realistic indices. The "intractable" cell measures *which numeral library you linked*, not the
  primitive.
- **It is mislabeled.** `Lt` resolution is *linear* — strictly *cheaper* than the refined index's
  NP-hard QF-LIA. So the cost was never "tractability." What `recursion_limit` marks is the absence of
  a guaranteed *terminating-resolution* fragment — exactly GHC's `UndecidableInstances` / the Paterson
  conditions / `-freduction-depth`. A real, **pre-existing** concern, not a new lattice axis.

Verdict: **FAILED [the axis deliverable]; the seam is sound.** The count stays **N ≥ 7** *on this
probe*. The lesson is in Part 11 (#8): a discovery procedure that cannot reject a bad candidate is not
measuring anything. But the *next* primitive shows the other half — the same method confirming a clean
candidate.

### Primitive 9 — take-and-replace → the eighth axis, CONFIRMED (`seam/src/take.rs`)

The second eighth-axis probe, built deliberately to avoid Primitive 8's failure mode: a primitive whose
candidate cost is isolable *without* dragging another axis along. The danger is different in kind — not
aliasing or bounds but **time**. `take(slot, f)` moves the value out of a `&mut T` (`ptr::read`), runs
`f`, and writes the result back. Between the read and the write there is a **gap**: the slot holds a
bitwise *duplicate* of the old value, a `T` that must never be dropped or read. The one thing that can
run arbitrary code in the gap, on Rust's floor, is **unwinding** — if `f` panics, the stack unwinds,
`f`'s frame drops the real value, and then the caller would drop the *duplicate* still in the slot: a
**double free**. This is the exact UB of a naive `take_mut`.

The sound witness is an **abort-bomb** (`take_mut`'s real pattern): a zero-sized local whose `Drop`
panics, armed across the gap. If `f` panics, unwinding drops the bomb *first* (it is `take`'s inner
local; inner frames unwind before outer ones), and a panic *during* unwinding is a **double-panic →
process abort** — which fires before the caller can drop the duplicate. The broken slot is never
observed. On the happy path the bomb is `mem::forget`-ten and nothing residual runs.

Here is the clean off-diagonal Primitive 8 lacked: this seam **aces all seven prior axes** — sound,
decidable & proof-free, O(1)-concentrated, brand-free, zero-cost on the happy path, legible, `core`-only
— and pays on **exactly one** new thing: U2, *unwind-safety*. Identical to SPSC on every named axis,
split on the candidate alone. Verdict: **SEAM-COSTS [none]** (a TRADEOFF — sound, all horns held). The
cold panel **confirmed unwind-safety as the eighth axis (N ≥ 8)** and explicitly contrasted its clean
isolation with Primitive 8's confound. Deflated, as ever, to pre-existing: Abrahams' basic/strong/nothrow
exception-safety guarantees, C++ `noexcept`, Rust's `UnwindSafe`/`catch_unwind`, `Mutex` poisoning.

The panel added two corrections that are now part of the result:
- **The axis is the *exception-safety-guarantee-level*, not "the abort cost".** Abort is merely *this
  seam's* discharge of U2. A `replace_with`-style sentinel (`T: Default`) discharges the *same* U2
  **soundly and recoverably** by filling the gap with a valid value — paying **API-surface** instead. A
  lock pays by **poisoning**. So the axis carries a **discharge menu**: one obligation, several pay-points
  that each spend a *different other* resource. (A naive move-out/move-back with no guard is just unsound.)
- **Historical honesty:** RFC #1508 *rejected* exactly this `take_mut` from `std` on precisely the
  abort-downgrade cost — the "ace seven, pay one" framing was the historically *losing* argument.

So the two eighth-axis probes together are the whole instrument: Primitive 8 is the method **rejecting**
a confounded candidate, Primitive 9 is the same method **confirming** a clean one (and relabelling it).
N reaches 8 only on a witness that holds every other axis fixed.

### Primitive 10 — drain-and-yield → the ninth axis, CONFIRMED (`seam/src/drain.rs`)

Having confirmed unwind-safety, the obvious next question: is there a *sibling* — and is it the same
axis or a different one? `Drop` not running *normally* has two failure modes: it runs at a **bad time**
(an unwind, Primitive 9), or it **never runs at all** (a leak). Rust makes the second a first-class
hazard, because **`mem::forget` is a *safe* function** (so are `Box::leak`, `ManuallyDrop`, and `Rc`
cycles) — you can always *skip* a destructor. This is the famous Rust **leakpocalypse**.

The danger primitive is `Vec::drain`: an iterator that moves elements out of a buffer by value. While
it iterates, the already-yielded slots are *moved-from* (bitwise live, must not be dropped). The
historical pre-1.0 bug ("leak amplification"): if the `Drain` guard is leaked mid-iteration and the
buffer still counts those slots, the buffer's `Drop` double-frees them. The sound witness is
**pre-pessimization** (the real `Vec::drain` fix): set the buffer length to **0 up front**, before
yielding anything; the guard tracks the un-yielded range to drop on its own `Drop`. Then a leaked guard
leaves a valid *empty* buffer — the un-yielded tail is **leaked** (a memory leak, not UB), never
double-freed. (Because `mem::forget` is safe, the consumer test can exercise the leak path *itself*
under `#![forbid(unsafe_code)]` — a rare keystone.)

The crucial design choice, learned from Primitive 8: a `Drain` is a **free-standing** value — a plain
`&mut` borrow, no closure. That matters because the *obvious* leak-freedom witness, a scoped-thread
guard (`thread::scoped`), would have confounded with **API-locality**: its sound form is a closure
(`thread::scope`), and "forced scope-closure" is *by definition* an API-locality cost. The drain guard
sidesteps that — it buys leak-safety by *owning its data* and downgrading a guarantee (leak ⇒ lose the
tail), so it needs no closure and aces API-locality. It aces all eight prior axes and pays on **exactly
one** new thing: leak-freedom. Verdict: **SEAM-HOLDS / SURVIVED**, the ninth axis confirmed (N ≥ 9),
deflated to pre-existing (the leakpocalypse; `mem::forget`-is-safe; RFC 1066; the `JoinGuard`/
`thread::scoped` removal; linear-vs-affine types).

The load-bearing ruling: **leak-freedom is *distinct* from unwind-safety, not a merged
"destructor-robustness" axis.** The off-diagonal is populated both directions — take-and-replace is
*unwind-sensitive* (needs the abort-bomb) but *leak-safe* (forgetting the bomb is fine); drain is
*leak-sensitive* (needs pre-pessimization) but *unwind-safe* (a panic mid-iteration just runs the
guard's `Drop`). "`Drop` runs at a bad time" and "`Drop` never runs" are independently witnessed, and
Rust's own docs codify them separately (*Exception Safety* vs *Leaking*). One honest nuance the panel
flagged: leak-freedom is the **thinnest, most-deflatable** axis in the lattice — its cost (leaked-tail
data loss) is invisible in all non-leak code — yet it is a genuinely populated off-diagonal, not the
non-axis the checker-tractability probe turned out to be. "Deflatable" and "not an axis" are different
verdicts; this is the former.

So three probes past seven map most of the instrument's behavior: **reject** the confounded (8),
**confirm** the clean (9), **confirm-and-distinguish** the sibling (10). The fourth probe found a mode
none of those covers.

### Primitive 11 — self-referential cell → the *inseparable* verdict (`seam/src/selfref.rs`)

The danger here is new in kind: a value whose **address** is load-bearing. A `SelfRef` holds a `u64`
and a `*const u64` that, after `init`, points at its own `value` field. Reading through that pointer is
sound **only if the struct never moves after `init`** — a move relocates `value` but leaves the pointer
aimed at the old address, a dangling read. None of the nine confirmed axes is about a value's address,
so move-stability is a *genuinely distinct* danger.

The floor's witness is **`Pin`**: a `Pin<&mut Self>`/`Pin<&Self>` over a `!Unpin` cell (here via
`PhantomPinned`) statically forbids the move — every move path (`swap`/`replace`/`get_mut`/return) routes
through a capability that needs `Unpin`, which `PhantomPinned` denies. So the seam is **sound** (the
panel conceded it unanimously; the dangling move is unconstructible in safe code).

And yet — this is the project's most novel verdict — move-stability is **not a clean tenth axis**. The
cold panel ruled it **real but structurally *inseparable* (N stays 9)**, on three grounds:
- **It cannot ace API-locality.** `Pin<&Self>` is the viral-*restrictive* wrapper: it infects every
  receiver **and revokes `&mut`** — and that revocation *is* the soundness mechanism. You cannot have the
  safety without the type-surface cost; there is nothing to isolate.
- **Its cost is *non-uniform* across witnesses.** Stack-pin (`pin!`) spends **API-locality**; `Box::pin`
  spends **bare-metal**; a hypothetical `?Move` immovable type would spend an ambient bound. One danger,
  several different costs — so it is not one mapped axis "wearing a hat" (the `typed_nat` confound) either.
- **So the honest cost is *immovability*** — the revocation of the move/return/store capability, heavier
  than API-locality (a merely leaked type surface). Move-stability funds itself by *being* immovable.

Two caveats the panel insisted on (and they sharpen the result rather than soften it): the inseparability
is **contingent, not necessary** — RFC 2349's *rejected* `?Move`/`Move` immovable types *were* a designed
free-standing witness; Rust **chose** hookless `memcpy` moves + a library `Pin`. And the crisp `core`-only
reading is the isolation of a **non-composable single-frame toy**: any real intrusive node / `Future` /
slab entry that move-stability exists to serve forces `unsafe` pin-projection or `Box::pin`. So this is a
*fourth* verdict mode, beyond {reject, confirm, confirm-and-distinguish}: **a real danger with no clean
coordinate** — the lattice's first inseparable entry. N stays 9.

### Primitive 12 — validity-checked bytes→enum → the basis is CLOSED (`seam/src/validity.rs`)

Eleven primitives in, a different question presses: are we *finding* axes, or just enumerating them
forever? Every danger so far either landed on a clean axis, a confound, or the inseparable category — but
maybe the next danger adds a tenth coordinate, and the one after that an eleventh, with no end. The
twelfth primitive is built to test that directly: **is the cost-basis closed?**

Its danger is genuinely new — a **validity invariant**. A `#[repr(u8)]` enum `Tag {A=0, B=5, C=200}` is
one byte, but only the bytes `0,5,200` are valid; `transmute::<u8,Tag>(7)` is **UB the moment it exists**
(no dereference — the compiler may assume a `Tag` is always a valid discriminant, e.g. an exhaustive
`match` with no fallthrough). No prior axis is about a value's *bit pattern*. The seam reads a `Tag` out
of a byte buffer via `transmute`-after-check; sound because the check (set membership `b ∈ {0,5,200}`)
precedes the unsafe. *(A teaching aside from the real run: the first two gauntlets **FAILED** — the first
on a genuine bug, a *range* check `b <= 200` that launders the non-contiguous bytes `3,4,6..199` into
invalid enums; the second on a fix the builder propagated incompletely, leaving a stale `b <= 2` in
another section. The adversary earned its keep to the very last primitive. The third, fully-corrected,
survived.)*

And the verdict is the **capstone of the whole arc**: *unanimously*, this brand-new danger class adds
**no tenth axis**. Every witness for it spends a coordinate *already on the map* — a single checked read
is a runtime tag check (**zero-cost #6**, the slab's axis via a different predicate), and the zero-copy
`&[u8] → &[Tag]` view that systems code actually wants is either eager-O(n) (#6) or a sealed constructor
(**API-locality**). (That non-uniformity also makes validity a *second* inseparable danger, after
move-stability — so inseparability recurs.) This is the **dual** of the `get_unchecked` family: there
*one* danger had *many* witnesses spending *different* axes; here *many* dangers (stale-key, bit-validity)
converge on the *same* axis. Read together, the two directions say the cost-space is a **small fixed
basis** that dangers *project onto*. The fifth verdict mode — **"real, but it adds nothing new"** — is the
one that *closes* the basis.

So the constructive arc concludes: the Seam Trilemma is a **3-of-N projection of a closed object** —
**nine clean axes**, an **inseparable category** (move-stability, validity), and **two outside-the-axes**
(the MMIO TCB axiom, the intrusive horn-drop). *"What does the safe witness cost?"* has finitely many
answers. (Honest caveats the panel logged: validity is sound only in the fieldless single-byte corner —
data-carrying enums need a recursive `is_bit_valid` — and the whole thing is non-novel, the shipped
design of `num_enum`/`bytemuck`/`zerocopy`. The bound stays a *lower* bound; but a brand-new danger
projecting cleanly onto the basis is what closure looks like.)

### Primitive 13 — a constant-time secret → the closure is PER DOMAIN (`seam/src/secret.rs`)

The closed basis answers "is the axis-list finished?" *within memory safety*. One question remains: is
that closure **absolute**, or is "memory safety" itself just one *domain*? The thirteenth primitive tests
it by leaving the domain entirely. Its danger is **not UB** at all — it is an **information-flow / timing
leak**: a naive `a == b` over secret bytes *short-circuits at the first mismatch*, so its **running time**
reveals the matching-prefix length — a byte-by-byte key/MAC recovery attack. No memory is corrupted; the
Rust Abstract Machine, which models no time, sees nothing wrong.

`Secret<N>` re-imports comparison as **constant-time**: it omits `PartialEq`/`Eq`/`Index`/`Deref` (so you
*cannot* branch on a secret — CF1), and vends only branchless ops (`ct_eq → Choice`, `conditional_select`)
with one loud exit, `Choice::expose_bool`. It is the `subtle` crate's pattern. Two things make this seam
unlike the previous twelve, and both are the point: there is **no `unsafe` keyword** (the danger lives in
a non-functional property the AM ignores), and constant-time-ness is **not guaranteed by the compiler**
(LLVM may re-introduce a branch).

The cold panel's verdict is the project's most generative. First, the seam **BREAKS** in the
information-flow sense — the exact analog of the intrusive list's memory-safety horn-drop:
`secretA.ct_eq(&secretB).expose_bool()` then `if b { … }` **type-checks** with only the vended ops and
leaks a secret *relationship* through control flow. The type forecloses branching *on `Secret`*, but
`expose_bool` returns an **untainted `bool`** you can then branch on — so "declassify once, at the end" is
**etiquette, not type-structure**. Closing it would need a whole-program, **undecidable** non-interference
analysis (a full IFC type system — Jif, FlowCaml). *A concentrated, decidable seam cannot soundly enforce
non-interference* — Horn 2 passes only because the real property is never checked. **The Seam Trilemma
recurs in the new domain, with a sharper bite.**

Second — and this is the generalization — the panel ruled information-flow **out of frame**: a *different
soundness domain* with its **own basis**. So "the cost-basis is closed" refines to **closed *per
domain***. Two attackers independently surfaced **declassification discipline** (which / where /
how-often a secret-derived bit may re-enter control flow — exactly the hole that broke this seam) as a
genuine IFC-domain axis with **no memory-safety analog**, alongside termination and concurrency channels.
The build-and-break *method* transfers — its very first IFC seam reproduced the trilemma and surfaced a
new axis — but the *coordinates* are the new domain's. (Honest correction the panel forced: an earlier
draft called the trust-the-codegen assumption "MMIO-shaped / irreducible"; it is in fact **dischargeable**
— CompCert-CT, Jasmin, and FaCT *formally preserve* constant-time — a contingent artifact of stock
`rustc`/LLVM, a different category than MMIO's permanent axiom.)

**So the deepest form of the result is not "the basis is closed" but "the Seam Trilemma and its cost-basis
are *domain-parameterized*."** Memory safety is one domain — mapped, basis closed, nine axes. Information
flow is another — its own basis (declassification, …), its own sharper trilemma.

### Primitive 14 — capability-gated declassification → a closing seam needs a matching FLOOR (`seam/src/labeled.rs`)

The secret seam established information-flow is a *different domain* — but it *broke*. One question
completes the picture: does the IFC domain have a **closing** seam (its own SPSC / slab), confirming it is
a real domain with a real cost-basis, not just a place where things break? The fourteenth primitive tries
to build one — and its failure is the project's deepest single result.

The secret seam broke because declassification was *etiquette*. The fix is the exact analog of how every
memory-safety seam works — you don't get *"no `unsafe`"*, you get *concentrated, checked `unsafe`*; so
here you don't get *"no declassification"*, you get **capability-gated** declassification: `Secret<T>` has
no read-out except `declassify(self, cap: Cap)`, and `Cap` is **affine** (`!Copy`/`!Clone`, minted only at
the trust boundary by `Cap::issue()`). Now *where/when/how-often* a secret is released is type-structural,
not convention — exactly the gap that broke the secret seam, closed.

And yet the cold panel **broke it too, unanimously (5/5)** — for two reasons that *together* are the
finding. (1) To **compute** on a secret, `map(self, f: FnOnce(T) -> U)` must hand the closure the **bare
plaintext** `T`; a safe closure with captured `&mut` state exfiltrates it — `s.map(move |x| sink.push(x))`
— with **zero caps and no `declassify` site**. The affine cap "guards a door while the wall beside it is
open." Unpatchable without an **effect system** Rust's floor disclaims. (2) The "budget" counts **uses,
not bits**: `Cap::issue()` in a loop mints N caps (statically unbounded — Rice), and one cap can release a
megabyte. What the cap *does* enforce — release *location* — **projects onto the memory-safety affine
axis** (it is one-shot privilege, not a new coordinate); the genuinely-IFC coordinate is the *amount*
(**quantitative information flow** — Smith min-entropy / g-leakage), and *an affine type system cannot
bound bits, only uses*.

So **both information-flow seams break, for one structural reason: Rust's affine/borrow floor carries
*none* of the IFC precondition.** And that is the deepest result of the whole project — not about
information flow specifically, but about the relationship between a floor and a domain:

> **A closing seam exists only when the floor *matches* the domain.** Rust's affine floor is *matched* to
> memory safety — where SPSC closes for *free* (the floor's affinity/NLL carry the precondition) and the
> basis is closed — and *mismatched* to information flow — where *no* seam closes, because affinity tracks
> *resource uses*, while IFC needs *information amount*. To get closing seams in the IFC domain you need a
> *different floor*: a quantitative / effect-tracking type system (Jif, FlowCaml, QIF).

So the shape after fourteen primitives is three layered claims. **The Seam Trilemma is domain-general** —
it recurs in every soundness domain. **The cost-basis is per-domain** — memory safety has its nine closed
axes; information flow has its own (declassification, quantitative leakage). **And which seams close depends
on the floor-domain match** — a floor closes a domain's seams only when it carries that domain's
precondition. But that third claim was still stated as a *binary* — match or mismatch — and a fifteenth
primitive, in a *third* domain, showed it is really a **spectrum**.

### Primitive 15 — type-level lock ordering → the floor-domain match is GRADED (`seam/src/ordered_locks.rs`)

The two information-flow seams both *broke* — a clean negative ("no IFC seam closes on Rust's floor"). The
obvious next question is the **positive** half: pick a *third* domain whose precondition Rust's floor
*does* seem to carry, and watch a closing seam reappear. **Deadlock-freedom** is the ideal test —
a deadlocked program is memory-safe and leaks nothing, so liveness is a genuinely distinct domain, and its
precondition (*acquire locks in a strictly-decreasing global rank order → no cyclic wait*) is **exactly a
partial order**, which Rust's trait resolver carries natively (the sealed-`Lt` mechanism `typed_nat`
already uses). So: give each `OrderedLock<T, L>` a type-level rank `L`; thread an affine `Floor<F>`
capability recording the lowest rank held; gate acquisition `with(floor, …)` on `L: Below<F>` and borrow
the floor `&mut` for the scope so the held set marches strictly downward. A lock-order inversion looks
*unconstructible* — the obligation it needs (`La: Below<Lb>` with `La` outranking `Lb`) has no impl. The
four sound-use tests acquire in-rank and the three `compile_fail` doctests reject the inversions.

And yet the cold panel **broke it (3/5 attackers, one fatal — SEAM-BREAKS [horn 1])**, the *third* seam to
drop a horn — and the break is richer than the positive result it denied, because it breaks along **two
independent dimensions, both forms of "the floor under-carries the precondition."** (1) **Granularity.**
`Floor::root` is an ordinary safe call, hence *re-entrant*: two sibling `root` calls mint two independent
`Floor<TopRank>` tokens, and safe code acquires a *low* rank under one and a *high* rank under the other —
a lock-order inversion in which every `L: Below<F>` still discharges (verified: it compiles and runs).
The discipline enforces descent *within one token lineage*; the precondition is *per-thread-global*.
Affinity on a *value* (the token) is the wrong granularity for a *per-thread* fact, and Rust's floor has
**no ambient per-thread one-shot resource** to mint `root` from — so even the *static* fragment doesn't
close. (2) **Coverage.** Even granting a `root` fix, what closes is *lock-order-inversion freedom*, **not**
deadlock-freedom: the canonical deadlock-free idioms — bank-transfer `min(i,j)` ordering, hand-over-hand
list/tree coupling, address-ordered bucket locks, interrupt ordering — are **inexpressible** (same static
rank + irreflexive `Below` + compile-time-only order). The **dynamic-acyclicity** coordinate (what
`lockdep` checks at runtime) **escapes the static floor exactly as IFC's quantitative coordinate escaped
the affine floor**. Prior art forces this reading: it is **Boyapati–Lee–Rinard (OOPSLA'02) minus their
dynamic tree-order extension**, and **Kobayashi's natural-number levels** — both abandoned pure static
levels *because* the data-dependent fragment forced them off the wall.

So the third claim is not binary but **graded**:

> **Whether a seam closes is GRADED by the floor-domain match.** A floor carries a domain's precondition
> along (at least) two axes — *granularity* (per-value vs per-thread/global) and *coverage*
> (static-vs-dynamic) — and the match can be **full**, **partial**, or **none**. **Memory safety** on the
> **affine** floor: a *full* match (basis closed, nine axes). **Information flow** on the affine floor: *no*
> match (both seams break — affinity tracks uses, not bits). **Liveness/deadlock-freedom** on the
> **static-type-order** floor: a *partial* match — the static, distinct-rank, single-stack fragment is
> carried; the per-thread (granularity) and dynamic (coverage) fragments escape.

(Two corrections the panel pinned, worth keeping: the honest name is **lock-order-inversion freedom**, never
"deadlock-freedom" — condvar/channel/join cycles remain; and the claim that Rust's *affine* floor is what
makes liveness close is **overstated** — the order-carrying floor predates affinity by two decades in BLR's
Java and Kobayashi's π-calculus, neither affine. One panel sub-claim went the other way: the judge said the
inversion doctests surface as `E0277`, but standalone `rustc` emits `E0308` — the recursive `Below` impls
shape `F` from the obligation before unifying the floor argument. The panel adjudicates the verdict; a
checkable diagnostic is settled by the compiler.)

So the shape after fifteen primitives is **four** layered claims, the last *grading* the third. **The Seam
Trilemma is domain-general; the cost-basis is per-domain; which seams close depends on the floor-domain
match; and that match is graded — full / partial / none.** But "graded" still suggested a knob you could
*turn up* — pick a better floor, get a fuller match. A sixteenth primitive tried exactly that, and showed
you can't.

### Primitive 16 — a runtime lock registry → the floor-upgrade is NON-MONOTONE (`seam/src/lock_registry.rs`)

The static fifteenth seam failed on **granularity** (per-token, not per-thread) and **coverage** (static,
not dynamic). The obvious repair is to *upgrade the floor*: replace the compile-time type-level order with
a **runtime registry** — a `thread_local!` held-set of `u64` ranks, checked on every `lock()`, panicking
on a detected inversion. This is `lockdep`-in-miniature, and it *looks* like it fixes both gaps at once.
**Granularity:** `thread_local!` gives exactly one held-set per thread, *un-mintable twice* — the
fifteenth seam's nested-`root` hole has no analog. **Coverage:** ranks are *runtime values*, so two locks
of the same kind take different ranks (`accounts[i].rank = i`), and the canonical deadlock-free idioms the
static floor could not express — bank-transfer `min(i,j)`-before-`max(i,j)`, hand-over-hand coupling —
become ordinary safe code. (The crate proves this: a bank-transfer test in runtime index order *compiles
and runs* where the static seam rejected it.)

And yet the cold panel **broke it (FAILED, horn 1)** — the *fourth* seam to drop a horn — and the break is
the deepest refinement of the whole arc, because it shows **upgrading the floor is non-monotone**. The
upgrade *did* fix coverage, and it *did* pay the price the graded thesis predicted: **#5 bare-metal** (a
`thread_local!` needs OS thread-local storage, so this seam **cannot be `no_std`** — it is gated behind a
`lockdep` cargo feature precisely to make that cost visible in the build graph), **#6 zero-cost** (a
runtime check per acquire — the *slab's* place, in a new domain), and a **modality shift** from static
*prevention* to runtime *detection* (the inversion now compiles and is trapped only on a path that
*executes* — soundness is path-coverage-dependent, the real `lockdep` caveat). **But granularity did not
get fixed — it recurred one rung finer.** The held-set is keyed on the **OS thread**; the unit of
sequential control in async is the **task**. A guard held across `.await` migrates (work-stealing) to a
fresh thread whose held-set is empty, so a *lower* rank is accepted with **no panic** — an executed,
undetected inversion. (I verified the mechanism: a rank held on one thread is invisible to another — the
registry is thread-blind.) A single-threaded executor has the mirror bug — it *false-panics* on two
independent tasks that merely share a thread — and making the guard `!Send` blocks one direction only.

The lesson generalizes past locks:

> **The "granularity" axis of the floor-domain match is a CONTAINMENT-SCOPE LADDER** — *token < thread <
> task < …* — and each ambient floor reaches exactly **one rung**: the affine token (#15, too fine),
> thread-local (#16, matches per-thread but too coarse for tasks). Rust has **no ambient task-local** (async
> is a library, not a language primitive), so no ambient floor carries the per-control-unit precondition.
> **Upgrading the floor is non-monotone:** it can fix one fragment (coverage) while the granularity gap
> *relocates* to a finer grain. You do not buy a partial match up to full by picking a better floor — you
> trade one gap for a subtler one, and spend cost-basis axes (#5, #6) plus a modality downgrade doing it.

Prior art forecloses the design from the other side: this is FreeBSD **WITNESS** / **lockdep** lock-leveling,
and WITNESS, lockdep, and Abseil all **abandoned static ranks** for dynamically-*learned* acquires-before
graphs — because hand-assigned global ranks don't compose, and a single-thread held-set can't even catch a
cross-thread A→B / B→A inversion a graph checker catches. The production answer is a still-higher floor (a
learned runtime graph + cycle detection), which fixes coverage fully but lives *entirely* in the detection
modality. So the arc's true ending is not a gradient with a knob, but a **ladder**: each floor you climb to
carries one more rung of the precondition and reveals the next. *That* is where the constructive arc ends —
not at a wall, not at a border, not on a tunable gradient, but on a **staircase** whose next step is always
named and never last.

### Primitive 17 — a must-run cleanup guard → a NEW verdict mode: *wrong structural logic* (`seam/src/linear.rs`)

The staircase was one direction — *up*, climbing floors within one precondition. The seventeenth primitive
went *sideways*: hold the floor fixed and change the **precondition**. Pick a new liveness obligation —
*"this finalizer MUST run"* — and grade Rust's ambient **affine** floor against it. It is the exact
*inverse* of the tenth seam: `drain` **closed** by making a skipped `Drop` *harmless* (leak-**tolerance**);
this one needs a skipped `Drop` to be *impossible* (leak-**intolerance**) — the same ninth-axis danger from
the opposite side. A `MustRun<F: FnOnce()>` arms a closure that runs on drop; the re-import promise is "bind
it and it runs." (Core-only — back to the default `no_std` build, no feature gate.)

It **broke** — the *fifth* seam to drop a horn, and the break is **compiler-settled**, not a panel judgment
call: `core::mem::forget` is a *safe* function whose unbounded `fn<T>(T)` consumes the guard by one legal
affine move, so `Drop` never fires and the finalizer runs **zero** times (`ManuallyDrop`/`Box::leak`/an
`Rc`-cycle identically). This is the *leakpocalypse* (RFC 1066; the removal of `thread::scoped`, RFC 1084),
made in 2015. "Runs *exactly* once" is a **linear** requirement; Rust's floor is **affine** — *at most*
once — so it structurally cannot forbid the skip. The cold panel confirmed the result across **two
independent rounds**, the breaking attackers included, and that confirmation is the new finding on four
counts:

> - **The grade is a clean NONE** at the type level. The tempting "partial via the `#[must_use]` lint" was
>   *rejected*: the deployed `MustRun` (`scopeguard`) already ships that exact warning as a cosmetic patch
>   over a non-guarantee, so the lint is a **non-type advisory** (silenced by `let _ =`, blind to `forget`),
>   not a floor fragment. Stipulating "`must_use` ∈ floor" only *manufactures* a false partial.
> - **The match unit is the PRECONDITION, not the DOMAIN.** The same liveness domain grades
>   deadlock-freedom *partial* (#15/#16) and must-run-cleanup *none*. So Part 9's "domain-parameterized"
>   result sharpens: it is *preconditions*, not whole domains, that the floor matches or misses.
> - **N is unchanged.** This is the *enforcement* face of the ninth axis (leak-freedom); `drain` was the
>   *tolerance* face. No new coordinate — a populated off-diagonal, not an extension.
> - **A NEW verdict mode: *wrong structural logic*, not domain-mismatch.** Closing the gap while keeping
>   `forget` safe needs either an *undecidable* finalizer-reachability pass (`Rc`-cycle escape ≅ halting) or
>   a *viral `Leak`/linear bound* — every sound **decidable** enforcement collapses to the latter. So *no
>   decidable boundary price* buys soundness back: the floor is the wrong **structural logic** (affine vs
>   linear), distinct from the Phase-13 *wrong domain*. Sharpened by the prior-art attacker: it is not a
>   binary affine→linear swap but Boats' **scoped-task trilemma**, of which Rust *deliberately rejected* the
>   linear horn.

And one meta-lesson the *process* taught, worth more than a clean pass: **a seam designed to break cannot be
given a coherent "usable" spec — the incoherence IS the break.** Both rounds FAILED not on the break (which
is the designed result) but on *spec coherence*: the commit-or-rollback idiom I used to motivate it needs a
finalizer that must *not* run on success, contradicting the must-run precondition; and the cancel I added to
fix that (`disarm`) hands back the *closure*, never the protected *value* — so it can't host
commit-or-rollback after all. The residual incoherence is not noise; it is positive evidence for the
verdict. A guard that cannot compel its finalizer cannot be honestly dressed as a closing interface, and the
gauntlet keeps finding the seam wherever you paper over it.

### Primitive 18 — a session-typed channel → the same gap can be a *cost*, not a break (`seam/src/session.rs`)

The seventeenth seam broke on the linearity gap. The eighteenth asks a question the break makes irresistible:
is "you can `mem::forget` it" *always* fatal? Take a **session-typed channel** — a `Chan<P>` whose type
parameter `P` is the *remaining protocol* (markers `Tx`/`Rx`/`End`); each operation consumes the handle and
returns one at the successor state. Its precondition is a **composite**: *safety* (operations in protocol
order) **and** *completion* (the protocol runs to `End`). This is the crucible for the two prior liveness
findings — #15 showed affine **typestate carries static order**, #17 showed the floor **cannot compel
completion** — so it asks whether the floor matches a composite precondition **component by component**.

It **held** — SEAM-COSTS, the *first* out-of-memory-safety probe to survive rather than break. The panel
confirmed the components are genuinely **orthogonal** (the soundness attacker's sharpest move — can
`mem::forget` release the `&'a mut Transport` early to start a *second*, conflicting session? — fails,
because *leaking a value holding a `&'a mut` does not shorten `'a`*), so the completion gap cannot infect
safety:

> - **Safety (order) CLOSES** via affine typestate: `send`/`recv`/`close` exist *only* at the matching state,
>   so an out-of-order step names a method that does not exist (`E0599`), and — unlike #15 — there is **no
>   `root` to re-enter**, so the granularity hole has no analog. A sound, concentrated sub-seam.
> - **Completion (progress) only COSTS — horn 3, not horn 1.** It is the *same* `mem::forget` gap as #17, but
>   here it is one undischargeable *"don't abandon"* obligation that recurs at every `?`/panic/early-return —
>   a **concentration** cost, not a soundness break, because the hazard is liveness and completion is *one
>   component beside a sound safety core*, not the whole promise.

That contrast is the finding: **the same linearity gap is a horn-1 *break* (#17) or a horn-3 *cost* (#18)
depending on how the re-import rule packages it** — whether abandoned-completion is the entire soundness
claim, or a recoverable obligation sitting beside an independently-sound core. "Which horn a danger
threatens" is not a property of the danger; it is a design choice. (The panel deflated three over-claims I
made, all accepted: this `core`/no-`unsafe` witness has no duality — a single-handle typestate *walk*, not a
binary session (E0499 forecloses two endpoints), so abandonment is *self*-abandonment; "usable" holds only
for *infallible straight-line* protocols, because a fallible transport forces `?`, which *is* the
abandonment; and the order-closes/completion-breaks split is documented folklore — Jespersen–Munksgaard–Larsen
2015, RFC 1066, ECOOP 2022 *"Stay Safe Under Panic"* — so the contribution is *locating* a known split on the
lattice, not discovering it. The seam adds **no new verdict mode**: it *re-packages* an existing one.)

### Primitive 19 — a scope-restore guard → the packaging dial is real but *bounded* (`seam/src/restore.rs`)

Primitive 18 *noticed* that the same `mem::forget` gap breaks #17 but only costs #18 — but #17 and #18 were
different primitives, so the horn-move could be incidental. The nineteenth turns the observation into a
**controlled experiment**: take a **value-owning scope-restore guard** (the real `scopeguard::guard(value,
f)` + `into_inner` — the shape #17's `MustRun` *deliberately declined* to host), carry #17's *identical*
un-compellable-`Drop` danger (`mem::forget` skips the restore closure, bit-for-bit the same gap), and wrap it
in #18's *structure*: a sound affine-ownership **core** (the guard owns and vends scoped `DerefMut` access to
a `T`, `Option::take` so the value is committed-xor-restored exactly once — no `unsafe`, no UAF) beside which
the restore is a mere **side-obligation**. The prediction: the gap that *broke* #17's horn 1 now only
*costs* horn 3.

It **held** — SEAM-COSTS [horn 3] (cold panel r26, survived). Three attackers conceded the memory-safety
core is sound (no safe double-free/alias/double-consume; trivially decidable), and the predicted horn-3 cost
is right: the skipped restore strands a *logical invariant* (a lost lease, a stale global), recurring at
every `?`/panic/return — never UB. **The same `mem::forget` danger that broke #17 only costs here.** But the
panel **bounded** the thesis, and the bound is the real lesson:

> - **The move is *confounded*, not single-variable.** "Only the packaging changed" is false: to package the
>   danger as a side-obligation, #19 *also* introduces the value-ownership core #17 lacked — so the
>   value-ownership model co-varies with the packaging. (This is structural, not sloppy: a side-obligation
>   needs a sound core to sit *beside*.) A better-controlled probe than the incidental #17-vs-#18 pair, but
>   not a clean manipulation.
> - **The horn is *forceable* — partly danger-intrinsic.** The *same* skippable-`Drop` danger in a
>   *borrowed-slot / `take_mut`* packaging forces **horn-1 UB** (a mid-restore panic double-drops the
>   borrowed slot — the ninth-axis `take` danger), and `Pin`'s drop guarantee and the post-leakpocalypse
>   `thread::scope` restructure exhibit horns that **cannot** be repackaged away. So *which* horn depends on
>   **where safety rests**.

The refined finding: **the packaging dial is real but bounded.** Which horn a danger threatens is a design
choice *within a structural envelope* set by whether soundness can be **decoupled** from the un-compellable
closure — decouplable (an *owned* value → side-obligation) ⇒ the danger **costs**; not decouplable (a
*borrowed* value across unwind, `Pin`'s load-bearing address, `thread::scope`'s borrowed frames) ⇒ the danger
**forces** horn 1. Phase Q said the horn moves; the nineteenth says it moves *within a fence*, and names the
fence. (And it is not novel: `JoinGuard` broke horn-1 and `scopeguard` cost horn-3 already in 2015, RFC 1066;
the contribution is the *method* — a deliberately controlled probe — and *locating* the dial. No new verdict
mode.)

### Primitive 20 — the borrowed-slot take, forcing the horn → the fence is real (`seam/src/reseat.rs`)

The nineteenth named a fence but could not *test* it cleanly: its own verdict flagged the move as
**confounded** — #17→#19 co-varied *two* things, the packaging *and* the value-ownership model. The twentieth
runs the single-variable experiment the confound forbade. Take a **take-and-reseat guard over a borrowed
slot** (`take_mut` in guard form: `ptr::read` the value out of `&mut T`, lend it through `DerefMut`,
`ptr::write` it back on `Drop`) and keep #19's packaging **byte-for-byte identical** — a guard with
`Deref`/`DerefMut`/`into_inner`/Drop-restore. The *only* thing that changes from #19 is the decoupling
property: an **owned** value becomes a **borrowed slot**.

Strikingly, the guard form is **unwind-safe** (Drop reseats *during* unwinding — no abort-bomb, unlike the
eager ninth seam `take`) and **forget-safe** (`mem::forget` leaks the mutation but the slot keeps valid
original bits — a *leak*, not a double-free). The break is **isolated to `into_inner`**: the API that was
*sound* for #19's owned value (one owner) is a **double-free** over a borrowed slot, because it hands the
value out while the slot keeps the `ptr::read` duplicate — two owners.

It **broke** — SEAM-BREAKS [horn 1], FAILED (cold panel r27), the **sixth** broken seam. The soundness
attacker landed a 4-line *safe* witness — `let mut s = String::from("hello"); let mut g = Reseat::new(&mut s);
g.push_str(" world"); g.into_inner()` — where `into_inner` returns the owned `String` while the slot keeps the
duplicate (`E0509` forces the `mem::forget(self)` that suppresses the reseat), so both own overlapping heap and
**double-free, with no `unsafe` in user code**. The panel ruled two things:

> - **The single-variable horn flip is *clean*** (all five attackers). #19 (owned + `into_inner`) *cost* horn
>   3; this (borrowed + `into_inner`) *breaks* horn 1, packaging byte-identical. Where #17→#19 co-varied two
>   variables, #19→#20 co-varies **none** — so Phase R's confound is **repaired**, and the cost-3 → break-1
>   flip is attributable to the **decoupling property alone**.
> - **The fence is *forced*, not movable.** My spec hedged — maybe the `into_inner`-less subset holds, pushing
>   the fence inward. The panel **rejected** that: the sound subset is **degenerate**, deleting the by-value
>   move-out that *defines* `take_mut` (it is strictly dominated by plain `&mut T`). So *every usable*
>   packaging of the borrowed-slot take breaks horn 1; the fence sits exactly at the **owned-vs-borrowed**
>   line the nineteenth drew.

So the packaging trilogy closes with one law: **the horn a danger threatens is movable iff the value-ownership
model lets soundness *decouple* from the un-compellable closure.** An owned value can (#19 → cost); a borrowed
slot cannot, for any usable packaging (#20 → forced break). Phase Q: the horn moves. Phase R: within a fence
(but confounded). Phase S: the fence is real, *forced*, and sits at owned-vs-borrowed. (Pre-registered
deflation: no novel mechanism — `take_mut` + `scopeguard`-`into_inner` + RFC 1066 reassembled; the
contribution is the clean controlled pair and *locating* the fence. No new verdict mode.)

### Primitive 21 — a state-based CRDT merge, the fourth-domain probe → the domain *deflated* (`seam/src/crdt.rs`)

The packaging trilogy closed, so the twenty-first turns *away* from packaging and aims at a **fourth soundness
domain**: distributed/replicated **convergence** (eventual consistency), after memory-safety (full match),
information-flow (mismatch), and liveness (partial). Take a state-based CRDT — a `Replica<T: JoinSemilattice>`
whose `merge` applies the semilattice `join` — with a blessed `GCounter` (element-wise `max`) as the sound
baseline. The danger is **replica divergence** (two replicas that observed the same updates disagree): neither
UB, nor a leak, nor a deadlock, nor a timing channel, and — like the constant-time `secret` seam — **no
`unsafe`**. Convergence holds iff `join` is commutative, associative, and idempotent.

The supply-side break is real and conceded (horn-classifier **SEAM-BREAKS [floor-mismatch]**, cold panel
r28): `T: JoinSemilattice` is a **syntactic proxy, not a witness** — the bound checks only the method
*signature*, so a safe `impl` that adds instead of taking the max type-checks and **double-counts on
at-least-once redelivery**, diverging replicas with no `unsafe` in sight (closing it needs a law-verification
pass that is undecidable by Rice). But the panel scored it a **TRADEOFF survival** — the break was
pre-registered — and the headline is the **deflation**: the candidate fourth domain *collapsed*. Three
corrections, all accepted:

> - **The blessed-CRDT TCB rescue is *not* MMIO-shaped.** Unlike MMIO's finite, foundation-owned registers,
>   the **lattice space is open-ended**, so the law-trust **recurs per user-defined CRDT** instead of
>   amortizing into one foundation axiom — every real use falls back to supply-your-own-`join`.
> - **"Dischargeable" holds only at the *model* level.** Liquid-Haskell/Coq *can* verify the semilattice laws,
>   but about a mathematical model; **no Rust pass checks model-to-shipped-code**, so the trust stays
>   irreducible at the compilation boundary (subtler than `secret`'s constant-time, which CompCert-CT/Jasmin
>   *do* preserve to shipped code).
> - **The decisive deflation: it is the *generic* unchecked-typeclass-law gap.** Rust checks the *signatures*
>   of `Ord`, `Eq`/`Hash`, `Monoid`, and `JoinSemilattice` alike — but **none of their algebraic laws**. So
>   "convergence" is not a fourth soundness domain parallel to information-flow; it **projects onto
>   `restore`'s application-correctness basis** (a logical property the type system never promised).

So the fourth-domain probe found **no clean fourth domain here**: the soundness-domain map stays at **three**
(memory safety / information flow / liveness) plus the **application-correctness** basis, into which
convergence folds. Floor-relatively the supply-side break stands (the floor does not carry the semilattice
laws — the IFC-shaped mismatch), but the *structural* result is a deflation, not a new domain. N stays **nine**
verdict modes; the FAILED broken-seam tally stays **six** (this survived). A probe that earned its keep by
collapsing.

### Primitive 22 — a cost-typed computation, the *second* fourth-domain probe → FAILED (`seam/src/wcet.rs`)

Convergence deflated, so the twenty-second tries a **different** fourth-domain candidate: **real-time /
worst-case execution time (WCET)** — cleaner, because a cycle budget is a *quantitative resource bound*, not
an algebraic law. Take a `Timed<C, T>` — a value produced by a computation whose cost is the type-level
numeral `C`, in a "one unit per `step`" model — and reuse the *exact* sealed `Nat`/`Lt` machinery of the
eighth seam (`typed_nat`). `pure` is cost 0, each `step` charges one unit, and `within::<D>` discharges a
deadline, returning the value only if `C ≤ D` (a compile-time proof). The danger is a **deadline miss**:
neither UB, leak, deadlock, timing channel, nor divergence; and — like `crdt`/`secret` — **no `unsafe`**.

It **FAILED** (cold panel r29, **SEAM-BREAKS [horn 1]**) — all five attackers ruled BREAKS, from five distinct
angles. Unlike `crdt` (which deflated and *survived*), this one **failed**, because the §4 re-import rule
**over-claimed**: it promised `within` returns the value "only when the computation meets its deadline," but
the type tracks the **op-count**, not the time.

> - **Soundness (fatal):** `step` advances `C := S<C>` *decoupled from its closure*, so a million-iteration
>   loop is cost `1` and `within::<1>` returns it. `C ≤ D` proves nothing about real cycles.
> - **Ergonomics — Horn 3 is *false*:** an `if` whose arms do different work yields **non-unifying**
>   `Timed<S<C>>` vs `Timed<S<S<S<C>>>>`, forcing manual step-padding. The cost index is *heavy* bookkeeping.
> - **Implementation:** the unary Peano cost (the `typed_nat` machinery) is **O(C)**, so honest per-cycle
>   stepping is *uncompilable* at real 10³–10⁷-cycle budgets — and the only compilable path, bundling work
>   into one `step`, *is* the soundness break.
> - **Expressiveness:** no type-level *addition* — computations cannot be sequenced and summed, so every
>   program is one flat lying step-chain.
> - **Prior-art:** the primitive is **Danielsson's 2008 cost monad**, and "op-count ≠ cycles" is WCET's
>   *founding premise*, not a finding. Every precedent (RaML, gas, aiT) got soundness only by dropping the
>   physical-cycle claim or installing a defining machine. **Pick a lane:** abstract cost *xor* physical
>   deadline (the latter needs a machine the floor lacks).

So WCET does **not** open a fourth domain either: it **deflates** multiply — the static cost monad onto
Danielsson 2008 / `typed_nat` (#8, which does not even *scale*); the *physical-deadline* claim onto an
`mmio`-shaped axiom (#2) the seam **never installs** (so it is *worse* than MMIO — a fiction *and* absent);
the data-dependent-loop gap onto liveness's **partial-match**. The op-count is an **unsound** proxy —
*sharper* than `secret`, whose proxy foreclosed *some* leaks. The meta-finding for the record: **two**
fourth-domain candidates have now been probed — `crdt` (deflated + survived) and `wcet` (deflated + FAILED) —
and **neither opens a fourth domain**, by *different* mechanisms. The three confirmed domains (memory safety /
information flow / liveness) look increasingly **special**. N stays **nine** verdict modes; the FAILED
broken-seam tally rises to **seven** (this one failed).

### Primitive 23 — a real binary session, deepening liveness → HELD (`seam/src/binary_session.rs`)

Two fourth-domain candidates having deflated, the twenty-third stops bidding for a fourth domain and
**deepens the third** (liveness). It removes the most-cited limitation of the eighteenth seam: the r25 panel's
deflation #1 of `session` was *"not a binary session — a single-handle typestate walk"* — one `&mut Transport`
cannot split into two concurrently-live dual endpoints (`E0499`), so #18's motivating *"a peer waits forever"*
hazard was inexpressible (only *self*-abandonment of one handle). So build the real thing: `session::<P>()`
returns `(Chan<P>, Chan<P::Dual>)` — two affine endpoints, **both live**, sharing the wire through
`Rc<RefCell<Transport>>` (the `alloc` + interior mutability the duality costs). A `HasDual` trait maps
`Tx<C> ↔ Rx<C::Dual>`, `End ↔ End`. No `unsafe`: the danger is liveness, and `Rc`/`RefCell` mediate sharing.

It **HELD** (cold panel r30, **SEAM-COSTS [horn 3], TRADEOFF**) — every attacker granted Horn 1 / no-UB (the
worst case is a starved, memory-safe handle). Order **closes** (typestate, as #18), *per-session* relative
progress **closes** (duality), decidability holds (`HasDual` is Θ(depth), no SMT), and completion **costs horn
3**, now peer-observable. So **#18's component-wise grading survives** removing the single-handle limitation.
But three "BREAKS" attackers converged on one thing — the *framing* overclaimed — and the five corrections all
landed:

> - **Relative progress closes only *per session* (the sharpest).** Two interleaved `session()` calls
>   type-check a **mutual-wait deadlock** (Kobayashi/Padovani): each session is individually dual, the
>   composition has no terminating schedule. So the flat "mutual-wait deadlock is a type error you cannot
>   write" is *false* — narrowed to the two endpoints of *one* session, where it is near-trivial (two endpoints
>   can't both be `Rx`). **Global deadlock-freedom is out of reach of binary session types — it needs
>   multiparty/Kobayashi typing.**
> - **The single-threaded witness makes "peer waits forever" self-abandonment-flavored.** One thread owns
>   *both* handles, so only the *static* two-live-endpoint limitation is lifted — not `session_types` (2015),
>   which had real two-thread concurrency a decade earlier. The contribution is *locating* the binary session
>   on the lattice, not out-realing prior art.
> - **The forced-fallible `recv`'s `Err` is *non-actionable* — the decisive sharpening.** `recv` must be
>   fallible (the value comes from a peer who may never send), but `mem::forget` **leaks the `Rc`** (skips
>   `Drop`, strong count not decremented), so a starved dual cannot tell "not-yet" from "abandoned"
>   (`strong_count` still sees the peer; no closed-flag is set). `Err` conflates *not ready* with *never*, and
>   the design forecloses the fix — an *actionable* signal (`mpsc`'s disconnect `Err`) needs `Drop` to fire.
>   The **leakpocalypse double-bind**: the *same* gap that costs completion *also* denies an actionable
>   abandonment signal. Honest at the type level (vs #18's hidden `.expect`), but actionability is absent.
> - **Only straight-line `i32` protocols** — no `offer`/`choose`, no **delegation** (channel-over-channel, *the*
>   feature distinguishing session types from coupled typestate), no recursion. Without delegation it is "#18
>   doubled."
> - **Citation fix:** ECOOP 2022 "Stay Safe Under Panic" is **Lagaillardie–Neykova–Yoshida** (corrected in #18
>   too).

So the probe **earned its keep by refuting its own headline** while confirming the grading. The genuinely new
content, once "real binary session" deflates to "#18 with a duplicated handle and a `RefCell` between the
copies," is two sharpenings of the liveness map: binary duality gives only **per-session** progress, and the
**leakpocalypse double-bind** deepens the #17/#18 linearity gap. N stays **nine** modes; liveness stays the
**third** domain; the FAILED broken-seam tally **stays seven** (this survived).

### Primitive 24 — delegation (channel-over-channel) → SOUND, but the deliverable is REJECTED (`seam/src/delegation.rs`)

The twenty-third's expressiveness attacker named **delegation** — sending a channel endpoint as a payload over
another channel — as *the* feature distinguishing session types from coupled typestate machines, the one #23's
straight-line `i32` witness lacked. So build it: on #23's `Rc<RefCell<Transport>>` endpoints, the wire's slot
now carries *either* an `i32` *or* a whole delegated endpoint; `TxChan<Q, C>::send_chan(c: Chan<Q>)` hands off a
live session, `RxChan<Q, C>::recv_chan` receives it (re-attaching `Q`, which duality pins). `HasDual` passes
`Q` through unchanged. Delegation is inherently **three-party**: A delegates its session-with-C to B, so B now
drives that session with C. No `unsafe`.

The pre-registered hypothesis was a *closed-basis confirmation*: delegation adds no new coordinate; its hazards
are #23's completion gap (now three-party) plus a leak (abandoning a carrier with a delegated channel in flight
strands the payload `Rc`), **projecting onto the leak-freedom axis (#9/#10)** — the Phase-K pattern, one domain
over. The cold panel (r31) **rejected that deliverable** — and this is the more instructive outcome. The
*mechanism* is sound (horn-classifier **SEAM-COSTS [horn 3]**; the soundness attacker confirmed no wrong-`Q`
re-typing — duality-by-construction + generic invariance block any Gay–Hole subtyping smuggle; the worst case is
a memory-safe leaked/starved session, not UB). But the **judge ruled FAILED on the §6 deliverable** — the
method's **rejection mode**, firing for the *second* time (after `typed_nat` #8, also sound + deliverable-
rejected). The refutation came *from my own cited lineage*:

> - **Delegation's signature hazard is *progress / deadlock-freedom*, a coordinate distinct from completion AND
>   leak-freedom — not a leak.** The three-party hazard (B `mem::forget`s the received `Chan<Q>`, starving C) is,
>   in the *exact* words of the `session_types` crate I cited (Jespersen–Munksgaard–Larsen, WGP 2015), that an
>   affine session "*means an agent can drop the connection anytime without finishing the protocol, which leads
>   to **deadlock at the other end**.*" That is **progress** — a recognized *third* property. An entire
>   literature exists *because delegation is the feature that breaks first-order binary sessions' automatic
>   deadlock-freedom*: Coppo–Dezani–Yoshida–Padovani (MSCS 2016, "*the communication topology changes because of
>   delegation*"), Kobayashi-style deadlock/lock-freedom typing, deadlock-free GV (Lindley–Morris; Padovani,
>   "Progress as Compositional Lock-Freedom"; Multiparty GV, PACMPL 2022; Rusty Variation, 2019).
> - **The category error.** I saw the `Rc` hazard's *memory-leak face* and filed it under leak-freedom (#9/#10),
>   missing that the signature danger is a **progress** phenomenon that merely *also* leaks under `Rc`. The new
>   danger does not land on an existing coordinate — it is a *distinct* one (progress), projected onto the wrong
>   axis because the witness cannot see its true one.
> - **The scoping is what hid it.** Progress/deadlock manifests only under interleaving; a single-threaded
>   synchronous witness *definitionally cannot deadlock*, so "no new coordinate" was an **artifact of the
>   witness**. My one real defense — the symmetric two-session delegation **cycle is type-obstructed** (an
>   inexpressible infinite type) — rescues *behavior*, not *classification*.
> - **Recorded tradeoffs.** No mid-protocol abort verb and `Result<_, Self>` bans `?`, so on an I/O error the
>   completion obligation is undischargeable *by the programmer*, not merely the type system; `HasDual`
>   well-formedness is Θ(depth²) (the involution normalizes `dual(dual(carrier))`), not the spec's Θ(depth); and
>   the finite-type, `!Send`, single-thread witness cannot express the recursive accept-loop server that is
>   delegation's canonical use case.

So the probe **earned its keep by being instructively wrong**: I predicted a closed-basis confirmation, and the
method refuted it *with my own citations*, opening the **progress / deadlock-freedom** coordinate — distinct
from completion (linearity) and leak-freedom. The liveness domain thus has *progress* as a third coordinate: the
lock-ordering seams (#15/#16) were its *lock* face, delegation its *session* face. The sound order +
don't-use-after-delegate core (the affine *move* carries the one linearity delegation needs) still **closes**
(`delegation_handoff_core_closes`, like `typed_nat`); N stays **nine** modes; the FAILED *broken-seam* tally
**stays seven** (this is a deliverable rejection, not a soundness break).

### Primitive 25 — multiparty (n-party progress) → SOUND, but the deliverable is REJECTED (`seam/src/multiparty.rs`)

#23's deflation #1 conceded the gap outright — "*two interleaved `session()` calls type-check a mutual-wait
deadlock; global deadlock-freedom is out of reach of binary session types — it needs multiparty/Kobayashi
typing*" — and #24 named the coordinate (**progress / deadlock-freedom**). So build the multiparty thing, and
*from #23's own binary edges* — deliberately, because the claim under test is that **multiparty progress is not
reducible to the binary duality of the edges it is built from**. A `RecvThenSend` role typestate enforces
recv-before-send (`recv` yields the `JustSend` that alone can send). Two constructors mint role bundles from the
**identical** per-edge floor: `relay()` wires a coherent `A→B→C` (a source role exists, so it runs to
completion), and `ring()` wires an incoherent `A→B→C→A` where **every** role is `RecvThenSend` — so there is
**no source**, and the composition has no terminating schedule. Crucially, **`ring()` type-checks**: every edge
is perfectly dual, every role-local type is a well-formed affine chain, yet the system is globally deadlocked.
#23's *asserted* interleaved mutual-wait deadlock is now a *compiled* three-party artifact. No `unsafe`.

The pre-registered deliverable staked the **concentration leg (horn 3)**: that a *global* property like
deadlock-freedom cannot be concentrated in a per-call re-import rule — only **whole-protocol**, in a *generative*
projection of a coherent global type to per-role local types (Honda–Yoshida–Carbone; `mpstthree`) — and asked
the panel to confirm that *no* per-role discipline could reject the ring without a global type. The cold panel
(r32) **rejected that deliverable** — the **rejection mode's third firing** (after `typed_nat` #8 and
`delegation` #24), and, as with #24, the refutation came *from my own cited lineage*. The *mechanism* is sound
(horn-classifier **SEAM-COSTS [horn 3]**; the worst case is a memory-safe recv-`Err` hang, not UB; structural
`HasDual` keeps it decidable), but the **judge ruled FAILED on the §6 deliverable**:

> - **It is a false dichotomy — there are three rungs, not two.** The middle rung I omitted is **per-channel
>   priority / obligation typing** (Kobayashi linear-π; Padovani; **Dardha–Gay, FoSSaCS 2018**; Kokke–Dardha):
>   it is *per-channel, compositional, non-generative, and uses no global type*, yet rejects the source-less
>   ring *exactly* via `prio(recv) < prio(send)` chaining to a contradictory `p < p` around the cycle, while
>   admitting the relay. The `ring`/`relay` pair is that tradition's own running example. Worse, I had
>   **mis-filed Kobayashi in §5 as *supporting* the whole-protocol thesis when he is its *counterexample***,
>   collapsing rung (b) into rung (c) to manufacture the result. The honest taxonomy: **(a)** the per-edge
>   `HasDual` floor [does not carry it]; **(b)** per-channel priority typing + a whole-program acyclicity check
>   [no global type]; **(c)** a generative MPST projection.
> - **What survives is the weaker, true claim.** Global progress is **not concentrable per-call** and needs
>   strictly more than the per-edge floor (`ring()` proves the floor accepts a deadlock) — but it is a
>   **whole-program** obligation, *not* necessarily a *generative* one. I over-reached from the true weaker
>   claim to a false unique-home claim.

Recorded tradeoffs: the horn-3 "concentration confirmed" rests on a degenerate witness (single-shot, fixed
`N = 3`, single-threaded), and the "deadlock" is itself degenerate — not a concurrent cyclic-wait *hang* but
"no source → every `recv` is `Err`," a static starvation the one thread observes at once; recursion / dynamic-`N`
/ async erode the concentration. The sound role-local order + linearity core still **closes**
(`multiparty_role_order_core_closes`, like `typed_nat`/`delegation`); liveness stays the third domain, and
**progress** stays its third coordinate — the lock seams #15/#16 are its *lock* face, delegation its *session*
face, multiparty its *global-topology* face. N stays **nine** modes; the FAILED *broken-seam* tally **stays
seven** (a deliverable rejection, not a soundness break). The probe earned its keep by being instructively
**wrong** a third time — and each time, the literature I myself cited is what corrected the map.

### Primitive 26 — priority (per-channel priority typing) → SEAM-BREAKS [horn 1], the eighth broken seam (`seam/src/priority.rs`)

#25's rejection named the omitted middle rung — **per-channel priority / obligation typing** (Kobayashi linear-π;
Padovani; **Dardha–Gay, FoSSaCS 2018**; Kokke–Dardha): per-channel, compositional, non-generative, *no global
type*, yet it rejects the source-less ring via `prio(recv) < prio(send)` chaining to a contradictory `p < p`
around the cycle. So build *that* — the rung-(b) floor — and build it **to confirm #25's weaker thesis humbly**:
that the per-channel bound carries nothing global, that deadlock-freedom still rests on something more than the
edge floor. Over #23's binary edges and #8's sealed `typed_nat::Lt` (Peano `<` — no reflexive `Lt<N>` instance
exists), a forwarder `Relay<Pi, Po>` recv-then-sends under the bound `Pi: Lt<Po>`. The pre-registered claim
(§4.3): the deadlocking ring `A→B→C→A` becomes *untypable*, because `p_CA < p_AB < p_BC < p_CA` chains by
transitivity to `p: Lt<p>`, for which no instance exists. I staked **SEAM-COSTS [horn 3]** — a humble
confirmation that priority typing closes the gap the per-edge floor cannot, at the cost of a per-channel
obligation discipline. No `unsafe`.

The cold panel (r33) moved it to **SEAM-BREAKS [horn 1]** — the **eighth broken seam**, and the *first soundness
break since `wcet` #22* (the broken-seam tally rises 7→8; the prior three primitives were sound deliverable-
rejections). The §4.3 untypability claim is **false**, and the soundness attacker proved it with a passing test
(`public_role_builds_a_typed_deadlocking_ring`):

> - **The collapse secretly assumes edge-priority *consistency*.** The chain `p_CA < p_AB < p_BC < p_CA` forms
>   *only* if the `Nat` A sends on edge AB is the *same* `Nat` B receives on AB — one shared priority per edge.
>   But Rust does **no transitive reasoning**: it checks each `Relay<Pi, Po>`'s bound `Pi: Lt<Po>` in *isolation*,
>   per endpoint, never relating one relay's `Po` to the next's `Pi`. The contradiction `p: Lt<p>` materializes
>   only when a single shared `Nat` threads each edge — which **only the `relay()` constructor enforces**.
> - **The public constructor is the hole.** `role::<Pi, Po>` is public, so three `role::<N1, N2>` relays — each
>   discharging the trivial bound `1 < 2` locally — **wire into the ring**: it *compiles*, with no `unsafe`, and
>   **deadlocks**. Edge-priority consistency is the **constructor's invariant, a mini-projection — not a
>   per-endpoint type fact** the floor can range over.
> - **The sting: I described this exact flaw in §4 and shipped `role()` public anyway.** §4 says in plain words
>   that "edge consistency is the constructor's invariant, not a per-endpoint type fact"; the soundness attacker
>   **quoted my own sentence** back and pointed at the public `role()`. The seam breaks on precisely the seam I
>   documented and left open.

So the probe **confirmed #25's weaker thesis by *breaking*, not costing**: the per-channel `Lt` bound carries
*nothing global* — it never names the dual endpoint — so deadlock-freedom rests on a **trusted edge-consistent
constructor** (`relay()`), a mini-projection by another name. Hide `role()` and mint only through an
edge-consistent constructor and the seam *salvages* to **SEAM-COSTS [horn 3]** (the humble result I predicted) —
but as *shipped*, with the constructor public, it is a **soundness break**. The unification is the sharp part:
this is the **same break as `ordered_locks` #15/#16**, one soundness sub-domain over — the type-level order
carries the *local* bound (`prio(recv) < prio(send)`, like a per-token rank), but **whole-program acyclicity
escapes** it (the cycle the per-edge floor cannot see, like the per-thread inversion). Progress's **lock face**
(#15/#16) and its **session face** (here) break the *same way*: the *wrong-granularity* gap — a local order
typed, a global acyclicity un-typed. The sound role-local recv-then-send discipline still factors
(`priority_typed_discipline_factors`, a `..._factors` — the kernel models neither liveness nor the missing
transitive reasoning); liveness is the **third domain**, **progress** its third coordinate (lock face #15/#16,
session face #24, global-topology face #25, and now the *priority* face that *breaks*). N stays **nine** modes;
the FAILED *broken-seam* tally **rises to eight** (a real soundness break, the first since `wcet`).

### Primitive 27 — projected (deadlock-freedom by construction → schedulability by construction) → SEAM-COSTS [horn 3], SOUND but deliverable REJECTED (`seam/src/projected.rs`)

#26's break named its own salvage in the same breath: *hide `role()`, mint only through an edge-consistent
constructor, recovering the humble SEAM-COSTS [horn 3] result the primitive was built to confirm.* So build
exactly that. **SEAL** `worker()` — make the per-role constructor module-private — and vend only two *trusted
builders*, `relay` (a three-party `A→B→C`) and `relay4` (four-party), each threading a single strictly-increasing
`Nat` *per edge* so the priorities are edge-consistent **by construction**. The #26 deadlocking ring is now
**unconstructible**, and the soundness attacker (r34) conceded it on two independent grounds:

> - **Access.** There is no public `worker()`, so a consumer cannot hand-wire three relays with mismatched
>   per-edge priorities — the only mints are `relay`/`relay4`, which thread one shared `Nat` per edge.
> - **Structure.** Even granting access, the ring cannot *type*: the closing edge back to the source needs a
>   *decreasing* `Lt` (`p_CA < p_AB`) to shut the monotone chain `p_AB < p_BC < p_CA`, and `typed_nat::Lt`
>   (Peano `<`) has no such instance. The cycle is doubly foreclosed.

Horn 1 is conceded — **the salvage works**, exactly as #26 predicted, and the horn-classifier agreed with my
staked **SEAM-COSTS [horn 3]**. But the judge (r34) **FAILED the §6 deliverable** — the **rejection mode's
fourth firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25; #26 was the break between). The §4
over-claim is the familiar one, and the panel quoted it back:

> - **"Deadlock-freedom by construction" is really "*schedulability* by construction."** The trusted builder
>   proves a deadlock-free *schedule exists* — there is a static `Nat` witness ordering the edges acyclically.
>   But affine Rust never *forces the runtime to follow it*: the consumer may run the roles in any order,
>   interleave them, or drop one (`mem::forget` again). A **run-all / in-order / no-drop progress residue**
>   stays on the *consumer*, undischarged by the constructor. The honest framing is "schedulability by
>   construction, *given every role executes*."
> - **The concentration is only *partial*.** §4 claimed "the consumer carries nothing global." It carries
>   *topology* nothing — the constructor really does discharge "no cycle," the #26 fix. But
>   *completion/execution* never left the consumer; it is the #17/#18/#23 residue wearing a multiparty hat.
> - **The imported giveaway.** Scribble / `mpstthree` endpoints are deadlock-free *only assuming every
>   participant executes* — I imported that guarantee (a "single-threaded witness" that runs every minted role
>   to completion) while simultaneously claiming the consumer carries nothing. The witness *is* the assumption.
> - **A secondary over-claim, conceded.** I priced the seal's expressiveness cost too high: the acyclic
>   generality the public `worker()` exposed is recoverable through *one* generic combinator (`mpstthree`'s own
>   move), so sealing it costs far less than §4 implied.

So the probe **confirmed the salvage soundly and was rejected on its headline**: the **progress coordinate
decomposes**. Deadlock-freedom / *topology* — "no cycle exists" — is carried by the trusted constructor (the
#26 fix, genuinely concentrated at the mint). *Completion / execution* — "every minted role actually runs, in
order, to the end" — never leaves the consumer (the #17 linear obligation, the #18 per-`?` residue, the #23
per-session completion gap, now at multiparty scale). The #26→#27 pair is the sharpest statement of the #24–26
arc: sealing the constructor *closes the topology half* (horn 3, concentrated at construction) and *leaves the
execution half exactly where it was* (horn 3, un-concentrable, recurring at every role). The sound topology
core factors (`projected_schedulability_closes`, a `..._closes` — the kernel models the static schedule's
*existence*, not its runtime *observance*); liveness stays the **third domain**, **progress** its third
coordinate (lock face #15/#16, session face #24, global-topology face #25, the *priority* face that broke #26,
and now the *projected* face that splits topology from execution). N stays **nine** modes (the rejection's
fourth firing adds none); the FAILED *broken-seam* tally **stays eight** (this is sound — a deliverable
rejection, not a soundness break).

### Primitive 28 — choice (`offer`/`select`, external `&` and internal `⊕`) → SEAM-COSTS [horn 3], SOUND but deliverable REJECTED (`seam/src/choice.rs`)

The session trilogy (#23 binary, #24 delegation, #25 multiparty, #26 priority, #27 projected) left exactly one
*core* session-type connective unprobed: **choice**, the branch. There are two dual forms (Honda 1993).
**External choice** `&` — your peer *sends a label* and you `offer` a branch per label; you *receive* the choice.
**Internal choice** `⊕` — you `select` a label and *send it*; you *make* the choice. Build it: an `Offer` enum
whose arms each carry the chosen continuation `Chan<R_i>`, an `offer()` returning it, and a `select()` that sends
a label and hands back the matching continuation. Staked **SEAM-COSTS [horn 3]** — the branch *mechanism* is
sound (a wrong branch cannot re-type; duality pairs each `select` label with its `offer` arm), and order /
linearity close as in #23. The soundness attacker (r35) conceded it: no UB, decidable, worst case a *starved or
leaked* but memory-safe session. Horn 1 holds.

But the judge (r35) **FAILED the §6 deliverable** — the **rejection mode's fifth firing** (after `typed_nat` #8,
`delegation` #24, `multiparty` #25, `projected` #27; #26 was the break between). The break is in §4 guarantee #2,
**"handle every label,"** and it is **vacuous**:

> - **Exhaustiveness is *scrutinee coverage*, not *run-to-End*.** Rust forces a `match` on the `Offer` enum to
>   cover every variant — but a safe arm `Ok(Branch::Left(c)) => { /* … */ }, _ => {}` type-checks with **no
>   `unsafe`** and the wildcard simply **drops** its `Chan<R>`. The branch is "handled" (the compiler is
>   satisfied) and yet *starved* (its protocol never runs to `End`). "Handle every label" has no content
>   *separate from* the conceded #17/#18/#23/#27 completion residue — it **collapses into it**. So there are now
>   *two* safe paths to the same gap: alongside `mem::forget`, the dropped wildcard arm.

The panel made four corrections, all accepted; each sharpens the thesis rather than sinking it:

> 1. **Branch-safety is *two-legged*, not the four §4 listed.** The genuinely branch-specific guarantees are
>    *label–continuation correlation* (label `L_i` is paired with exactly continuation `R_i`) and *branch-set
>    duality* (`select`'s label set is the dual of `offer`'s). The other two §4 items are not branch-specific:
>    *order* is the generic typestate guarantee carried since #18, and *"handle every label"* is the false leg.
> 2. **The deflation is onto *binary* `Either` + exhaustiveness — not n-ary *named* sums.** Rust's exhaustiveness
>    buys label–continuation correlation only for the *positional* `Either` shape; genuinely labeled n-ary choice
>    needs either positional nesting (which **drops the label names**) or `offer!`/`choose!` macros — *exactly*
>    what the `session_types` crate had to reach for.
> 3. **Horn 2 is Θ(type-size), not Θ(depth).** A balanced `Offer` tree has `2^depth` leaves, so the per-branch
>    resolver cost is linear in *type size*, not depth; interning keeps the pass linear-in-source (so in practice
>    it is *costless*), but the metric label I wrote was wrong.
> 4. **Completion is *amplified per-branch* and *peer-choice-reachable*, not "unchanged by branching."** Which
>    completion obligation fires is chosen by the *dual's* `select` — it is **adversary-triggered**, so the
>    residue is not branch-invariant; each branch carries its own.

So the core thesis **survives, sharpened**: choice adds **no new coordinate**. The branching connective opens no
new ground — completion stays the #17/#18/#23/#27 residue (now fed by two safe paths, `mem::forget` and the
dropped arm); liveness stays the **third domain**; **progress** stays its third coordinate (lock face #15/#16,
session face #24, global-topology face #25, priority face #26, projected face #27); **N stays nine** verdict
modes (the rejection's fifth firing adds none); and the FAILED *broken-seam* tally **stays eight** (sound — a
deliverable rejection, not a soundness break). Prior art confirms the whole shape: Honda's 1993 `&`/`⊕` are the
connectives, and the `session_types` / Jespersen–Munksgaard–Larsen (2015) **affine-vs-linear** gap *is* this
completion residue. The Lean instance is `choice_branch_typing_closes` (renamed from the staked
`choice_branch_safety_closes` — what closes is the *typing* of the branch, not its run-to-completion *safety*).

### Primitive 29 — must-complete (a `Session<P>` handle with a `Drop` completion bomb) → SEAM-COSTS [horn 3], SOUND but deliverable REJECTED (`seam/src/must_complete.rs`)

With every session *connective* exhausted (#23 binary through #28 choice), one thing was left un-probed — not a
feature but the **residue**: every session seam #17–#28 conceded the same *completion* gap (a session can be
abandoned mid-protocol, starving its peer). So #29 stopped probing features and attacked that leftover head-on.
Build it: a `Session<P>` affine handle carrying a `Drop` **completion bomb** (the same shape as `take` #9's
guard) — drop a `live` session before its protocol reaches `End` and the destructor *aborts*. The bomb makes the
floor *appear* to compel run-to-`End`. Staked the humble hypothesis **SEAM-BREAKS**: this is the *session face of
`linear` #17's must-run break* — `mem::forget` skips the bomb, the peer is starved, and so completion is an
**irreducible linear gap** that no affine floor can close.

The soundness attacker (r36) **conceded the mechanism break**: yes, `mem::forget` skips the bomb and the peer
starves — confirmed, no UB. But that only shows the *free-handle witness* is wrong, not that completion is
*unforceable*. And then the judge (r36) did something it had not done in the prior five rejections — it
**REVERSED** the finding instead of relabelling it (the **rejection mode's sixth firing**, and the most
instructive of the arc):

> - **The "universal none" is *refuted*.** A **scoped-closure / CPS combinator** — `with_session(|s| { …;
>   s.finish() })`, the **`std::thread::scope` analogue** — **compels completion modulo divergence, with no
>   `unsafe`**. It moves the completion obligation to the *re-import / construction boundary*: the consumer is
>   handed `s` only *inside* the scope and must return `End` (or diverge), so it never holds a forgettable *free*
>   handle. "Return `End` or diverge" is enforced by the type of `with_session`, not by a destructor that
>   `mem::forget` can dodge.

The sting is the **sixth refutation from my own cited lineage**: I cited `thread::scoped`'s **removal** (RFC 1084,
the `JoinGuard` leakpocalypse) as proof that completion *can't* be forced — and ignored its actual **fix**,
**`thread::scope`** (stable 1.63), which forces exactly this discipline. The panel made the corrections precise:

> 1. **Completion is *concentrable*, not an irreducible gap.** The whole #17–#28 completion residue was the
>    **price of the FREE-HANDLE API** — every prior session seam vended a forgettable `Chan`, so every one
>    carried it. Surrender the free movable handle and run the session *inside a scope*, and completion closes —
>    at a **horn-3 API-locality** cost, not at no cost and not never.
> 2. **Sessions ≠ `linear` #17's cleanup.** A *delimited* session **can** be scoped to force completion; an
>    *open-ended* cleanup cannot. So the pre-registered "session face of #17" claim is wrong — the analogy to
>    #17's must-run break does not hold, because the session has a construction boundary the cleanup lacks.
> 3. **The free-handle bomb is an ergonomic hazard.** A `Drop`-that-aborts turns `?` into a **process-abort
>    operator** — unusable on fallible or cancellable code. And peer starvation is **not** irreducible: affine-
>    session work (Mostrous–Vasconcelos, *Affine Sessions*; ECOOP 2022 *Stay Safe Under Panic*; MultiCrusty;
>    Exceptional GV) fixes it via **cancellation propagation**.

What **survives**: decidability — the completion check is Θ(depth), and where leak-detection is genuinely
undecidable the seam *correctly* conceded a NONE. So the verdict is **SEAM-COSTS [horn 3]**, sound but the §6
deliverable rejected — the FAILED *broken-seam* tally **stays eight** (my 8→9 prediction was wrong); **N stays
nine** verdict modes (the rejection's sixth firing adds none); liveness stays the **third domain**. The Lean
instance is `must_complete_scoped_completion_closes` (renamed from the staked
`must_complete_typed_discipline_factors`: the scoped witness *closes* completion at an API-locality cost; the
free-handle witness only *factored* it). And the **inverted meta-lesson** is the keeper: across the five prior
rejections I overclaimed concentration is **total**; here I overclaimed it is **irreducible** — the recurring
error is that I **overstate the extremes** (fully-carried or wholly-uncarried) when the truth is a **cost in
between**. The concrete scoped salvage — a `with_session` combinator that closes the gap #29 only located — **is now built
and HELD as #30** (`session_scope(a, b)`, the `std::thread::scope` analogue), mirroring the #26→#27 break→salvage
rhythm: completion is **concentrable** after all, closing the #29 reversal exactly as predicted — no safe `Done`
certificate can be forged, smuggled, or forgotten, so #29's forgettable destructor obligation becomes an
unforgeable return-type one. But the concentration is **per-endpoint, not whole-session**: under the
single-threaded run-`a`-then-`b` witness `a` can `finish` while `b` diverges, so *global* completion still escapes
via peer divergence — the #23/#27 per-session-vs-global gap one structural step on, the per-endpoint cost between
the extremes.

### The taxonomy (the whole point, in one table)

| Primitive | What the missing conjunct *is* | Fate | Pays with |
|---|---|---|---|
| SPSC queue | floor-expressible (affinity/NLL/`Send`) | **closes** | nothing — every axis ✓ |
| Slab | runtime-checkable (generation) | **closes** | zero-cost (#6) |
| Branded index | cross-object identity (generative brand) | **closes** | API-locality; brand is illegible (#7); #6 deferred |
| Refined index | in-bounds via a legible predicate (SMT) | **closes** | solver-freedom (horn 2); R2 trade — brand-free *xor* zero-cost |
| Shared (Rc) | dynamic shared ownership (refcount) | **closes** | bare-metal (#5) — needs a heap; bare-metal↔capacity trade |
| Type-level index | in-bounds via a compile-time trait proof | **closes (sound)** | a `recursion_limit` cost inside horn 2 — but **no clean axis** (confounded + encoding-relative) |
| Take-and-replace | transient broken invariant across an unwind | **closes** | **unwind-safety (8th axis)** — via a discharge menu (abort / repair / poisoning) |
| Drain-and-yield | a load-bearing guard `Drop` on a leak-permitting floor | **closes** | **leak-freedom (9th axis)** — pre-pessimize; a leaked guard loses the tail (distinct from unwind-safety) |
| Self-ref cell | a value whose *address* is load-bearing | **closes (sound)** | **move-stability — INSEPARABLE** (immovability; non-uniform: stack-pin→API-locality, heap-pin→bare-metal). A distinct danger with **no clean axis**; N stays 9 |
| Validity (bytes→enum) | a value whose *bit pattern* must be valid | **closes (sound)** | **PROJECTS ONTO THE BASIS** — per-read → #6, zero-copy view → API-locality. **No new axis** (2nd inseparable). The closed-basis proof; N stays 9 |
| MMIO | external physical axiom | **sound modulo TCB** | a trusted `assume` |
| Intrusive list | aliasing+liveness that escapes to the caller | **breaks** | a trilemma horn |
| Constant-time secret | branching on a secret leaks it (timing — **not UB**) | **breaks (IFC sense)** | **DIFFERENT DOMAIN** — information-flow, out of frame. The trilemma recurs; closure is *per domain* (its own basis: declassification discipline, …) |
| Capability-gated declassify | release of a secret must be *bounded* (where/when/how-much) | **breaks (IFC sense)** | **FLOOR MISMATCH** — `map` hands a closure bare plaintext (needs an effect system); the affine cap counts *uses*, not *bits* (needs QIF). No IFC seam closes on Rust's floor; affinity bounds *resource uses*, not *information amount* |
| Type-level lock ordering | held lock set must be a strictly-decreasing global chain (deadlock — **not UB**) | **breaks (liveness sense)** | **PARTIAL FLOOR-MATCH** — re-entrant `root` mints a 2nd token lineage → typable inversion (wrong *granularity*: per-token vs per-thread); and the dynamic/same-rank fragment escapes the static floor (wrong *coverage*). A *third* domain; the match is **graded** (full/partial/none), not binary. BLR'02-minus-dynamic; name it *lock-order-inversion freedom* |
| Runtime lock registry | same — but on an *upgraded* (runtime, thread-local) floor | **breaks (liveness sense)** | **NON-MONOTONE FLOOR-UPGRADE** — fixes *coverage* (runtime ranks → dynamic orderings) and pays #5 (can't be `no_std`) + #6 (per-acquire check) + a prevention→detection modality; but *granularity recurs* — held-set is thread-keyed, async control is task-keyed, so work-stealing migration launders an undetected inversion. Granularity is a **ladder** (token<thread<task); no ambient floor tops it. WITNESS/lockdep precedent |
| Must-run cleanup guard | a finalizer that must run *exactly* once (a *liveness/correctness* obligation — **not UB**) | **breaks (liveness sense)** | **WRONG STRUCTURAL LOGIC** (new verdict mode) — `mem::forget` is safe, so the affine floor (*at most* once) cannot compel a *linear* (*exactly* once) finalizer; the leakpocalypse. Grade is a clean **none** (the `#[must_use]` lint is a cosmetic advisory, not a floor fragment — `scopeguard`). The match unit is the *precondition*, not the domain (deadlock-freedom is *partial*, this is *none*, same liveness domain). N unchanged — the *enforcement* face of axis 9 (`drain` was *tolerance*). No decidable price closes it; sharpened to Boats' scoped-task trilemma (Rust rejected the linear horn) |
| Scope-restore guard (`scopeguard`) | #17's *identical* un-compellable finalizer — but packaged as a side-obligation beside a sound ownership core | **costs (holds)** | **PACKAGING DIAL — real but BOUNDED** — the `mem::forget` gap that *broke* #17's horn 1 only **costs horn 3** here (a stranded invariant, recurring per `?`/panic/return — not UB), because soundness is *decoupled* from the closure (an owned value). But the move is **confounded** (a side-obligation needs a sound core to sit beside, so value-ownership co-varies) and the horn is **forceable** (borrowed-slot/`take_mut` packaging forces horn-1 UB; `Pin` / `thread::scope` are un-repackageable) — *which horn* depends on **where safety rests**, partly danger-intrinsic. Not novel (`JoinGuard` horn-1 / `scopeguard` horn-3, RFC 1066). No new mode |
| Borrowed-slot take-and-reseat (`take_mut` as guard) | #19's *identical* packaging (a guard with `into_inner`/Drop-restore) but over a **borrowed slot**, not an owned value — single ownership across the transient `ptr::read` duplicate | **breaks (horn 1)** | **FORCING THE HORN — the fence is real** — `into_inner` (sound for #19's owned value) is a safe-code **double-free** here (returns the value while the slot keeps the duplicate — two owners; a 4-line `String` witness, no `unsafe`). The single-variable flip #19→#20 (only owned→borrowed) is **clean**, repairing #19's confound; and the fence is **forced** (the `into_inner`-less sound subset is degenerate, dominated by `&mut T`, so every *usable* packaging breaks). Closes the trilogy: the horn is movable **iff** soundness can decouple from the un-compellable closure. Not novel (`take_mut`+`scopeguard`+RFC 1066). No new mode |
| State-based CRDT merge (`Replica<T: JoinSemilattice>`) | replicas must *converge* — `join` must obey the semilattice laws (commutative/associative/idempotent — **not UB**) | **breaks (floor-mismatch), but DEFLATES** | **FOURTH-DOMAIN PROBE — NO NEW DOMAIN** — the `T: JoinSemilattice` bound is a *syntactic proxy, not a witness*; a safe lawless `join` type-checks and diverges replicas (double-counts on redelivery; undecidable by Rice). But the candidate fourth domain **deflated**: the blessed-CRDT TCB rescue is *not* MMIO-shaped (open lattice space → trust recurs per CRDT), 'dischargeable' only at the model level (no Rust pass checks model-to-shipped-code), and the gap is the *generic* unchecked-typeclass-law phenomenon (`Ord`/`Eq`/`Hash`/`Monoid`/semilattice — signatures checked, laws not), so convergence **projects onto `restore`'s application-correctness basis**. The map stays at three domains + application-correctness; scored a TRADEOFF survival, FAILED tally stays 6. No new mode |
| Cost-typed computation (`Timed<C, T>`, a graded cost monad) | a computation must meet a *cycle deadline* — real elapsed time ≤ budget (real-time/WCET — **not UB**) | **breaks (horn 1), FAILED** | **SECOND FOURTH-DOMAIN PROBE — FAILED, no new domain** — `step` charges +1 *decoupled* from its closure, so a 10⁶-iteration loop is cost 1 and `within::<1>` admits it (the op-count is an *unsound* proxy for cycles; my §4 over-claimed the deadline). All five attackers landed: Horn 3 is *false* (non-unifying `if`-branches force manual step-padding); the unary cost is O(C), uncompilable at real budgets; no additive composition; and it is **Danielsson's 2008 cost monad** — "op-count ≠ cycles" is WCET's *founding premise*, so *pick a lane* (abstract cost xor physical deadline). Deflates: cost monad / `typed_nat` #8 for the static part, an `mmio`-shaped-but-*uninstalled* axiom #2 for the physical claim, liveness's partial-match for data-dependent loops. The map stays at three domains + application-correctness; FAILED tally rises to **7**. No new mode |
| Real binary session (`session()` → two dual `Chan` over `Rc<RefCell>`) | two peers' protocol run in order *and to completion* — neither abandons, starving its dual (liveness — **not UB**) | **costs (holds)** | **LIVENESS DEEPENING — #18's grading survives, framing deflated** — removes #18's single-handle limitation (two concurrently-live dual endpoints, the `E0499` #18 hit); order **closes** + *per-session* relative progress **closes** (duality, `HasDual`), completion **costs horn 3** (now peer-observable). But the panel deflated the "real binary session" framing in five accepted ways: relative progress closes only **per session** (two interleaved `session()`s type-check a mutual-wait deadlock — Kobayashi/Padovani; global deadlock-freedom needs *multiparty* typing); the single-threaded witness makes "peer waits forever" self-abandonment-flavored (only the *static* limitation lifted, not `session_types` 2015's real concurrency); the fallible `recv`'s `Err` is **non-actionable** because `mem::forget` leaks the `Rc` (the **leakpocalypse double-bind** — the gap that costs completion also denies an actionable abandonment signal); only straight-line `i32` protocols (no choice/delegation/recursion). Net: two sharpenings of the liveness map; refuted its own headline, confirmed the grading. FAILED tally stays 7; N stays 9. No new mode |
| Delegation (channel-over-channel; `send_chan`/`recv_chan` over `Rc<RefCell>`) | a delegated session must reach its peer and run to completion — abandoning it deadlocks the *third* party (liveness — **not UB**) | **holds (sound), DELIVERABLE REJECTED** | **REJECTION MODE (2nd, after #8) — delegation opens the PROGRESS coordinate** — the *mechanism* is sound (SEAM-COSTS [horn 3]; no wrong-`Q` re-typing — duality pins `Q`; order + don't-use-after-delegate close, the latter carried free by the affine *move*), but the judge **FAILED the §6 deliverable**: my "no new coordinate / projects onto leak-freedom (#9/#10)" claim was refuted *from my own cited lineage*. Delegation's signature hazard (B forgets `Chan<Q>`, starving C) is "deadlock at the other end" (Jespersen et al. 2015) — **progress / deadlock-freedom**, a coordinate *distinct* from completion and leak-freedom; delegation is the recognized feature that *breaks* first-order sessions' automatic deadlock-freedom (Coppo–Dezani–Yoshida–Padovani; Kobayashi; deadlock-free GV). I saw the `Rc` hazard's leak-*face* and mislabeled the progress *axis*; the single-threaded witness hides deadlock by construction (the symmetric cycle is type-obstructed — rescues *behavior*, not *classification*). The probe earned its keep by being instructively *wrong*. FAILED tally stays 7 (sound — a deliverable rejection); N stays 9 |
| Multiparty session (`relay`/`ring` over #23's binary edges) | the *global* composition must be deadlock-free — a source role must exist; the per-edge floor cannot range over the whole topology (liveness — **not UB**) | **holds (sound), DELIVERABLE REJECTED** | **REJECTION MODE (3rd, after #8/#24) — a FALSE DICHOTOMY** — the *mechanism* is sound (SEAM-COSTS [horn 3]; role-local order + linearity close; `ring()` type-checks a globally-deadlocked system of perfectly-dual edges), but the judge **FAILED the §6 deliverable**: my claim that global progress lives *only* whole-protocol in a *generative* global-type projection (no per-role discipline able to reject the ring) was refuted *from my own cited lineage*. The omitted middle rung — **per-channel priority/obligation typing** (Kobayashi linear-π; Padovani; Dardha–Gay FoSSaCS 2018; Kokke–Dardha) — rejects the source-less ring *compositionally, with no global type*, via `prio(recv) < prio(send)` → `p < p` on the cycle; I had *mis-filed Kobayashi as support when he is the counterexample*. Three rungs, not two: (a) per-edge floor [no]; (b) per-channel priority typing + whole-program acyclicity [no global type]; (c) generative MPST. Survives: global progress is *not concentrable per-call* and needs strictly more than the per-edge floor — but it is a *whole-program* obligation, not necessarily a generative one. `multiparty_role_order_core_closes`; FAILED tally stays 7; N stays 9 |
| Scoped session (`session_scope(a, b)` → two dual `Done`-returning closures over a `VecDeque` FIFO; no free `start`) | #29's *identical* completion obligation — each endpoint must run to `End` — but re-imported as an **unforgeable return-type** certificate, not a `mem::forget`-able `Drop` bomb (liveness — **not UB**) | **costs (HELD), salvages #29** | **SCOPED SALVAGE OF #29 — completion is *concentrable* (HELD, a TRADEOFF like #19/#23, not a rejection and not a break)** — there is no free `start`: the only `Session` comes from `session_scope(a, b)` (the self-contained two-endpoint form of #29's reversal `with_session(|s| …)`), which hands each dual role a brand-trapped endpoint and **requires each closure to return a `Done<'scope>`** — a certificate mintable *only* by `finish()` at `End`. So #29's *forgettable destructor* obligation becomes an **unforgeable return-type** obligation: every attacker conceded no safe `Done` can be **forged** (private fields), **smuggled** (the invariant `'scope` brand, lifted verbatim from branded-index #5 = `thread::scope`'s generativity), or **forgotten** (forgetting mints no certificate — `mem::forget` is genuinely defanged, the #29 reversal confirmed as predicted). Horn 1 (no UB — worst case a hung/leaked session) and Horn 2 (Θ(depth) typestate, no SMT) stay **intact**, so the whole #17–#28 completion residue **was the price of the FREE-HANDLE API**, recoverable by surrendering the free handle. The sharpening (the caveat *is* the finding): concentration is **per-endpoint, not whole-session** — §2 over-claimed "every session a scope opens runs to `End`," but the single-threaded run-`a`-then-`b` witness lets `a` `finish` while `b` *diverges*, starving the peer, so **global (both-endpoint) completion escapes via peer divergence** — the #23 (relative progress closes only per-session) / #27 (the execution obligation never leaves the consumer) gap one structural step on; the builder hedged this in the per-endpoint / modulo-divergence residue. Secondary: the **synchronous-trap** cost was understated (sequential `a`-then-`b` cannot express genuinely concurrent/interleaved sessions). Deflation (conceded): no new mechanism — `thread::scope` + generativity (branded #5) + #29's `finish`/`End` typestate. Meta (cf. #29): I overstated the extreme again (#29 "irreducible," here "whole-session global"); the truth is the per-endpoint cost between. FAILED *broken-seam* tally **stays eight**; **N stays nine**. `must_scoped_completion_closes` |
| Recursion (`Rec<P>`/`Var`; single-level `µX.T` unfolding via an env-threaded `Session<P, E>` with `enter()`/`recur()`) | the looping connective `µX.T` at unbounded depth — a recursive session must be *productive* (well-founded/contractive unfolding) to make progress, the boundary of #30 probed at infinite depth; without a choice connective, single-level recursion is *necessarily* non-terminating, no reachable `End` (liveness — **not UB**) | **costs (sound), DELIVERABLE REJECTED (7th)** | **LAST STRUCTURAL SESSION FEATURE — REJECTION MODE (7th, after #8/#24/#25/#27/#28/#29) — the OVER-CONCESSION** — the *mechanism* is sound (SEAM-COSTS; **Horn 1**: the env-threaded `Var` encoding admits no ill-formed unfolding — `enter()` unfolds `Rec<P>` (sets `E := Rec<P>`) and `recur()` exists *only* for `Session<Var, Rec<P>>`, so a top-level `Var` over env `NoRec` is **inert**, sound *by construction* like branded #5, worst case a starved peer not UB; **Horn 2**: decidable, Θ(type-size); `HasDual` threads recursion, `Rec<Tx<Var>>` infinite producer ↔ `Rec<Rx<Var>>` consumer), and it **bounds #30**: `Rec<Tx<Var>>` has *no reachable `End`*, so #30's `Done`-forcing is **vacuous** — recursion exposes #30's unstated precondition (a reachable terminal). But the judge **FAILED the §6 deliverable**: I **over-conceded** — filed *productivity/guardedness* as an undecidable **NONE** beside termination. WRONG: guardedness is a **DECIDABLE COST**. `Rec<Var>` = `µX.X` is the **non-contractive** type every system I cited (Coq/Agda **guarded corecursion**; contractive recursive types) rejects *syntactically*; a sealed **`Guarded` marker** (impl for `Tx`/`Rx`/`End`/`Rec`, **not** `Var`) bounding `enter`/`Rec`-formation carries it in Θ(type-size), no SMT — the **same machinery as order** — so guardedness **CONCENTRATES (horn 3)**; **only termination** (halting) is the genuine undecidable NONE. The **seventh refutation from my own cited lineage**. Meta (the *inverse* of the arc's usual error, same shape): #8/#24/#25/#27/#28 over-claimed concentration *total*, #29/#30 over-claimed the danger *irreducible/global*; here I **over-conceded** — put a decidable cost at the undecidable extreme. Unifying: I flee the **middle** (a decidable, partial cost) toward whichever extreme is rhetorically cleaner. Corrected: order + duality + **guardedness** all **close** (concentrable, decidable); **termination alone** is the undecidable NONE. FAILED *broken-seam* tally **stays eight**; **N stays nine**; the #32 salvage — *build the `Guarded` marker* — is **now built and HELD as #32** ([none], SURVIVED; the #31→#32 reject→salvage rhythm completes, mirroring #26→#27 and #29→#30). `recursion_guarded_order_closes` |
| Guarded (#31's `Rec<P>`/`Var` machinery + a sealed `Guarded` marker — a `sealed::Sealed` supertrait, no downstream impls; impl'd for `Tx<C>`/`Rx<C>`/`End`, **not** `Var` — bounding `enter()`/`recur()` on the recursion body) | the same `µX.T` looping connective, now its *productivity/contractivity* obligation: the unguarded `Rec<Var>` = `µX.X` (the non-contractive spin loop) must be rejected so unfolding makes progress (liveness — **not UB**) | **costs [none] — SURVIVED, salvages #31** | **SALVAGE OF #31 — the SURVIVAL, [none], the CLEANEST result of the arc** — #31's `Rec<P>`/`Var` machinery PLUS a sealed **`Guarded` marker** (a `sealed::Sealed` supertrait, no downstream impls; impl'd for `Tx<C>`/`Rx<C>`/`End`, **not** `Var`) bounds `enter()`/`recur()`, so the unguarded `Rec<Var>` = `µX.X` (the non-contractive spin loop) **cannot be unfolded** — rejected at compile time, Θ(type-size), no SMT. KEY: for single-level recursion a **shallow head-check IS full contractivity** (an action head `Tx`/`Rx` guards everything inside it incl. any `Var`; `End` is vacuously guarded; a bare-`Var` head is unguarded), so `Guarded` need only be impl'd for the **3 heads** — no recursive walk, *literally the same trait-resolution machinery as order*. Verdict **SEAM-COSTS [none]**, JUDGE → **SURVIVED (1/1)**: all three horns hold, **no horn dropped** (stronger than #30's [horn3]) — no safe unfold of `Rec<Var>` (Horn 1, sealed marker + depth-1 resolution), zero-ceremony concentration (Horn 3), decidable no-SMT (Horn 2), no in-scope false positive. **Closes #31's over-concession**: guardedness/productivity **concentrates** (a decidable cost, not the undecidable NONE I filed); and I **avoided the #27 salvage-over-claim trap** (where #26's salvage was itself rejected for a fresh over-claim) by claiming **only productivity** and conceding termination up front. The panel **marked a tradeoff (not a horn-drop — the seam still SURVIVED)**: completeness is **bought by restriction** — the head-check = full contractivity *only* because the single-level, choice-free grammar collapses it to "head ≠ `Var`"; a richer grammar (nested `Rec`, or choice-in-recursion `Rec<Offer<End,…Var>>`) needs the real recursive contractivity walk, which this seam **scopes out, not escapes** (and my "folklore" deflation slightly overstated generality). Buys **productivity** (no silent `µX.X` spin), **NOT termination** (`Rec<Tx<Var>>` is guarded and still infinite — the halting problem stays the one conceded undecidable NONE). FAILED *broken-seam* tally **stays eight**; **N stays nine**. `guarded_recursion_productivity_closes` |

The decontamination signal (Part 6) held across all twelve **memory-safety** primitives: the obstruction
*vanished* (SPSC), *moved and stayed decidable* (MMIO), *moved onto a runtime witness* (slab), *moved onto
the type surface* (branded index), *moved into the trusted toolchain* (refined index — into the SMT
solver), *moved into a runtime subsystem* (shared — into the global allocator), *moved onto the resolver's
fuel budget* (type-level index — but that site proved *not* to be a clean axis), *moved onto the unwind
path* (take-and-replace — a mid-take panic becomes an abort; that site *was* a clean axis, the eighth),
*moved onto the leak path* (drain-and-yield — a forgotten guard loses the tail; the ninth axis, distinct
from the eighth), *moved onto immovability* (self-ref cell — a `Pin`-revoked move capability; a real
danger but with no clean coordinate, the inseparable category), *moved onto an already-occupied axis*
(validity — bit-validity → #6 or API-locality, adding nothing new; the closed-basis signature), or *moved
and broke soundness* (intrusive) — and **where it moves is the entire verdict.** The thirteenth, the
constant-time secret, *moved out of the memory-safety domain entirely* (onto a timing channel the AM does
not model) and **broke** there — the same decontamination logic, applied in a new domain, locating a
*different* basis. The fourteenth, capability-gated declassification, *tried to close that new domain* on
Rust's floor and **broke for a structural reason**: the obstruction has nowhere to move *to*, because the
floor carries none of the IFC precondition — `map` must hand the closure bare plaintext (the contaminant
escapes uncaught) and the affine cap counts *uses* where the danger is *bits*. So the decontamination
signal completes its own statement: **the obstruction vanishes only when the floor matches the domain.**
On a matched floor it moves somewhere nameable (every memory-safety primitive); on a mismatched floor it
has no landing site and the seam breaks (both IFC primitives) — pointing at the floor that *would* carry it
(Jif / FlowCaml / QIF). The fifteenth, type-level lock ordering, *moved the obstruction into a third domain*
(liveness) and found the floor carries it only **partway**: the static, single-stack, distinct-rank
fragment lands cleanly (the obstruction moves onto the type-level order), but the per-thread fragment has no
ambient landing site (re-entrant `root` re-opens it) and the dynamic fragment escapes the static floor
entirely. So the obstruction *half-moves* — and that completes the signal's penultimate form: **the match is
graded.** It moves fully (memory safety), not at all (information flow), or *partly* (liveness). The
sixteenth, the runtime registry, then tried to push the half-moved obstruction *the rest of the way* by
upgrading the floor — and found it **moves the obstruction sideways, not all the way home**: the dynamic
fragment now lands (onto the runtime check), but the granularity fragment, evicted from the token rung,
*reappears* at the thread rung (and the task rung beyond it). So the signal's final form: **the obstruction
moves one rung per floor, and the rungs don't run out.** Where it lands, how completely, and whether the
next floor up merely relocates it, is the entire verdict.

---

# Part 10 — Mechanization and honest bounds

The trilemma's logical skeleton is machine-checked in Lean 4 (`proof/SeamGap.lean`,
`propext`-only, no `sorry`). The core characterization:

> **A seam closes if and only if its soundness precondition factors through the safe witness.**
> (`seam_closes_iff_factors`.)

Instantiated: `spsc_seam_closes` (precondition = witness → closes); `intrusive_seam_does_not_close`
(precondition is the gap → no closing decision exists); the slab pairing
`slab_runtime_closes_but_static_does_not` (the *same* precondition closes through a *runtime*
witness but not a *static* one — the zero-cost axis, made formal at the model level); and
`branded_seam_closes`/`refined_seam_closes`/`shared_seam_closes`/`typed_nat_seam_closes`/`take_seam_closes`/`drain_seam_closes`/`selfref_seam_closes`/`validity_seam_closes`
(all close *indistinguishably from SPSC* in the kernel — which is the point: the branded index's viral
type-parameter cost, the refined index's solver dependency, the shared seam's heap requirement, the
type-level index's `recursion_limit` cost, the take seam's abort-on-panic, the drain seam's leaked-tail,
the self-ref cell's immovability, and the validity seam's bit-check are alike invisible to a kernel that
sees only whether a Boolean witness suffices — which is *exactly* why the axis claims had to be
adjudicated by the cold panel in prose, not the kernel: the kernel certifies that all of
`typed_nat`/`take`/`drain`/`selfref`/`validity` *close*, and says nothing about the fact that one spends a
confounded non-axis, two spend clean distinct axes, one spends a real-but-inseparable danger, and one
spends a coordinate *already on the map* — the last of which is how you know the basis is closed). Four more
*close* in the kernel but **hold-with-a-cost** in prose — `session_order_safety_closes`,
`restore_ownership_core_closes`, `binary_session_progress_core_closes`, and `delegation_handoff_core_closes`
(the eighteenth, nineteenth, twenty-third, and twenty-fourth seams): the kernel sees their sound *order-safety* /
*ownership-core* / *order-plus-per-session-progress* / *order-plus-affine-handoff* component factor through the
witness, but not the cost the cold panel charged each (the session's un-completable protocol; the restore
guard's un-compellable closure — the #17 gap re-packaged so it costs instead of breaks; the binary session's
un-compellable *peer*, whose `mem::forget` starves the dual — the #17/#18 gap now across a two-party boundary,
the real-binary-session deepening of #18 that survives but whose "real session" framing the panel deflated to
per-session-only progress; and the delegation seam's order-plus-affine-handoff core — sound, but whose §6
deliverable the panel *rejected*, the rejection mode firing a second time after `typed_nat` #8: delegation opens
**progress / deadlock-freedom** as a coordinate distinct from completion and leak-freedom, which I had mislabeled
as a leak and the kernel models not at all). The
last five, the out-of-memory-safety primitives, are the sharpest illustration: their Lean instances are
named `secret_typed_discipline_factors`, `labeled_typed_discipline_factors`,
`ordered_locks_typed_discipline_factors`, `lock_registry_typed_discipline_factors`, and
`linear_typed_discipline_factors`, *not* `..._closes`, because the kernel models neither time nor
non-interference nor quantitative leakage nor liveness/progress nor linearity —
each certifies only that the *typed* discipline factors, while the cold panel ruled all five real seams
**break** in a domain the kernel cannot see at all (the secret out of frame, the labeled on a mismatched
floor, the lock ordering on a *partially* matched one, the runtime registry on a *non-monotonically*
upgraded one — the thread-vs-task granularity recursion the kernel's Boolean witness cannot model — and the
must-run guard on a floor of the *wrong structural logic*, affine where linear is needed). A **sixth**
`..._factors` — `reseat_typed_discipline_factors`, the twentieth seam — is the exception that proves the
pattern: it breaks *inside* memory safety (a safe-code `into_inner` double-free over a borrowed slot), so the
kernel *could* in principle see it, yet the Boolean witness still factors — it models single ownership but not
the transient `ptr::read` duplicate window, exactly the gap the cold panel's `String` witness drove through.
A **seventh** — `crdt_typed_discipline_factors`, the twenty-first seam — is the fourth-domain probe that
*deflated*: it breaks *floor-relatively* (a safe lawless `join` diverges replicas, the semilattice laws being
unenforceable — undecidable by Rice), and the kernel sees only that the `T: JoinSemilattice` trait bound
factors, never the divergence. But the cold panel ruled the candidate fourth domain a **deflation** — the
generic unchecked-typeclass-law gap (`Ord`/`Eq`/`Hash`/`Monoid`/semilattice signatures checked, laws not),
projecting onto `restore`'s application-correctness basis — so convergence opens no new domain, and the map
stays at three soundness domains plus application-correctness. An **eighth** — `wcet_typed_discipline_factors`,
the twenty-second seam — is the *second* fourth-domain probe, and the first of the two to **FAIL** outright:
it breaks because `step` charges one unit decoupled from its closure's work, so `within::<1>` admits a
million-cycle computation (the op-count is an *unsound* proxy for real time), and the kernel sees only that the
static op-count factors, never the cycle gap. The cold panel ruled WCET another **deflation** (Danielsson's
2008 cost monad / `typed_nat` #8 for the static part, an uninstalled `mmio`-shaped axiom for the physical
claim, liveness's partial-match for data-dependent loops) — so the *second* fourth-domain candidate opens no
new domain either, and the three-domain-plus-application-correctness map holds.

**The honest bound — and why it matters as a lesson.** A cold adversarial review of the proof
returned the right verdict: the *checked* math is a real characterization, but the *faithfulness*
of "precondition" and "witness" to actual Rust primitives still lives in the **prose modeling
dictionary**, not the Lean kernel. Lifting that into the kernel is a RustBelt/Iris-scale,
months-long effort. The integrity move is to **ship the adversary's verdict alongside the proof**
and to never let the word "machine-checked" transfer more confidence than the *checked part*
earns. This is a transferable habit: when you mechanize something, be precise about *which*
claim the machine actually checked.

---

# Part 11 — The transferable meta-lessons

You can forget every primitive and keep these.

1. **Conservation of essential difficulty (Brooks).** Fred Brooks split complexity into
   *accidental* (incidental to your tools, removable) and *essential* (inherent to the problem,
   not removable). Memory-safety-with-aliasing-and-no-GC has irreducible essential difficulty;
   you can *relocate* it (to a borrow checker, a runtime flag, a proof, a trusted axiom) but not
   *delete* it. When a design promises to make essential difficulty vanish, look for where it
   actually went.

2. **Trilemmas are the signature of a real boundary.** A "pick 2 of 3" where every *pair* works
   but the *triple* doesn't is a stronger signal than a flat "impossible." It tells you the
   constraints genuinely pull in different directions. Compare CAP (consistency/availability/
   partition-tolerance), the macro "impossible trinity," FLP. The companion essay
   (`THE-SAME-WALL.md`) develops a **falsifiability test** distinguishing these real boundaries
   from fake ones: a deletable seam (Maxwell fusing electricity and magnetism) is a *veil over a
   latent unity*; a permanent one (decidability vs expressiveness) is a *boundary between opposed
   constraints*. The test: does merging reveal the two were the same thing (→ delete) or force a
   choice neither side can make (→ relocate)?

3. **Name your seam, and shape it deliberately.** Every nontrivial system has a boundary between
   a cheap/checked world and a trusted/proven one. The amateur move is to pretend it isn't there;
   the professional move is to (a) put it in one place (concentration), (b) make the obligation
   at it as visible as possible, and (c) choose *consciously* whether the boundary costs you
   soundness, decidability, concentration, runtime, API surface, a solver in your toolchain, or
   exception-safety, leak-freedom. The twenty-four-primitive taxonomy is a decision procedure for that choice.

4. **Adversarial separation of build and break.** Construction wants continuity; demolition wants
   a cold, context-free skeptic. Keeping them as separate machines — and *calibrating* your
   skeptic on known-good and known-bad inputs before trusting it — is a method you can apply to
   code review, design review, and your own reasoning. A reviewer who never rejects anything (or
   rejects everything) is uncalibrated and worthless; prove yours isn't.

5. **Decontamination beats argument.** When something fails and you can't tell whether the *idea*
   is impossible or your *attempt* was weak, hand the attempt a stipulated known-good component
   and watch whether the failure *moves* or *vanishes*. Moving = structural; vanishing = you were
   just bad at it. This single discriminator is worth more than a week of debate.

6. **A bundled property hides axes; the witness that splits them proves they were there.** "Pick 2
   of 3" framings are clean, but a horn is often a *bundle* — "concentrated & proof-free" hid zero
   runtime cost; "concentration" hid the difference between code crossings and type infection; even
   "decidable" hid the difference between a terminating checker and a solver in your TCB. You
   discover a folded axis exactly one way: exhibit two artifacts the framing calls identical that
   *measurably differ* on the candidate (the slab vs. SPSC on runtime; the branded index vs. the
   slab on the type surface). And mind the geometry: to prove two axes *independent* you need an
   **off-diagonal** — two artifacts that agree on one axis and flip the *other* (the refined index
   and the slab are both brand-free but differ on legibility; a single brand-free-and-legible point
   would not have done it). Until you have that witness, a "new axis" is a hunch; with it, the
   dimension is real. Your tidy trilemma may be a shadow — look for the object casting it. **But the
off-diagonal must be *clean*:** the eighth probe (the type-level index) *looked* like it flipped only
checker-tractability against SPSC — and was rejected because it silently *also* flipped expressiveness
(it indexes only compile-time constants). An off-diagonal that moves two coordinates at once proves
nothing. Check that everything *except* the candidate is genuinely held fixed; if it can't be (here,
intrinsically — a runtime index cannot ride a compile-time proof), the axis is not isolable by that
witness, and the honest move is to drop the claim.

7. **Deflate your own discoveries: "new to me" ≠ "new."** *Every* axis the project's adversary
   confirmed as real and independent it also proved *not novel* — solver-freedom is refinement-types'
   founding tradeoff (DML, 1998); bare-metal is Rust's `core`/`alloc` boundary (RFC 2480); unwind-safety
   is Abrahams' exception-safety guarantees (and RFC #1508 had already rejected the very `take_mut` that
   witnesses it). Each was a genuine dimension of *this* lattice and each already had a name in the
   literature. The discipline: before claiming a discovery, ask "what is this *already called*?" The
   honest, larger result was not "nine new axes" but "nine *known* PL design axes are all faces of
   one tension." A unification you can name is worth more than a dimension you can't — and a cold
   reviewer who deflates your count is doing you the biggest favor in the room. (One subtlety the ninth
   axis added: "deflatable" is not "not an axis." Leak-freedom is the *thinnest* axis — its cost is
   invisible in all non-leak code — yet still real, a populated off-diagonal. Thinness is a property of
   an axis, not a disqualification from being one.)

8. **A measuring instrument must return "no", "yes", the right *name*, the right *boundaries*, and "real
   but unmeasurable".** The axes past seven took four probes, and together they exercise every mode an
   instrument needs. On the first (a type-level index pitched as *checker-tractability*) the adversary
   returned **no**: it threw the axis out — confounded with expressiveness, the cost a unary-encoding
   artifact, the name wrong. On the second (take-and-replace, *unwind-safety*) it returned **yes, but** —
   confirmed a clean axis *and* corrected the label (it is the exception-safety-*guarantee-level*; abort
   is one discharge of it). On the third (drain-and-yield, *leak-freedom*) it returned **yes, and here's
   why it doesn't merge** — confirmed a clean axis *and* ruled it distinct from its tempting sibling
   unwind-safety (leak = `Drop` never runs; unwind = `Drop` runs at a bad time). On the fourth (a
   self-referential cell, *move-stability*) it returned the subtlest answer of all — **yes, this danger
   is real and distinct, but it has no isolable cost**: every witness for it spends a *different other*
   axis (stack-pin → API-locality, heap-pin → bare-metal), so it is neither a clean coordinate nor a
   confound. A procedure that only ever confirms is a confirmation machine; one that only ever rejects is
   a cynic; one that confirms but can't tell two neighbors apart is a blurry lens; one that calls every
   real danger a "dimension" is an over-counter. And on the fifth (a validity-checked read) it returned
   the answer that *ends the search* — **yes, this danger is real and brand-new, but it adds nothing to
   the list; its cost is a coordinate you already have**. This one did all of it — reject, confirm,
   relabel, distinguish, recognise-the-inseparable, *and certify-that-a-new-thing-adds-nothing* — which is
   exactly why you can trust the nine axes it reports (and the dangers it honestly declined to make a
   tenth). Seek the input that makes your method return *false*, the input that makes it confirm *the
   wrong thing*, the input that tempts it to *merge two different things*, the input that is *real but
   cannot be cleanly measured*, and the input that is *new but redundant*; the value is in those five gaps.

9. **A measuring system is finished when a genuinely new input lands on an old coordinate.** The whole
   constructive arc was a search for axes, and the honest way to *end* it was not to run out of energy
   but to find a stopping criterion. That criterion is the **closed basis**: when you deliberately feed
   in a danger from a category you have never touched (here, bit-level validity — no prior axis was about
   a value's bit pattern) and the cost it pays is *already on your map*, you have evidence the map is
   complete. It is the dual of the discovery move — there, *one* danger (out-of-bounds access) needed
   *many* witnesses that each spent a *different* axis (so the axes were really distinct); here, *many*
   dangers (a stale handle, an invalid bit pattern) spend the *same* axis (so the axis-list is really
   finite). Neither direction is a proof — the bound stays a *lower* bound, a future danger could still
   surprise — but "a new danger class projecting cleanly onto the basis" is what closure *looks like*,
   and naming that as a result is more honest than either claiming completeness or probing forever. When
   your taxonomy stops growing under new inputs, say so, and stop.

10. **A closed basis is closed *within a domain* — name the domain, then look for the next one.** The
    strongest move after "stop" is to ask what you were *implicitly* holding fixed. The whole nine-axis
    basis answers **"what does a sound *memory-safety* witness cost?"** — and *memory safety* was the
    silent parameter. Feed the method a danger from a *different soundness domain* — **information flow**,
    where the danger is a timing leak, not UB — and three things happen at once: the Rust Abstract Machine
    sees *nothing* (it models no time), so "can safe code reach UB?" is the wrong question; the seam
    **breaks** (sound non-interference needs whole-program undecidable analysis — the trilemma recurs);
    and a *new* axis appears with no analog in the old domain (**declassification discipline**). The
    lesson: an impossibility result and its cost-basis are usually **domain-parameterized**. The Seam
    Trilemma is the same *shape* in memory safety, information flow, and (one expects) real-time, liveness,
    or distributed consistency — but a *different basis* in each. So "the basis is closed" is never the
    last word; "**closed in *this* domain; here is the next domain, and the method transfers**" is.

    **And one refinement deeper — the floor-domain match.** Asking the obvious follow-up — does the new
    domain have a *closing* seam, the way memory safety has SPSC? — the fourteenth primitive (a
    capability-gated declassification, the IFC analog of "concentrated checked `unsafe`") tried to build
    one and **broke, structurally**: to compute on a secret you must hand a closure the bare plaintext (the
    contaminant escapes uncaught — needs an effect system), and an affine capability counts *uses* where
    the danger is *bits* (needs quantitative information flow). The reason is general: **a closing seam
    exists only when the *floor* carries the *domain's* precondition.** Rust's affine/borrow floor is
    *matched* to memory safety — affinity *is* the cardinality precondition, so SPSC closes for free — and
    *mismatched* to information flow — affinity bounds resource uses, not information amount, so *no* IFC
    seam closes on it.

    **And one refinement deeper still — the match is GRADED.** A fifteenth primitive tested the *positive*
    half in a *third* domain — **liveness/deadlock-freedom**, whose precondition (a strictly-decreasing
    global lock order) *looks* like exactly the partial order Rust's trait resolver carries. It **broke
    too — but only partly**, and the partial break is the finding. The floor carries the precondition along
    *two* separable axes, and fails on both: **granularity** — a re-entrant `root` mints a second token
    lineage, so the per-*token* discipline never witnesses the per-*thread* held set (Rust has no ambient
    per-thread one-shot resource); and **coverage** — the *dynamic* fragment (runtime-ordered, same-rank
    locks: the bank-transfer and hand-over-hand idioms) escapes the *static* floor, just as IFC's
    quantitative coordinate escaped the affine one. So the full shape is **four-layered**: the trilemma is
    **domain-general**, its cost-basis is **per-domain**, *which seams close* depends on the **floor-domain
    match**, and that match is **graded — full / partial / none** (memory safety / liveness / information
    flow).

    **And the last refinement — the grade is a LADDER, not a knob.** A sixteenth primitive tried to lift
    liveness's *partial* match to *full* by upgrading the floor — replacing the static type-level order with
    a *runtime* `thread_local!` registry (lockdep-in-miniature). It **broke too**, and the break is the
    deepest single lesson: **upgrading the floor is non-monotone.** The runtime registry *did* fix the
    coverage gap (dynamic orderings now work) and *did* pay the predicted price — #5 bare-metal (it cannot
    be `no_std`), #6 a runtime check, and a prevention→detection modality. But the **granularity** gap was
    not fixed; it *recurred one rung finer*: the held-set is keyed on the *thread*, while async *tasks* are
    the unit of control, so work-stealing migration launders an undetected inversion. The "granularity"
    axis turns out to be a **containment-scope ladder** — *token < thread < task < …* — and each ambient
    floor (affine token, thread-local) reaches exactly one rung; Rust has no ambient *task-local*. So you do
    not buy a partial match up to full by picking a better floor — you climb a step, fix one fragment, and
    the gap reappears a rung higher. The most honest way to finish a map is to draw its border, point past
    it, name the floor the next country needs — *grade how much of it your current floor reaches, and warn
    that the floor above merely relocates the gap.*

    **And a sideways refinement — the floor can be wrong in a way *no* better floor fixes.** A seventeenth
    primitive held the floor fixed and changed the *precondition*: "this finalizer must run." Rust's floor
    is **affine** (use *at most* once); "runs *exactly* once" is **linear** — so a safe `mem::forget` skips
    the destructor and the obligation drops outright (the leakpocalypse). This broke too, but in a *new
    mode*: not a graded partial, not a wrong domain, but the **wrong structural logic** — every sound
    *decidable* fix collapses to a viral `Leak`/linear bound, so no floor *upgrade* helps (linearity was a
    horn Rust's scoped-task trilemma deliberately rejected). Two lessons fall out. The floor-domain match is
    really a floor-**precondition** match: the *same* liveness domain grades deadlock-freedom *partial* and
    must-run-cleanup *none*. And — the deepest process lesson of the whole project — **a seam designed to
    break cannot be given a coherent "usable" spec; the incoherence *is* the break**, surfacing wherever you
    paper over it, which is exactly what a cold adversary is *for*.

11. **A probe can survive its soundness gauntlet and still have its *headline* deflated — the adversary
    refutes framing, not just soundness.** The twenty-third primitive built *the real binary session* the
    eighteenth could not — two concurrently-live dual endpoints over `Rc<RefCell>` — explicitly to remove a
    prior deflation ("not a binary session, just a single-handle walk"). It **held**: order and *per-session*
    progress close, completion costs horn 3, no UB, the #18 grading transfers. Yet every "BREAKS" attacker
    granted soundness and instead refuted the *story*: relative progress closes only **per session** (interleaved
    sessions deadlock and type-check — global deadlock-freedom needs *multiparty* typing); the single-threaded
    witness lifts only the *static* limitation, not the real concurrency `session_types` (2015) already had; the
    forced-fallible `recv`'s `Err` is **non-actionable**, because the very `mem::forget` that costs completion
    *also leaks the `Rc`*, denying any abandonment signal (a *double-bind*, not a fixable API). The lesson: when
    you build "the real thing" to answer a critic, point the cold adversary at the *claim*, not only the code —
    a primitive can pass "is it sound?" and fail "is it what you said it was?" The honest yield was not the
    headline ("real binary session built") but the two sharpenings the deflation forced (per-session-only
    progress; the leakpocalypse double-bind). The instrument is calibrated when it can ratify your soundness and
    reject your sales pitch in the same verdict.

12. **The rejection mode fires twice — and the second time it corrects your *map*, using your own citations.**
    A sound seam can still **fail** its gauntlet when its *deliverable* overclaims; this is the rejection mode,
    seen first at `typed_nat` (#8, a sound seam whose claimed eighth axis was rejected). The twenty-fourth
    primitive — **delegation** (channel-over-channel), built to answer #23's "no delegation, so it's #18 doubled"
    — fired it again. The mechanism is sound (SEAM-COSTS [horn 3], no UB), but I pre-registered a *closed-basis
    confirmation*: "delegation adds no new coordinate; its hazards project onto leak-freedom." The cold panel
    refuted that **from the very lineage I cited**: delegation is the *recognized* feature that breaks
    first-order sessions' automatic deadlock-freedom (it spawned the global-progress literature —
    Coppo–Dezani–Yoshida–Padovani; Kobayashi; deadlock-free GV), so its signature hazard is **progress /
    deadlock-freedom**, a coordinate *distinct* from completion and leak-freedom. I had seen the `Rc` hazard's
    memory-*leak face* and mislabeled the progress *axis* — an error I could sustain only because a
    single-threaded witness *definitionally cannot deadlock*. The lesson: building "the real feature" can
    **refute your map, not just confirm it** — and the sharpest refutations come from reading your own
    references more honestly than your hypothesis did. (The liveness domain thus has *progress* as a third
    coordinate: the lock seams #15/#16 were its *lock* face, delegation its *session* face.)

---

# Part 12 — Do it yourself: exercises and a reading path

**Run the artifacts (30 minutes).**
- `cd seam && cargo test` — read `tests/*.rs` (the sound uses) and the `compile_fail` doctests
  in `src/*.rs` (the laundering attempts the floor rejects). Try to *make one compile* — feel the
  borrow checker push back.
- `lean proof/SeamGap.lean` — read the characterization; change a definition and watch it break.
- Skim `design_seam/REPORT_SEAM_SPEC_*.md` — the cold attackers' actual verdicts, in their words.

**Build a twenty-eighth primitive — the fourth-domain door looks hard to open, and the twenty-seventh just
*salvaged* #26's priority break (horn 1, the eighth broken seam) by sealing the public constructor — holding
*soundly* (the deadlocking ring now unconstructible) yet **rejected a fourth time**, splitting progress into a
*topology* half the constructor concentrates and a *completion/execution* half that never leaves the consumer.** (The packaging trilogy is *closed*: #18 found the horn moves, #19 that it moves
within a fence, #20 that the fence is *forced* — see below; a further packaging probe will only re-confirm. And
**both** fourth-domain attempts failed to open a fourth domain: #21, **distributed/replicated convergence** via
a state-based CRDT, **deflated** — it breaks floor-relatively but folded into the *generic
unchecked-typeclass-law* gap, projecting onto `restore`'s application-correctness basis; and #22, **real-time /
WCET** via a cost-typed computation, **FAILED** outright — the op-count is an *unsound* proxy for cycles, the
construct is Danielsson's 2008 cost monad, and it deflates onto `typed_nat` #8 + an uninstalled `mmio`-shaped
axiom + liveness's partial-match. Then #23, a **real binary session** — the very next move proposed below — was
*built* and **held** (SEAM-COSTS [horn 3]), deepening liveness rather than opening a domain: it removed #18's
single-handle limitation and confirmed the component-wise grading transfers, but the panel deflated the "real
binary session" framing — relative progress closes only *per session* (interleaved sessions deadlock; global
deadlock-freedom needs *multiparty* typing), and the fallible `recv`'s `Err` is non-actionable because
`mem::forget` leaks the `Rc` (the leakpocalypse double-bind). *Two* fourth-domain candidates, *two* non-domains,
by different mechanisms — the meta-finding is that the three confirmed domains (memory safety / information flow
/ liveness) look **special**, and a would-be fourth keeps reducing
to an existing basis or failing on an unsound proxy. And #24, **delegation** (channel-over-channel) — the very
next move proposed below — was *built*; its mechanism **held** (SEAM-COSTS [horn 3], no UB), but the panel
**rejected its deliverable** (the rejection mode, a second time after #8): my pre-registered "delegation projects
onto leak-freedom, no new coordinate" claim was refuted *from my own cited lineage* — delegation is the
recognized feature that *breaks* first-order sessions' automatic deadlock-freedom, so it opens **progress /
deadlock-freedom** as a coordinate *distinct* from completion and leak-freedom, which I had mislabeled as a leak
because a single-threaded witness cannot deadlock. So liveness now carries *three* coordinates, not two.) The
memory-safety basis is closed; a within-memory-safety probe will
*project*, not extend. Primitives 13–21 opened the live frontiers. Primitive
13 showed information flow is a *distinct*
domain; 14, that *no IFC seam closes on Rust's affine floor*; 15, that liveness **half-closes** on a static
type-level order (a *partial* match); 16, that *upgrading* that floor to a runtime registry is
**non-monotone** (it fixes coverage but granularity *recurs* a rung finer — a containment-scope **ladder**);
17, breadth-wise, that a *different precondition* in the same liveness domain (must-run cleanup) gets a
*different grade* — a clean **none** — under a **new verdict mode**, the *wrong structural logic* (affine vs
linear), which **no floor upgrade fixes**; 18, that the *same* gap can be a *cost* not a break depending on
how the rule **packages** it; 19, that this packaging dial is **real but bounded** — the horn moves only
where soundness *decouples* from the danger, and is **forceable** where it does not; and 20, that the fence is
**forced** — the *same* packaging over a *borrowed slot* (vs #19's owned value) breaks horn 1, the clean
single-variable confirmation #19's confound could not give; 21, that the first **fourth-domain** attempt
(replicated convergence) *deflated* into the generic unchecked-typeclass-law gap; 22, that the second
(real-time/WCET) **FAILED** — the op-count is an unsound proxy for cycles (Danielsson's cost monad), opening
no new domain either; and 23, that a **real binary session** *deepened* liveness — it **held**, removing #18's
single-handle limitation and confirming the grading transfers, but its "real session" framing deflated to
*per-session-only* progress (interleaved sessions deadlock; global deadlock-freedom needs *multiparty* typing)
and a non-actionable `recv` `Err` (the leakpocalypse double-bind); and 24, that **delegation**
(channel-over-channel) opened the **progress / deadlock-freedom** coordinate — its mechanism *held*
(SEAM-COSTS [horn 3]) but its deliverable was **rejected** (the rejection mode's second firing after #8),
refuting my "projects onto leak-freedom" claim from my own cited lineage, since delegation is the recognized
feature that breaks first-order sessions' automatic deadlock-freedom; and 25, that **multiparty session types**
*deepened* that progress coordinate without opening new ground — built from #23's binary edges, a source-less
`ring()` **type-checks a globally-deadlocked system**, the seam **held** (SEAM-COSTS [horn 3], sound) but its
deliverable was **rejected a third time**: my "global progress lives *only* in a generative whole-protocol
projection" claim was a *false dichotomy*, refuted from my own miscited lineage by **per-channel priority typing**
(Kobayashi; Dardha–Gay) that rejects the ring with no global type — restoring a **three-rung** taxonomy (per-edge
floor / per-channel priority typing + whole-program acyclicity / generative MPST) in which global progress is a
*whole-program*, not per-call, obligation; and 26, that **per-channel priority typing** — the rung-(b) floor
#25 named — *broke on soundness* (horn 1), the **eighth broken seam**: it rejects the ring only when a trusted
constructor threads one shared priority per edge, but the public `role()` lets three identically-typed relays
compile a deadlocking ring (Rust checks each `Pi: Lt<Po>` in isolation, no transitive reasoning), the same
*wrong-granularity* break as `ordered_locks` #15/#16 one sub-domain over. So the twenty-eighth has two honest
directions and a warning (the third, *forcing the horn*, the twentieth already did) — but the fourth-domain
door is looking hard to open. (i) **Deepen liveness further — and #24 named where it bites.** The binary
session shipped only a *straight-line `i32`* fragment, and delegation (#24) confirmed that the *fragment* — not
the single handle — was the real limit: the session-type features still unbuilt are the obvious probes —
`offer`/`choose` (external/internal choice, where the continuation *type depends on a received value*) or
recursion — **both since built: choice as #28, recursion as #31** (the latter *sound but its deliverable rejected*, the rejection mode's seventh firing, on a guardedness over-concession — I filed the non-contractive `Rec<Var>` = `µX.X` as undecidable when a sealed `Guarded` marker carries it in Θ(type-size), so *only termination* is the genuine undecidable NONE; the **#30-bounding finding holds**: `Rec<Tx<Var>>` has no reachable `End`, so #30's `Done`-forcing is vacuous). Sharper still, #24 *promoted progress to a coordinate*: binary duality gives only *per-session*
progress, and delegation is exactly the feature that breaks even that — so **multiparty session types**
(Honda–Yoshida–Carbone; Kobayashi's deadlock typing; deadlock-free GV) were the *named* floor a *global*
deadlock-freedom seam would need — and **#25 built it; the answer is *neither cleanly*.** A multiparty global
type (rung c) closes global progress by construction, but the per-edge floor does *not* (a source-less `ring()`
type-checks a deadlock), and the cold panel corrected my claim that the generative projection is its *only*
home — **per-channel priority typing** (Kobayashi; Dardha–Gay FoSSaCS 2018) closes it with *no global type at
all*. **#26 built that rung-(b) floor — and it *broke* (horn 1).** Per-channel priority typing rejects the ring
*only* when a trusted constructor threads one shared priority per edge; the public `role()` lets three
identically-priority-typed relays compile a deadlocking ring, because Rust checks each `Pi: Lt<Po>` bound in
*isolation* and does no transitive reasoning — edge-priority consistency is the constructor's invariant, not a
type fact (the same *wrong-granularity* break as `ordered_locks` #15/#16, one sub-domain over). **And #27 built
that salvage — sealed `worker()`, vended only the trusted `relay`/`relay4` builders — and the answer is
*relocate, not close*: it *held soundly* (the ring now unconstructible), confirming the humble **SEAM-COSTS
[horn 3]** result #26 named, but its deliverable was **rejected a fourth time** — "deadlock-freedom by
construction" is really *schedulability* by construction, splitting progress into a *topology* half the
constructor concentrates (no cycle — the #26 fix) and a *completion/execution* half that never leaves the
consumer (the #17/#18/#23 residue at multiparty scale).** So the frontier re-points to three honest
directions: (a) a seam that actually *forces* the run-all/in-order/no-drop residue #27 left on the caller — a
**linear / must-use-with-teeth discipline over the minted roles** (the same relevant/linear gap Primitive 17
exposed, now over a multiparty bundle), testing whether the completion half is *carriable at all* or is
irreducibly the consumer's; (b) the *fragment* limit #24 named — **recursive / choice (`offer`/`select`)
protocols**, where the continuation type *depends on a received value* (*both since built — choice #28, recursion #31 — sound but deliverables rejected; recursion bounds #30, its `Rec<Tx<Var>>` having no reachable `End`*); and (c) **dynamic-`N` / async
multiparty**, where the static, fixed-`N`, single-threaded witness that propped up #25/#26/#27 finally gives
way to work-stealing migration and a genuinely concurrent cyclic-wait hang. (Grading
a genuinely *fourth* domain is settled-hard: convergence deflated to application-correctness; WCET's op-count is
an unsound proxy that reduces to a cost monad.) (ii) **Probe another STRUCTURAL LOGIC** (linear was the first
*wrong-structural-logic* verdict): a **relevant**-type precondition — *must-use-at-least-once with an error
channel*, i.e. fallible must-run, the exact gap Primitive 17's infallible scope exposed — or an **ordered /
session-typed** protocol where the floor's substructural logic is again the mismatch. Write the spec + a
checked toy, run the *same*
harness (`python3 -m orchestrator.seam_phase --spec ... --gauntlets 1 --prior-art-web`). Predict, because
the seven out-of-domain/precondition probes got them all (the sixth, convergence, by *deflating*, and the
seventh, WCET, by *failing* on an unsound proxy — both real outcomes: not every candidate domain is one):
(a) the Rust Abstract Machine sees *nothing*; (b)
the Seam Trilemma *recurs* with the domain's own bite; (c) the domain has its *own* axes; (d) **a seam
closes only if your floor matches the precondition**; (e) **the match is graded** (full/partial/none), and
the unit is the *precondition*, not the whole domain; (f) **upgrading the floor is non-monotone** *and* may
be the **wrong structural logic** — measure whether a better floor *closes* the gap, merely *relocates* it,
or *cannot* close it at any decidable price. And one warning Primitive 17 earned: **a seam designed to break
may keep FAILING the spec on coherence — that is the break expressing itself, not a fixable defect** (don't
chase a clean pass for a primitive that breaks; record the verdict and the residual incoherence as evidence).
The deliverable is **"the next domain/logic's basis, the floor that closes it *and how completely*, where the
next gap reappears or why none can, and confirmation the method transfers."** (See
`design_seam/REALIZABILITY.md` for the memory-safety lattice, the closed-basis capstone, the per-domain
generalization, the graded floor-domain match, the non-monotone floor-upgrade, and the wrong-structural-logic
mode.)

**Reading path, in dependency order:**
1. *The Rust Book*, ownership/borrowing chapters — the affine system in practice.
2. *Programming Language Foundations* (Pierce et al., *Software Foundations* vol. 2) — soundness
   (progress + preservation) done properly, mechanized.
3. The **RustBelt** paper (Jung et al., POPL 2018) — the semantic-soundness idea; skim the Iris
   parts, absorb the *unsafe-is-justified-not-checked* thesis.
4. **GhostCell** (Yanovski et al., ICFP 2021) — the branding trick, and why it carries aliasing
   but not lifetime.
5. **Generativity, end to end** — Launchbury & Peyton Jones, "Lazy Functional State Threads"
   (PLDI 1994, the `runST` brand); Kiselyov & Shan, "Lightweight Static Capabilities" (PLPV 2006,
   branded indices for bounds-check elision, two decades early); Beingessner ("Gankra"), *You Can't
   Spell Trust Without Rust* (MSc thesis, Ch. 6.3); bluss's `indexing` crate. This lineage *is* the
   fifth primitive, and the source of its API-locality cost.
6. **Refinement types** — **Flux: Liquid Types for Rust** (Lehmann, Geller, Vazou, Jhala, PLDI 2023,
   the brand-free predicate witness *on Rust's own floor*) and **Liquid Haskell** (Vazou et al., ICFP
   2014). This pair *is* the sixth primitive, and the source of its solver-in-the-TCB cost — the
   founding tradeoff named back in DML (Xi & Pfenning 1998).
7. **Reference counting and the `core`/`alloc` line** — Rust's **RFC 2480** (the `alloc` crate split,
   stable 1.36) and the **`heapless`** crate (core-only, fixed-capacity containers incl. a pooled
   `Rc`). This pair *is* the seventh primitive's axis: the bare-metal boundary, and the proof that
   shared ownership trades *unbounded dynamic allocation* against *caller-provided bounded storage*.
8. **Verus** and **Creusot** docs — what "Rust but `unsafe` is proof" actually looks like when
   someone ships it (the productive horn from Part 8).
9. This repo's `WRITEUP.md` (the full result), `design_seam/SYNTHESIS.md` (the twenty-four-primitive map),
   `design_seam/REALIZABILITY.md` (the face lattice / 3-of-N proof), and `THE-SAME-WALL.md` (the
   cross-field boundary/veil discipline).

**The one thing to remember.** Decidable, sound, expressive, seamless, zero-cost — pick your
sacrifices on purpose. The seam isn't a failure of your design; it's the place your design tells
the truth about which difficulty it chose not to delete.
