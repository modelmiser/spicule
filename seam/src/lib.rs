//! # One seam — an SPSC lock-free ring buffer
//!
//! This crate is the **foundation-author side** of a single seam. It contains the
//! *only* `unsafe` block in the design. It re-exports a dangerous lock-free primitive
//! (a single-producer / single-consumer ring buffer) as a **checked safe interface**:
//! [`RingBuffer::split`] hands out two affine endpoint handles, and *safe* consumer code
//! uses [`Producer::push`] / [`Consumer::pop`] with no `unsafe` and no proofs.
//!
//! The point of the exercise (see `../design_seam/SEAM_SPEC_spsc.md`) is to see how many
//! of the primitive's true preconditions the **stipulated NLL floor** (real `rustc`)
//! carries *for free*, versus how many stay trusted inside this `unsafe` block.
//!
//! ## What the floor carries (so it is NOT trusted here)
//! - **Cardinality (P4):** `Producer`/`Consumer` are affine (no `Clone`/`Copy`). Safe code
//!   cannot mint a second producer ⇒ SPSC holds structurally.
//! - **Storage lifetime (no UAF):** `split(&'a mut self)` reborrows the unique `&mut` as a
//!   shared `&'a` given to both handles, so NLL forces the buffer to outlive both handles.
//!   Use-after-free is a *borrow error*, not a runtime hazard.
//! - **Payload thread-safety (P5):** `Send` is implemented only `where T: Send`, so moving
//!   a `T` across the producer→consumer thread boundary is checked by trait resolution.
//!
//! ## What stays trusted inside `unsafe` (P1–P3)
//! Per-slot exclusion, the Release/Acquire publication (happens-before), and the modular
//! index discipline. These live in a concurrent separation logic no decidable certificate
//! captures; the foundation author discharges them once, here.
//!
//! ## Re-import — safe code cannot launder UB (compile-checked)
//!
//! Cloning a handle (would break SPSC cardinality):
//! ```compile_fail
//! # use seam_spsc::RingBuffer;
//! let mut rb = RingBuffer::<u32, 4>::new();
//! let (p, _c) = rb.split();
//! let _p2 = p.clone(); // ERROR: Producer is affine — no Clone
//! ```
//!
//! Touching the buffer while handles are alive (would alias the storage):
//! ```compile_fail
//! # use seam_spsc::RingBuffer;
//! let mut rb = RingBuffer::<u32, 4>::new();
//! let (mut p, _c) = rb.split();
//! let (_p2, _c2) = rb.split(); // ERROR: cannot borrow `rb` mutably while handles live
//! p.push(1).ok();
//! ```
//!
//! Letting a handle outlive its buffer (would be a use-after-free):
//! ```compile_fail
//! # use seam_spsc::{RingBuffer, Producer};
//! fn escape() -> Producer<'static, u32, 4> {
//!     let mut rb = RingBuffer::<u32, 4>::new();
//!     let (p, _c) = rb.split();
//!     p // ERROR: `rb` does not live long enough
//! }
//! ```
//!
//! Sending a non-`Send` payload across the seam (would race `T`'s interior):
//! ```compile_fail
//! # use seam_spsc::RingBuffer;
//! use std::rc::Rc;
//! let mut rb = RingBuffer::<Rc<u32>, 4>::new();
//! let (p, _c) = rb.split();
//! std::thread::spawn(move || { let _p = p; }); // ERROR: Rc<u32> is not Send
//! ```
//!
//! And the sound use *does* type-check — see `tests/sound_use.rs`.

#![forbid(unsafe_op_in_unsafe_fn)]
// The crate is `#![no_std]` — and that is load-bearing for the seventh seam's axis. Seams 1–6
// (SPSC, MMIO, slab, intrusive, branded, refined) are `core`-only: they need no heap, no
// allocator, no runtime. The `extern crate alloc` below exists **solely** for `shared` (seam 7),
// the one primitive whose sound re-import requires a runtime memory manager (a global allocator +
// refcount lifecycle). Delete `pub mod shared` and you can delete `extern crate alloc`; the other
// six still compile. That compile-enforced fact *is* the seventh axis: bare-metal / runtime-freedom.
#![no_std]
extern crate alloc;

// The sixteenth seam alone needs OS thread-local storage (a runtime lock-order registry), so it
// links `std` — gated behind the `lockdep` feature. The base crate stays `no_std`; this `extern
// crate std` is compiled in *only* with `--features lockdep`, making the bare-metal (#5) cost of a
// runtime registry visible in the build graph rather than hidden in prose.
#[cfg(feature = "lockdep")]
extern crate std;

/// Second seam (Phase C contrast): an MMIO read-to-clear register. Memory-safe seam
/// closes to O(1) width like SPSC — but its soundness rests on an external physical-world
/// axiom (M1/M4) no checker can discharge. See `mmio::StatusReg`.
pub mod mmio;

/// Third seam (Phase C contrast): an intrusive doubly-linked list. The affine re-import that
/// closed SPSC is structurally inapplicable (nodes alias); the sound mechanism (GhostCell
/// branding) threads a token through ordinary consumer code — seam width O(accesses), a
/// measurable horn-3 drop. See `intrusive`.
pub mod intrusive;

/// Fourth seam (Phase C contrast): a generational slab allocator. The handle (`Key`) is
/// `Copy`, so affinity can't track its validity; the danger (stale/ABA keys) is resolved by a
/// runtime generation check. Keeps all three trilemma horns but spends the zero-cost property
/// (#6) — the constructive analog of `RefCell`. See `slab`.
pub mod slab;

/// Fifth seam (Phase D — the 3-of-N probe): branded indices / sound unchecked indexing. The
/// first primitive here to *ace* zero-cost (#6) — bounds checks are elided, the brand erases —
/// yet it still pays, on a new axis: the brand `'id` is a viral generative type parameter that
/// infects consumer signatures and traps indices in a `scope` closure. Seam width is O(1), so the
/// cost is neither runtime nor a seam crossing: a fifth axis (brand-freedom / API-locality). See
/// `branded`.
pub mod branded;

/// Sixth seam (Phase E — closing the #5⟂#7 cell): refinement-typed bounds (Flux / Liquid Haskell
/// style). The witness is a *visible predicate* `value < bound` carried by a plain, brand-free value
/// that can be stored and returned — the exact opposite corner from the branded index. It occupies
/// (brand-free ✓, legible ✓), and paired against the slab (brand-free, illegible) it proves
/// AI-legibility (#7) is independent of API-locality (#5). Its distinctive price is the
/// external-solver / one-toolchain cost — a refinement floor needs an SMT solver in the trusted
/// loop — which un-bundles horn 2 the way the branded index un-bundled horn 3. See `refined`.
pub mod refined;

/// Seventh seam (Phase F — the bare-metal axis): reference-counted shared ownership (a minimal
/// `Rc`). A genuinely new precondition shape — *dynamic shared ownership*, where the set of live
/// owners is data-dependent — that affinity (one owner), NLL (static lifetimes), and the slab's
/// generation check (single-owner pool) all structurally cannot express. Sound and brand-free and
/// concentrated, but its witness is a **runtime memory manager** (global heap + refcount lifecycle):
/// it is the one seam that needs `alloc`. That isolates the seventh axis — **bare-metal /
/// runtime-freedom (seed #5)** — confirmed independent of zero-cost (#6) by the off-diagonal against
/// the slab (both runtime-costly; only `shared` needs a manager). See `shared`.
pub mod shared;

/// Eighth seam (Phase G — the checker-tractability axis): a type-level *proof* of in-bounds
/// indexing. Same danger as the branded/refined seams (`get_unchecked`), a third witness: the
/// in-bounds fact is the trait bound `I: Lt<Len>`, discharged by the compiler's resolver and fully
/// erased — no brand, no value, *and no external solver*. It is the off-diagonal that un-bundles horn
/// 2 a second time: where the refined index is decidable-but-solver-bound, this is solver-free but its
/// checker halts only under `#![recursion_limit]` (resolution depth = the index value). That isolates
/// **checker-tractability** — distinct from both decidability and solver-freedom — making horn 2 a
/// product of *three* faces. See `typed_nat`.
pub mod typed_nat;

/// Ninth seam (Phase H — the unwind-safety probe): take-and-replace across a possible unwind. A
/// different danger — *time*, not aliasing or bounds: moving a value out of a `&mut T` and back leaves
/// the slot holding a duplicate during the gap, and an unwinding panic drags the caller through that
/// gap into a double-drop. The sound witness is an **abort-bomb** (`take_mut`'s pattern): a mid-take
/// panic aborts before the broken slot is observed. The seam aces all seven prior axes and pays only on
/// **unwind-safety** (exception safety — Abrahams; Rust's `UnwindSafe`), the clean off-diagonal the
/// `typed_nat` probe could not produce. See `take`.
pub mod take;

/// Tenth seam (Phase I — the leak-freedom probe): drain-and-yield with a leaked guard. The danger is
/// the canonical *leakpocalypse* — `mem::forget` is safe, so a seam whose soundness leans on a guard's
/// `Drop` must survive that `Drop` never running. The sound witness is **pre-pessimization**
/// (`Vec::drain`'s trick): empty the buffer up front, so a leaked guard loses the tail but never
/// double-frees. Free-standing (no closure → aces API-locality, unlike the scoped-thread witness), it
/// aces all eight prior axes and pays only on **leak-freedom** — a ninth-axis probe, sibling to but
/// distinct from unwind-safety (leak = `Drop` never runs; unwind = `Drop` runs at a bad time). See `drain`.
pub mod drain;

/// Eleventh seam (Phase J — the move-stability probe): a self-referential cell. A new danger class —
/// a value whose **address is load-bearing** (it holds a pointer into itself), so *moving* it dangles
/// the self-pointer. None of the nine confirmed axes is about address stability. The floor's witness is
/// `Pin` (a `!Unpin` cell, stack-pinned to stay `core`-only). The open question the spec poses: is
/// move-stability a clean tenth axis, does it *reduce* to API-locality (the `Pin` wrapper is viral), or
/// is it **real-but-structurally-inseparable** (no free-standing witness exists) — a new kind of
/// verdict. See `selfref`.
pub mod selfref;

/// Twelfth seam (Phase K — the closed-basis test): a checked bit-pattern. A new danger class — a
/// **validity invariant** (a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even without
/// a deref) — re-exported as a `transmute`-after-check. Built not to hunt a fresh axis but to test
/// whether the cost-basis is *closed*: does a new danger add a coordinate, or project onto an existing
/// one? The prediction (and the spec's §6 question): it deflates to **zero-cost** (a runtime tag check),
/// the dual of the `get_unchecked` family. See `validity`.
pub mod validity;

/// Thirteenth seam (Phase L — the *different-domain* probe): a constant-time secret. The first primitive
/// aimed *outside* memory safety: its danger is an **information-flow / timing leak** (branching on a
/// secret), not UB, and it has **no `unsafe` keyword** at all — the foundation is plain arithmetic, and
/// constant-time-ness is a *trusted* property the compiler does not guarantee (a TCB axiom, like MMIO's,
/// but for a security guarantee). Built to ask whether the closed cost-basis is closed *absolutely* or
/// only *within memory safety* — i.e. whether IFC is a different domain with its own basis. See `secret`.
pub mod secret;

/// Fourteenth seam (Phase M — a *closing* information-flow seam): capability-gated declassification. The
/// second IFC-domain primitive. Where the secret seam (`secret`) **broke** because declassification was
/// etiquette, this one makes it **type-structural**: a `Secret<T>` can be read only by spending an affine
/// `Cap` minted at the trust boundary, so *where/when/how-often* a secret is declassified is bounded and
/// concentrated by the type system — the IFC analog of "concentrated, checked `unsafe`". It tests whether
/// the IFC domain has a **closing** seam (paying *declassification discipline* — a clean IFC axis with no
/// memory-safety analog), confirming the domain has its own cost-basis. See `labeled`.
pub mod labeled;

/// Fifteenth seam (Phase N — a *third* domain): type-level lock ordering. The first primitive in the
/// **liveness / progress** domain — its danger is **deadlock** (a cyclic lock-acquisition wait), which is
/// neither UB nor an information leak; a deadlocked program is memory-safe and confidential, it just never
/// progresses. The witness is a type-level lock *rank* plus an affine `Floor` capability. It tested the
/// **positive** half of the floor-domain match — and **broke** (the third seam to drop a horn): a
/// re-entrant `Floor::root` mints a second token lineage, so safe code types a lock-order *inversion*
/// (wrong **granularity** — per-token, not per-thread), and even patched, the dynamic-acyclicity fragment
/// (runtime-ordered/same-rank locks) escapes the static floor (wrong **coverage**). The break **grades**
/// the floor-domain match into full (memory safety) / partial (this) / none (information flow). See
/// `ordered_locks`.
pub mod ordered_locks;

/// Sixteenth seam (Phase O — the *floor-upgrade* probe; feature `lockdep`): a **runtime** lock-order
/// registry (lockdep-in-miniature). Same liveness domain and same danger as `ordered_locks`, but on an
/// *upgraded floor*: a `thread_local!` held-stack of **runtime** ranks, checked on each acquire. This
/// targets the static seam's two gaps — **coverage** (runtime ranks express *dynamic* orderings —
/// bank-transfer `min/max`, hand-over-hand) and **granularity** (thread-local ⇒ per-thread). It tested
/// whether *upgrading the floor lifts the match* — and **broke** (the fourth seam to drop a horn): the
/// upgrade is **non-monotone**. Coverage lifted and it paid the predicted price (#5 bare-metal, #6 runtime
/// check, prevention→detection modality), but **granularity recurred one rung finer** — the held-set is
/// keyed on the OS *thread*, but async *tasks* are the control unit, so work-stealing migration launders an
/// undetected inversion. "Granularity" is a containment-scope **ladder** (token < thread < task) no ambient
/// Rust floor tops out. See `lock_registry`.
#[cfg(feature = "lockdep")]
pub mod lock_registry;

/// Seventeenth seam (Phase P — a *breadth-wise* probe): a must-run cleanup guard. A new precondition —
/// **"this finalizer MUST run"** (a *linearity* requirement: the guard must be *consumed*, not merely
/// droppable) — re-exported as a RAII scope-exit guard. It is the **inverse** of the tenth seam
/// (`drain`): where `drain` *closed* by making a skipped `Drop` harmless (leak-**tolerance**), this one
/// needs a skipped `Drop` to be impossible (leak-**intolerance**) — the same ninth-axis danger from the
/// opposite side. Rust's floor is **affine** (use *at most* once), so `core::mem::forget` / `ManuallyDrop`
/// / leaking — all **safe** — skip the destructor (the leakpocalypse, RFC 1066 / 1084); enforcing a
/// must-run finalizer needs **linear** types (use *exactly* once) the floor does not have. The break is
/// compiler-settled; the spec poses the open rulings (the **grade** of the affine floor against a
/// linearity precondition; whether the liveness domain hosts multiple preconditions of different grades;
/// that this is the *enforcement* face of axis 9, not a new axis). See `linear`.
pub mod linear;

/// Eighteenth seam (Phase Q — the *composite-precondition* probe): a session-typed channel. A `Chan<P>`
/// affine handle whose type parameter is the *remaining protocol* (`Tx`/`Rx`/`End` markers); each operation
/// consumes the handle and returns the successor state. Built as the crucible for the two prior liveness
/// findings — #15 (`ordered_locks`: affine typestate carries static **order**) and #17 (`linear`: the affine
/// floor cannot compel **completion**/linearity) — to ask whether the floor matches a *composite*
/// precondition **component-wise**. The hypothesis the spec poses: protocol **safety** (no out-of-order op)
/// *closes* via typestate — an out-of-turn call is a type error, and there is no `root` to re-enter (the
/// granularity hole that sank #15 has no analog) — while protocol **completion** *breaks* on the same #17
/// linearity gap (a mid-protocol `Chan` is `mem::forget`-able in safe code, abandoning the session). If so it
/// is the first primitive whose sub-component cleanly closes while the whole breaks. See `session`.
pub mod session;

/// Nineteenth seam (Phase R — the *controlled packaging* probe): a value-owning scope-restore guard (the
/// real `scopeguard::guard(value, f)` + `into_inner`, the shape #17's `MustRun` deliberately declined). It
/// carries the **identical** un-compellable-finalizer danger as the seventeenth seam (`mem::forget` skips
/// the closure) but wraps it in the **eighteenth seam's structure**: a sound affine-ownership core (the
/// guard owns and vends scoped access to a `T`, no `unsafe`, no UAF) beside which the restore closure is a
/// mere *side-obligation*. Built to turn Phase Q's headline — *which horn a danger threatens is a function
/// of how the re-import rule packages it* — from an **observation** (across the different primitives #17 and
/// #18) into a **controlled experiment** (change the packaging, hold the danger fixed). The cold panel (r26)
/// ruled **SEAM-COSTS [horn 3]** (survived): the `mem::forget` gap that **broke** #17's horn 1 here only
/// **costs** horn 3 (an un-restored logical invariant — a lost lease, a stale global — never UB) — the
/// packaging dial is **real**. But the panel **bounded** the thesis: the move is *confounded* (#19 also adds
/// the value-ownership core #17 lacked, so the value-ownership model co-varies), and the horn is *forceable*
/// — the same danger in a borrowed-slot/`take_mut` packaging forces horn-1 UB, and `Pin`/`thread::scope`
/// show un-repackageable horns. The refined finding: which horn a danger threatens is a design choice
/// **within a structural envelope** set by whether soundness can be **decoupled** from the un-compellable
/// closure (owned ⇒ costs; borrowed-across-unwind ⇒ forces). Not novel (`JoinGuard` horn-1 / `scopeguard`
/// horn-3, RFC 1066); contribution = the method + locating the dial. N stays 9 modes; the 5 broken seams
/// unchanged (this holds). See `restore`.
pub mod restore;

/// Twentieth seam (Phase S — *forcing the horn*, the Phase-R fence): a take-and-reseat guard over a
/// **borrowed slot** (`take_mut` in guard form). The **controlled counterpart** to #19 (`restore`): it keeps
/// #19's packaging **byte-for-byte identical** — a guard with `Deref`/`DerefMut`/`into_inner`/Drop-restore —
/// and changes **only** the decoupling property, owned value → borrowed slot, the single-variable experiment
/// #19's confound forbade. Strikingly, the guard form is **unwind-safe** (Drop reseats during unwinding — no
/// abort-bomb, unlike the eager ninth seam `take`) and **forget-safe** (`mem::forget` leaks the mutation but
/// the slot keeps valid original bits — a leak, not a double-free). The horn-1 break is **isolated to
/// `into_inner`**: the API that was *sound* for #19's owned value (one owner) is a **double-free** over a
/// borrowed slot (it returns `owned` while the slot keeps the read duplicate — two owners). The cold panel
/// (r27) ruled **SEAM-BREAKS [horn 1], FAILED — the sixth broken seam**: a 4-line safe `String` witness
/// (`new`/`push_str`/`into_inner`) double-frees with no `unsafe` in user code. Two findings: (a) the
/// single-variable horn flip is **clean** (cost 3 → break 1, packaging byte-identical) — Phase R's confound
/// is **repaired**, the flip is attributable to the decoupling property alone; (b) the fence is **FORCED, not
/// movable** — my spec's *movable* hedge was rejected: the `into_inner`-less "sound" subset is **degenerate**
/// (it deletes the by-value move-out that defines `take_mut`, strictly dominated by plain `&mut T`), so every
/// *usable* packaging breaks, and the fence sits at Phase R's **owned-vs-borrowed** line. Deflation
/// (pre-registered): no novel mechanism (`take_mut` + `scopeguard`-`into_inner` + RFC 1066); contribution is
/// analytical. 20 primitives; **6 broken seams** (was 5); 9 verdict modes unchanged. The refined finding
/// closing the trilogy: the horn is movable iff the ownership model lets soundness DECOUPLE from the
/// un-compellable closure (owned can ⇒ cost; borrowed cannot ⇒ forced break). See `reseat`.
pub mod reseat;

/// Twenty-first seam (Phase T — the *fourth-domain* probe): a state-based CRDT merge. The first primitive
/// aimed at a fourth soundness domain — **distributed/replicated convergence** (eventual consistency) — after
/// memory safety (match), information flow (mismatch), and liveness (partial). Its danger is **replica
/// divergence** (two replicas that observed the same updates disagree), neither UB nor leak nor deadlock nor
/// timing channel. Like the constant-time `secret` seam it has **no `unsafe`**: the danger is semantic. A
/// `Replica<T>` merges via `T: JoinSemilattice`; convergence holds iff `join` is commutative + associative +
/// idempotent — laws the affine/ownership floor **cannot enforce** (a safe non-conforming `join` type-checks
/// and diverges replicas, exactly as the floor enforced IFC's syntactic proxy but not non-interference). The
/// cold panel (r28) ruled **SEAM-BREAKS [floor-mismatch]** *floor-relatively* (the `T: JoinSemilattice` bound
/// is a syntactic proxy, not a witness — a safe lawless `join` type-checks and diverges replicas; closing it
/// is undecidable by Rice) — but scored a **TRADEOFF survival**, because the candidate fourth domain
/// **DEFLATED**: the gap is the *generic* unchecked-typeclass-law phenomenon (Rust checks the *signatures* of
/// `Ord`/`Eq`/`Hash`/`Monoid`/semilattice, none of their *laws*), so convergence **projects onto `restore`'s
/// application-correctness basis** rather than opening a fourth soundness domain. Two further corrections: the
/// blessed-CRDT TCB rescue is **not MMIO-shaped** (the lattice space is open-ended → law-trust recurs per
/// user CRDT, never amortizes), and 'dischargeable' holds only at the *model* level (no Rust pass checks
/// model-to-shipped-code). The domain map stays at **three** (memory/IFC/liveness) + the application-
/// correctness basis. 21 primitives; FAILED broken-seam tally stays 6; N stays 9 modes. See `crdt`.
pub mod crdt;

/// Twenty-second seam (Phase U — the *second* fourth-domain probe): a cost-typed computation, targeting
/// **real-time / worst-case execution time (WCET)** after the convergence probe (`crdt`) deflated. Its danger
/// is a **deadline miss** (a computation runs past its cycle budget) — neither UB, leak, deadlock, timing
/// side-channel, nor divergence. A cleaner candidate than convergence because a cycle budget is a
/// *quantitative resource bound*, not an algebraic law. No `unsafe`: the danger is the semantic mismatch
/// between a static cost and real time. A `Timed<C, T>` tracks a static op-count `C` (the *exact* sealed
/// `Nat`/`Lt` machinery of `typed_nat`, reused on purpose) and `within::<D>` proves `C ≤ D` at compile time —
/// but the op-count is an **unsound proxy** for real cycles: a single `step` can wrap a million-iteration loop
/// or a cache-missing division, so "op-count ≤ deadline" does not imply "cycles ≤ deadline". The cold panel
/// (r29) ruled **SEAM-BREAKS [horn 1], FAILED — the seventh broken seam**: `step` charges +1 regardless of
/// work, so `within::<1>` admits a million-cycle computation (my §4 over-claimed the deadline guarantee). All
/// five attackers landed: Horn 3 is *false* (non-unifying `if`-branches force manual step-padding); the unary
/// cost doesn't *scale* (O(C) blow-up, uncompilable at real budgets); no additive composition; and the
/// primitive is **Danielsson's 2008 cost monad** — "op-count ≠ cycles" is WCET's *founding premise*, so "pick
/// a lane" (abstract cost xor physical deadline). WCET does **not** open a fourth domain — it deflates (cost
/// monad / `typed_nat` #8 for the static part, an `mmio`-shaped-but-*uninstalled* axiom #2 for the physical
/// claim, liveness's partial-match for data-dependent loops). Meta: **two** fourth-domain candidates probed
/// (`crdt` deflated+survived, `wcet` deflated+FAILED), **neither opens a fourth domain** — the three domains
/// (memory/IFC/liveness) look special. 22 primitives; FAILED broken-seam tally rises to 7; N stays 9. See `wcet`.
pub mod wcet;

/// Twenty-third seam (Phase V — the *real binary session*, deepening liveness): two dual endpoints, both
/// concurrently live, sharing the wire through `Rc<RefCell<Transport>>`. NOT a fourth-domain bid — after both
/// fourth-domain candidates (`crdt`, `wcet`) deflated, this stays in the **third confirmed domain (liveness)**
/// and removes #18's most-cited limitation: the r25 panel's deflation #1 was "*not a binary session — a
/// single-handle typestate walk*" (one `&mut Transport` can't split into two live endpoints, `E0499`), so
/// #18's "a peer waits forever" hazard was inexpressible (only *self*-abandonment). Here the duality is real,
/// at the cost of `alloc` + interior mutability. Three structural shifts: (1) `recv` is forced **fallible**
/// (`Result<…, Self>`) — the value comes from a peer who may never send, so the completion gap surfaces in the
/// signature #18 hid it from; (2) mutual-wait deadlock is **foreclosed by duality typing** (`HasDual`: if one
/// peer is `Tx`, its dual is `Rx` — they cannot both block — the binary-session *relative*-progress theorem, a
/// NEW positive the single-handle walk couldn't show); (3) abandonment starvation is **not** foreclosed — a
/// peer's safe `mem::forget` starves its dual *forever*, now genuinely **peer-observable** (a second party
/// waits), where #18's non-completion was only self-observable. No `unsafe` (the danger is liveness; `Rc`/
/// `RefCell` mediate sharing — the worst case is a starved peer, not UB). The cold panel (r30) ruled
/// **SEAM-COSTS [horn 3], TRADEOFF** — it HOLDS, and #18's component-wise grading SURVIVES (order closes,
/// *per-session* progress closes, completion costs horn 3) — but **deflated** the "real binary session"
/// framing to "#18 with a duplicated handle and a `RefCell` between the copies": relative progress closes
/// only **per session** (two interleaved `session()`s type-check a mutual-wait deadlock — global
/// deadlock-freedom needs multiparty/Kobayashi typing), the single-threaded witness makes "peer waits
/// forever" self-abandonment-flavored (only the **static** two-live-endpoint limitation is lifted, not
/// `session_types` 2015's real concurrency), and the fallible `recv`'s `Err` is **non-actionable** because
/// `mem::forget` leaks the `Rc` (the leakpocalypse double-bind: the gap that costs completion also denies an
/// actionable abandonment signal). N stays 9; liveness stays the third domain; the FAILED tally stays 7
/// (this survived). See `binary_session`.
pub mod binary_session;

/// Twenty-fourth seam (Phase W — DELEGATION, channel-over-channel): the higher-order session step #23's
/// straight-line `i32` witness lacked (the r30 expressiveness attacker named it as *the* feature distinguishing
/// session types from coupled typestate). A `Chan<TxChan<Q, C>>::send_chan(c: Chan<Q>)` hands off a whole live
/// session as a payload; `RxChan<Q, C>::recv_chan` receives it. Still a LIVENESS probe, testing whether the
/// higher-order step opens new ground or deflates onto the basis. The floor carries: order (affine typestate)
/// and **don't-use-after-delegate for free** (the affine *move* consumes the delegated handle — exactly the
/// linearity delegation needs here). It does NOT carry: completion (the #17/#18/#23 horn-3 gap, now
/// **three-party** — the delegatee can `mem::forget` a session it never contracted to complete, starving the
/// third party); and **leak-freedom** — the new hazard: the payload is now an `Rc<RefCell<Transport>>`, so
/// abandoning a carrier with a delegated channel *in flight* leaks the payload session (the canonical
/// `Rc<RefCell>` leak, leakpocalypse / #9 / #10, *amplified* through delegation). The cold panel (r31) ruled it
/// **SOUND (SEAM-COSTS [horn 3]) but FAILED the §6 deliverable** — the REJECTION mode (2nd after `typed_nat`
/// #8): my "no new coordinate / projects onto leak-freedom" claim was **refuted from my own cited lineage**.
/// Delegation's signature hazard (B forgets `Chan<Q>`, starving C) is "*deadlock at the other end*" (Jespersen
/// et al. 2015) — **progress / deadlock-freedom**, a coordinate *distinct* from completion and leak-freedom;
/// delegation is the *recognized* feature that breaks first-order sessions' automatic deadlock-freedom
/// (Coppo–Dezani–Yoshida–Padovani; Kobayashi; deadlock-free GV). I saw the `Rc` hazard's leak-face and
/// mislabeled the progress axis; the single-threaded witness hides deadlock by construction. So the liveness
/// domain has progress as a third coordinate (the lock seams #15/#16 were its *lock* face; delegation its
/// *session* face). Sound, so the FAILED broken-seam tally STAYS 7 (a deliverable rejection, not a soundness
/// break); N stays 9; Lean `delegation_handoff_core_closes`. See `delegation`.
pub mod delegation;

/// Twenty-fifth seam (Phase X — MULTIPARTY session, n-party progress): the global-coherence step #23 conceded
/// and #24 named. #23's deflation #1 granted that "*global deadlock-freedom is out of reach of binary session
/// types — it needs multiparty/Kobayashi typing*"; #24 named **progress/deadlock-freedom** as a coordinate
/// distinct from completion and leak-freedom. This seam builds the multiparty thing from #23's binary edges —
/// deliberately, since the argument is that **multiparty progress is not reducible to the binary duality of the
/// edges it is built from**. A `RecvThenSend` role typestate enforces recv-before-send; [`multiparty::relay`]
/// mints a coherent `A→B→C` (a source role exists → it completes) and [`multiparty::ring`] mints an incoherent
/// `A→B→C→A` (every role recv-first → no source → no terminating schedule). Both come from the identical
/// per-edge floor; only the global topology differs, and the floor cannot see it: **`ring()` type-checks a
/// globally-deadlocked system**. The cold panel (r32) ruled it **SOUND (SEAM-COSTS [horn 3]) but FAILED the §6
/// deliverable** — the REJECTION mode's **3rd firing** (after `typed_nat` #8 and `delegation` #24), and again
/// **refuted from my own cited lineage**. My claim — global progress lives *only* whole-protocol in a
/// *generative* global-type projection, no per-role discipline able to reject the ring — was a **false
/// dichotomy**: I omitted the middle rung, **per-channel priority/obligation typing** (Kobayashi; Padovani;
/// Dardha–Gay FoSSaCS 2018; Kokke–Dardha), which rejects the source-less ring *compositionally, with no global
/// type*, via `prio(recv) < prio(send)` → `p < p` on the cycle. I even **mis-filed Kobayashi as *support* when
/// he is the *counterexample*.** The true taxonomy has **three rungs**: (a) per-edge `HasDual` floor [doesn't
/// carry]; (b) per-channel priority typing + whole-program acyclicity [no global type]; (c) generative MPST
/// projection. What survives: global progress is **not concentrable per-call** and needs strictly more than the
/// per-edge floor — but it is a **whole-program** obligation, not necessarily a generative one. Per-role order +
/// linearity CLOSE as in #23 (Lean `multiparty_role_order_core_closes`). Sound, so the broken-seam tally **stays
/// 7**; N stays 9 (rejection, 3rd firing); liveness stays the third domain; progress its third coordinate (lock
/// seams #15/#16 = lock face, delegation = session face, multiparty = global-topology face). See `multiparty`.
pub mod multiparty;

/// Twenty-sixth seam (Phase Y — PRIORITY-TYPED sessions, the rung-(b) probe): builds the exact rung #25's
/// rejection handed back. #25 (`multiparty`) staked a false dichotomy (global progress lives *only*
/// whole-protocol in a generative projection); the panel named the omitted middle rung — **per-channel
/// priority/obligation typing** (Kobayashi; Dardha–Gay FoSSaCS 2018) — which rejects the source-less ring with
/// no global type. This seam encodes it: over #23's binary edges and #8's sealed `typed_nat::Lt` (Peano `<`,
/// no reflexive `Lt<N>`), a forwarder [`priority::Relay`]`<Pi, Po>` recv-then-sends under the bound `Pi: Lt<Po>`.
/// The deadlocking ring — which #25's bare floor *type-checked* — forces `p_CA < p_AB < p_BC < p_CA`, i.e.
/// `p: Lt<p>` — **but only when one `Nat` is shared per edge**, which I assumed and did not enforce. The cold
/// panel (r33) ruled **SEAM-BREAKS [horn 1] — the EIGHTH broken seam**: I exposed the public `role::<Pi, Po>`
/// constructor carrying only the *local* bound `Pi: Lt<Po>`, so a caller wires three `role::<N1, N2>` relays
/// (every bound `N1: Lt<N2>`, 1 < 2, discharges) into the ring `A→B→C→A` — it **compiles, no `unsafe`, and
/// deadlocks**, falsifying §4.3's "deadlock-freedom of any well-typed assembly." Edge-priority consistency is a
/// trusted-constructor invariant (`relay` threads one `Nat`/edge), **not a per-endpoint type fact** — the spec
/// admitted this and shipped it anyway; the soundness attacker quoted my own words. This **confirms** #25's
/// weaker thesis by BREAKING not costing: the per-channel `Lt` discipline carries **nothing global** (it never
/// names the dual endpoint); deadlock-freedom rests entirely on the trusted edge-consistent constructor — a
/// mini-projection. **Same break as `ordered_locks` #15/#16, one soundness sub-domain over** (type-level order
/// carries the local bound; whole-program consistency/acyclicity escapes — the wrong-granularity gap): progress's
/// *lock face* and *session face* break the same way. Salvages to SEAM-COSTS [horn 3] only if `role()` is hidden.
/// N stays 9 (SEAM-BREAKS [horn 1] is one of the nine); liveness stays the third domain; Lean
/// `priority_typed_discipline_factors` (a `..._factors`, like `ordered_locks`). See `priority`.
pub mod priority;

/// Twenty-seventh seam (Phase Z — PROJECTED sessions, the #26 salvage): deadlock-freedom by construction. #26
/// (`priority`) broke (SEAM-BREAKS [horn 1]) because its per-role constructor `role()` was public — a caller
/// wired locally-valid roles into a deadlocking ring. The r33 verdict named the real carrier of deadlock-freedom
/// as the *trusted, edge-consistent constructor*, not the per-channel `Lt` discipline. This seam makes that
/// explicit: it **seals** the per-role constructor (`worker()` is module-private — the keystroke #26 skipped) and
/// vends only trusted builders ([`projected::relay`], [`projected::relay4`]) that thread one strictly-increasing
/// `Nat` per edge. Two mechanisms compound: (1) **access** — no public `worker()`, so the #26 exploit is
/// uncompilable; (2) **structure** — a monotone-priority chain cannot close into a ring (the closing edge needs a
/// decreasing `Lt` step, no instance). So the danger is unconstructible. The cold panel (r34) ruled **SEAM-COSTS
/// [horn 3] — SOUND** (every attacker conceded Horn 1: the #26 ring is unconstructible) **but FAILED the §6
/// deliverable** — the REJECTION mode's **4th firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25;
/// #26 was a break). My §4 overclaimed "deadlock-freedom by construction" + "the consumer carries nothing
/// global." The genuine price: the builder proves a deadlock-free schedule *exists* (a static `Nat` witness), but
/// affine Rust **never forces the runtime to follow it**, so a **progress residue** — *run-all, in-order,
/// no-drop* — stays on the untrusted consumer. Honest phrasing: "**schedulability** by construction, given all
/// roles execute." Concentration is **partial**: the constructor carries the *topology* obligation (no cycle —
/// the genuine #26 fix); the *completion/execution* obligation never left the consumer (the #17/#18/#23 residue —
/// Scribble/`mpstthree` are deadlock-free only assuming every participant executes, which I silently imported).
/// (Secondary: the expressiveness cost was *over*stated — acyclic generality is recoverable via one generic
/// combinator, as `mpstthree` proves.) The refined finding: the **progress coordinate decomposes** —
/// deadlock-freedom/topology (a trusted constructor can carry) vs completion/execution (the consumer always
/// carries). Sound, so the broken-seam tally STAYS 8; N stays 9; Lean `projected_schedulability_closes` (a
/// `..._closes`, the first since #25). See `projected`.
pub mod projected;

/// Twenty-eighth seam (Phase AA — CHOICE, offer/select, `&`/`⊕`): the last unprobed core session-type feature —
/// the *branching* connective, orthogonal to the *sequencing/progress* axis every prior session seam (#18,
/// #23–#27) probed. Self-contained over #23's `Rc<RefCell<Transport>>` dual endpoints. `Offer<L, R>` (external
/// choice) receives a label and branches; `Chan::offer` returns a [`choice::Branch`]`<L, R>` sum the consumer
/// must `match`. `Select<L, R>` (internal choice) sends a label; `select_left`/`select_right` commit to a branch.
/// `HasDual` threads choice: `Offer<L, R> ↔ Select<L::Dual, R::Dual>` (the selector's options are exactly the
/// offerer's branches). VERDICT (cold panel, r35): SEAM-COSTS [horn3-concentration], gauntlet **FAILED** on the §6
/// deliverable — the **REJECTION mode, 5th firing** (after #8/#24/#25/#27, all sound + rejected). Sound (no UB),
/// the cost is **horn 3**; my §4 over-claimed concentration as TOTAL again (the recurring trap, 5th). Four panel
/// corrections: (1) branch-safety is **two-legged** (label–continuation correlation + branch-set duality), not
/// three — the false guarantee is **#2 "handle every label"**: exhaustiveness gives *scrutinee coverage, not
/// run-to-`End`*, so a safe `match { Left(c) => .., _ => {} }` **drops** the other branch (order is the generic
/// typestate leg, not branch-specific); (2) it deflates onto **binary `Either`**, NOT n-ary *named* sum types
/// (labeled n-ary choice needs nesting that drops names, or `offer!`/`choose!` macros); (3) Horn 2 is
/// **Θ(type-size)**, not Θ(depth) (2^depth leaves, but interned linear-in-source); (4) completion is **amplified**
/// per-branch and **peer-choice-reachable**, not "unchanged by branching." Core thesis survives, sharpened: **no
/// new coordinate**; completion stays the #17/#18/#23/#27 residue, now fed by two safe paths (`mem::forget` AND
/// the dropped branch). N = 9, broken-seam tally stays 8, liveness stays the third domain. Prior art CONFIRMS
/// (Honda 1993's `&`/`⊕`; the `session_types`/JML 2015 affine-vs-linear gap IS this residue). Lean
/// `choice_branch_typing_closes` (renamed from `..._branch_safety_closes`: branch *typing* closes, not
/// *completion*). See `choice`.
pub mod choice;

/// Twenty-ninth seam (Phase AB — MUST-COMPLETE SESSION): the session connective set is exhausted, so this seam
/// stops probing features and attacks the **completion residue** itself — the leftover every session seam
/// (#17/#18/#23/#27/#28) carries and I over-claimed as carried five times. A `Session<P>` is an affine handle
/// with a `Drop` **completion bomb** (cf. `take` #9): drop a `live` mid-protocol session and it aborts, so the
/// floor *appears* to compel run-to-`End`. The danger is the *remote* twin of `linear` #17's *local*
/// must-run-cleanup: a `mem::forget`-ten session starves its **peer**. VERDICT (cold panel, r36): SEAM-COSTS
/// [horn3-concentration], gauntlet **FAILED** the §6 deliverable — the **rejection mode, 6th firing**, and the
/// most instructive: it **reverses** my finding. The free-handle bomb IS bypassable (`mem::forget`, confirmed),
/// but my "universal NONE" is **REFUTED**: a **scoped-closure / CPS combinator** (`with_session(|s| …)`, the
/// **`std::thread::scope` analogue**) **compels completion modulo divergence, no `unsafe`**, by moving the
/// obligation to the re-import boundary so the consumer never holds a forgettable handle. STING: I cited
/// `thread::scoped`'s removal (RFC 1084) but ignored its fix, `thread::scope` (stable 1.63). So completion is
/// **NOT irreducible — it is CONCENTRABLE** at a horn-3 **API-locality** cost (surrender the free handle): the
/// whole #17–#28 residue was the price of the **free-handle API**. Sessions ≠ #17's cleanup (a *delimited* session
/// CAN be scoped; open-ended cleanup cannot — refuting claim (a)). Ergonomics: the bomb turns `?` into a
/// process-abort (unusable on fallible code). Prior art: peer starvation is fixable by **cancellation
/// propagation** (Mostrous–Vasconcelos *Affine Sessions*; ECOOP 2022 *Stay Safe Under Panic*; MultiCrusty; EGV).
/// META (inverts my usual error): I overstate the *extremes* — total-carry or irreducible — when the truth is a
/// cost between. Tally **STAYS 8** (COSTS, not the BREAK I predicted); N stays 9; liveness third domain. Lean
/// `must_complete_scoped_completion_closes` (renamed from `..._typed_discipline_factors`: the scoped witness
/// *closes* completion at a cost; the free-handle witness only factored). See `must_complete`.
pub mod must_complete;

/// Thirtieth seam (Phase AC — the SCOPED SALVAGE of #29, mirroring #26→#27): the concrete `with_session`
/// combinator the #29 reversal named. #29 (`must_complete`) forced completion with a `Drop` **bomb** and the r36
/// panel reversed it — the bomb is `mem::forget`-able, yet completion is **concentrable** via a **scoped-closure /
/// CPS** witness (the `std::thread::scope` analogue). This seam builds it: there is **no free `start`**; the only
/// way to obtain a [`must_scoped::Session`] is [`must_scoped::session_scope`], which hands two **dual** role
/// closures a brand-trapped endpoint each and **requires each to return a [`must_scoped::Done`]** — a completion
/// certificate mintable **only** by [`must_scoped::Session::finish`] at `End`. So a body's only fates are
/// **complete** (finish ⇒ `Done`), **diverge** (`loop`/`panic` ⇒ never returns), or **nothing** — it cannot
/// return without a `Done` and cannot fabricate one (`mem::forget` conjures no certificate; the invariant
/// `'scope` brand — lifted verbatim from branded #5 — blocks smuggling a foreign one). The destructor obligation
/// of #29 (forgettable) becomes a **return-type** obligation (not forgettable). VERDICT (cold panel, r37):
/// **SEAM-COSTS [horn3-concentration], TRADEOFF — HELD (survival 1/1)**, like `restore` #19 / `binary_session` #23
/// (NOT the rejection mode #29 was). **The #29 reversal is CLOSED as predicted:** every attacker conceded **no safe
/// `Done` is forged, smuggled, or forgotten** — the return-type obligation defangs `mem::forget` (it mints no
/// certificate) — so completion *is* concentrable and the #17–#28 residue **was** the price of the free-handle API.
/// Horn 1 (no UB) and Horn 2 (Θ(depth), no SMT) intact. The panel **sharpened** it (merits-strongest, soundness):
/// concentration is **PER-ENDPOINT, not whole-session** — my §2 over-claimed "every session runs to `End`," but
/// `session_scope` forces only each *returning* closure's own endpoint; under run-`a`-then-`b`, **`a` finishes while
/// `b` diverges, starving the peer**, so global completion **escapes via peer divergence** (the #23/#27
/// per-session-vs-global gap, one step on). I hedged this correctly in the "per-endpoint"/"modulo divergence"
/// residue, so the **caveat is the finding**; secondary, the **synchronous-trap** cost (sequential run-`a`-then-`b`,
/// no genuine concurrency) was understated. META (consistent with #29): I overstated the extreme again — there
/// "irreducible," here "whole-session global"; the truth is the per-endpoint cost between. Deflation (conceded): no
/// new mechanism — `thread::scope` + generativity (branded #5) + #29's `finish`/`End` typestate. No new
/// domain/coordinate/mode; tally **STAYS 8**; N stays 9. Lean `must_scoped_completion_closes` (a `..._closes`:
/// completion closes through the scoped witness, *per-endpoint*; the free-handle #29 witness only factored). See
/// `must_scoped`.
pub mod must_scoped;

/// Thirty-first seam (Phase AD — RECURSION, the last unbuilt structural session feature): single-level recursive
/// sessions (`Rec<P>`/`Var`, `μX.T`), the looping connective orthogonal to sequencing (#18/#23), choice (#28), and
/// completion (#29/#30). A `Session<P, E>` threads a type-level recursion environment `E` (default `NoRec`):
/// [`recursion::Session::enter`] unfolds `Rec<P>` (setting `E := Rec<P>`), and [`recursion::Session::recur`] exists
/// **only** for `Session<Var, Rec<P>>` — so a top-level `Var` (env `NoRec`) has **no `recur`** and is inert, the
/// recursion analogue of branded #5 trapping an index in its scope (`Var` sound by construction, no UB).
/// `HasDual` threads recursion (`Rec<P> ↔ Rec<P::Dual>`, `Var ↔ Var`), so the infinite producer `Rec<Tx<Var>>` is
/// dual to the infinite consumer `Rec<Rx<Var>>`. THE SHARP POINT: without a choice connective inside the loop,
/// single-level recursion is **necessarily non-terminating** — `Rec<Tx<Var>>` has **no reachable `End`** — so (1)
/// **#30's completion-forcing is VACUOUS** here (a `Done` is mintable only at `End`; with no `End`, the scoped
/// salvage cannot force anything — recursion exposes #30's unstated precondition, a reachable terminal, and turns
/// its "modulo divergence" caveat into the whole protocol), and (2) **leaving a loop needs CHOICE (#28)** — a
/// terminating loop is `Rec<Offer<End, …Var>>`, which the floor types but **cannot force the consumer to ever exit**
/// (livelock = the halting problem). VERDICT (cold panel, r38): **SEAM-COSTS [horn3-concentration], FAILED — the
/// rejection mode's 7th firing** (after #8/#24/#25/#27/#28/#29). Sound, so the broken-seam tally **STAYS 8**. What
/// HELD: Horn 1 — the `Var`/environment encoding is sound (no safe ill-formed unfolding; worst case a starved peer,
/// not UB); Horn 2 — decidable (Θ(type-size)); and the #30-bounding finding (no reachable `End` ⇒ #30's
/// `Done`-forcing vacuous). THE REJECTION (I OVER-CONCEDED): I filed **productivity/guardedness** as an undecidable
/// NONE alongside termination — but it is a **DECIDABLE COST**. `Rec<Var>` = µX.X is the **non-contractive** type
/// every system I cited (Coq/Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**;
/// a sealed **`Guarded` marker** (impl for Tx/Rx/End/Rec, **not** Var) bounding `enter`/`Rec`-formation carries it
/// in Θ(type-size), no SMT — the SAME machinery as order. So guardedness **CONCENTRATES (horn 3)**; **only
/// TERMINATION is the genuine undecidable NONE** (halting). Refuted from my own cited lineage again (#24/#25/#29
/// pattern). META — the **inverse** of the arc's usual over-claim, same shape: prior rejections over-claimed
/// concentration *total* and #29/#30 over-claimed the danger *irreducible/global*; here I **over-conceded** (the
/// undecidable extreme) when the truth was a **decidable cost between** — I flee the middle toward whichever extreme
/// is cleaner. Corrected: order + duality + **guardedness** CLOSE; termination alone is the NONE. No new
/// domain/coordinate/mode; N stays 9. Lean `recursion_guarded_order_closes` (renamed from
/// `recursion_order_unfolding_closes`: guardedness joins the closing core). The natural **#32** is the concrete
/// salvage — add the sealed `Guarded` marker so `Rec<Var>` will not form — mirroring #26→#27, #29→#30. See
/// `recursion`.
pub mod recursion;

/// Thirty-second seam (Phase AE — GUARDED RECURSION, the salvage of #31): builds the exact fix the r38 panel
/// handed back. #31 (`recursion`) was SOUND but its deliverable was REJECTED because I **over-conceded** —
/// productivity/guardedness is not an undecidable NONE but a **decidable cost**. This seam carries it: a sealed
/// [`guarded::Guarded`] marker (impl for `Tx<C>`/`Rx<C>`/`End`, **not** `Var`) bounds [`guarded::Session::enter`],
/// so the unguarded `Rec<Var>` = µX.X (the non-contractive spin loop) **cannot be unfolded** — rejected at compile
/// time in Θ(type-size), no SMT, the **same machinery as order**. The key simplification: for single-level
/// recursion a **shallow head-check IS full contractivity** — if the body's head is an action (`Tx`/`Rx`),
/// everything inside (incl. any `Var`) is under it ⇒ guarded; `End` is vacuously guarded; a bare `Var` head is
/// unguarded ⇒ rejected — so `Guarded` need only be impl'd for the three heads, no recursive trait walk. HYPOTHESIS
/// VERDICT (cold panel, r39): **SEAM-COSTS [none], SURVIVED (1/1) — the salvage HELD.** `[none]` is the cleanest
/// result in the arc: all three horns hold, no horn dropped (stronger than #30's `[horn3]`). It **closes #31's
/// over-concession** — guardedness/productivity IS concentrable, the decidable cost #31 owed — and I **avoided the
/// #27 salvage-over-claim trap** by claiming only productivity and conceding termination up front. What HELD: no
/// safe unfold of `Rec<Var>` (sealed marker, depth-1 resolution), no UB (Horn 1), zero-ceremony concentration
/// (Horn 3), decidable no-SMT (Horn 2), no in-scope false positive. What the panel **MARKED** (tradeoffs, NOT
/// horn-drops): **completeness is bought by RESTRICTION** — the shallow head-check = full contractivity *only
/// because* the single-level, choice-free grammar collapses it to "head ≠ `Var`"; a richer grammar (nested `Rec`,
/// or choice-in-recursion `Rec<Offer<End,…Var>>`) needs the real recursive contractivity walk, which this seam
/// **scopes out, not escapes**; and my "folklore" deflation slightly overstated generality. META: the "flee the
/// middle" tic surfaced small even here (I wrote "full contractivity" for what is full contractivity *for the
/// single-level fragment*) — but as a marked caveat on a seam that HELD, not a horn-deciding over-claim. Final split
/// (the #31→#32 reject→salvage rhythm, mirroring #26→#27, #29→#30): order + duality + **guardedness/productivity**
/// all CLOSE; **termination alone** is the conceded undecidable NONE. No new domain/coordinate/mode; N stays 9;
/// tally STAYS 8 (SURVIVED). Lean `guarded_recursion_productivity_closes` (a `..._closes`: guardedness concentrates;
/// the restriction caveat + termination stay in the prose). See `guarded`.
pub mod guarded;

/// Thirty-third seam (Phase AF — TOTAL(-ISH) DIVISION, the first pivot OFF the session family): the fresh
/// **fourth-domain probe** — **panic-freedom / totality** (SPARK / Frama-C "absence of run-time errors"). Integer
/// `/` panics on **two** independent inputs: div-by-zero (`b == 0`) and signed overflow (`i32::MIN / -1`). This
/// seam re-exports division through a [`divisor::Divisor`] certified **nonzero** by a single check at construction
/// ([`divisor::Divisor::new`] → `None` on `0`), witnessed thereafter by the type — so div-by-zero becomes
/// **unwritable** ([`divisor::divide`] cannot even be called with a raw integer; see CF1). Crucially the witness
/// is [`core::num::NonZeroI32`], the one refinement type stock Rust SHIPS — so this fragment closes on the **real**
/// floor, no SMT and no hypothetical solver (unlike the refined index #6). VERDICT (cold panel, r40):
/// **SEAM-COSTS [horn3], SOUND — but the DELIVERABLE is REJECTED (the rejection mode's 8th firing, after
/// #8/#24/#25/#27/#28/#29/#31).** SURVIVED: the domain finding — panic-freedom **FOLDS** into the
/// application-correctness basis (#19/#21), NOT a fourth soundness domain (a panic is defined behaviour, never UB;
/// prior-art confirms the fold as the historical default — SPARK's division/overflow split, LH nonzero-`div`;
/// expressiveness granted it). So the **third** fourth-domain probe deflates (crdt fold, wcet break, divisor fold).
/// THE REJECTION — I **OVER-CONCEDED**: `divide` overflows on **exactly** `a == i32::MIN && d == -1`, whose
/// divisor-side conjunct lives on the side I already refine, so a witness `d ∉ {0,-1}` makes division **total for
/// every `a`** via the same `Θ(1)` check — no range analysis, no SMT. My §2/§4 "overflow needs undecidable
/// whole-program range analysis" is **false for division**: a decidable corner I *declined*, not an irreducible
/// one. "One source closes, one escapes" is WRONG — **BOTH** close; by returning `Result` I force `.unwrap()` on
/// provably-total divisions, reintroducing the panic the seam exists to remove. META: the **SAME tic as #31**,
/// inverted onto a new domain — over-CONCEDE a decidable cost as undecidable (fleeing the middle toward the cleaner
/// extreme). MARKED tradeoffs (not horn-drops): concentration overstated (value-threaded divisors only;
/// path-condition-nonzero still needs a runtime `Option`); ergonomic tax + `.unwrap()` re-panic. Salvage **#34**
/// (mirroring #31→#32): a `TotalDivisor` (`d ∉ {0,-1}`) vending total `-> i32` division. Tally STAYS 8 (sound —
/// deliverable rejection); N stays 9. Lean `divisor_nonzero_division_closes` (a `..._closes`: the div-by-zero
/// fragment closes; the owed `-1` exclusion + the rejection live in the prose). See `divisor`.
pub mod divisor;

/// Thirty-fourth seam (Phase AG — TOTAL DIVISION, the salvage of #33): builds the exact fix the r40 panel handed
/// back. #33 (`divisor`) was SOUND but its deliverable was REJECTED because I **over-conceded** — the `i32::MIN /
/// -1` overflow is not an undecidable range-analysis residue but a **decidable point-exclusion**. This seam carries
/// it: a [`total_div::TotalDivisor`] minted only by [`total_div::TotalDivisor::new`] (`None` iff `d == 0 || d ==
/// -1`) excludes **both** of division's panic sources at one `Θ(1)` construction check, so
/// [`total_div::total_divide`]/[`total_div::total_remainder`] return a plain `-> i32` that **cannot panic for any
/// `a: i32`** — no `Result`, no `.unwrap()`, no panic path (the thing #33 could not express). A *finite* exclusion
/// suffices because division's panic set is finite on the divisor side (`{0, -1}`); and excluding `-1` costs
/// nothing safe (`a / -1 == -a`, and the only overflowing `a == i32::MIN` is exactly where `-a` overflows too).
/// VERDICT (cold panel, r41): **SEAM-HOLDS [none], SURVIVED (1/1) — the salvage HELD**, the cleanest verdict in the
/// project (a full HOLD, no horn even costed, stronger than #32's SEAM-COSTS [none]). Soundness + implementation +
/// expressiveness all ARGUED-PAST (conceded): both panic sources sit in the excluded `{0,-1}`, so a `Θ(1)` witness
/// makes `total_divide`/`total_remainder` total for all `a` — no panic path. CLOSES #33's over-concession:
/// division's overflow was a **decidable point-exclusion**, not undecidable range-analysis. Dodged the #27
/// over-claim trap (all three non-claims confirmed). The `#33→#34` reject→salvage pair settles — the **fourth**
/// (after #26→#27, #29→#30, #31→#32). MARKED tradeoffs (ergonomic/novelty, NOT horn-drops): (1) the honest baseline
/// is `i32::checked_div`/`checked_rem` (which I OMITTED from prior-art) — already total, serving `a/-1` directly, so
/// "excluding -1 costs nothing safe" holds at the CAPABILITY level but not ergonomically vs `checked_div`; §3 argued
/// a raw-`/` strawman; (2) the genuine niche is narrow — the amortized reusable RUNTIME divisor; (3) folklore-thin
/// (core::NonZero + one excluded value = Ada-2012 predicate subtypes + SPARK checks). Tally STAYS 8; N stays 9;
/// rejection count STAYS 8 (survival). Lean `total_div_both_sources_closes` (a `..._closes`: the total-division core
/// closes). See `total_div`.
pub mod total_div;

/// Thirty-fifth seam (Phase AH — SHARED MUTABILITY ACROSS THREADS, the FOURTH fourth-domain probe): a deliberate
/// stress-test of the "3-domain map (memory/IFC/liveness) is complete" claim, probing **data-race freedom /
/// concurrency** — the property Rust's `Send`/`Sync` auto-traits enforce. [`sync_cell::SyncCell`] wraps an
/// `UnsafeCell` with a non-atomic `&self` get/set, plus the dangerous re-import: an **unconditional `unsafe impl
/// Sync`/`Send`** asserting a `&SyncCell` is safe to share across threads. THE KEY OBSERVATION: data-race freedom
/// sits at a *different structural place* than every prior domain — IFC's floor checks NOTHING (seams break by
/// omission), memory-aliasing's floor checks via borrows (seams concentrate `unsafe`), but data-race freedom is
/// **already closed by the floor's auto-derived `Send`/`Sync`** (Rustbelt-verified; a `Cell<i32>` is `!Sync`
/// precisely to forbid cross-thread sharing — CF1). So a re-import has nothing to add: the only SOUND interface
/// bounds `T: Sync`/uses atomics (= the floor, vacuous), and the only NON-trivial re-import is the `unsafe impl`
/// escape — an unchecked TCB axiom (MMIO-shaped). VERDICT (cold panel, r42): **SEAM-BREAKS [horn1]** — the break is
/// real (safe `thread::scope` races the non-atomic `set`, UB, no consumer `unsafe`), tally **8→9** (but a
/// pre-CONCEDED folklore break, the probe's vehicle, not a discovery). **JUDGE → FAILED the CONTRIBUTION:** my
/// staked classification — "no value-adding concurrency seam / DRF is floor-closed / a re-import is categorically an
/// unchecked axiom" — is **REFUTED** by an obvious omitted seam: a sound `core`-only **`SpinMutex<T>`** whose
/// `unsafe impl<T: Send> Sync` is **DISCHARGED by the lock invariant** (not false like `SyncCell`'s) — the
/// concurrency **slab**. CORRECTION: distinguish *discharged* from *false* `unsafe impl`s; reclassify data-race
/// freedom as **"floor-PROPAGATES-but-does-not-close," analogous to borrowck/memory-aliasing** — the auto-trait
/// propagates `Send`/`Sync` but closure comes from concentrating seams (locks/channels/atomics). So my trichotomy
/// (IFC / memory / concurrency-special) collapses to a **dichotomy** (concurrency = the memory-aliasing bucket).
/// SURVIVES: DRF is NOT a fourth soundness domain (a data race IS UB) — the 4th fourth-domain probe also fails to
/// open one, the map holds, but for the RIGHT reason (concentrate-behind-a-seam, not floor-closed). META: the same
/// clean-framing tic as #33 (omitted `SpinMutex` like #33 omitted `checked_div`) — but a horn-deciding failure this
/// time, the tic at full strength on fresh ground. Salvage **#36**: build the `SpinMutex<T>` (discharged `unsafe
/// impl`). Tally 8→9; N stays 9; rejection-mode count STAYS 8 (a break). Lean `sync_cell_datarace_factors` (a
/// `..._factors` — it breaks). See `sync_cell`.
pub mod sync_cell;

/// Thirty-sixth seam (Phase AI — the CONCURRENCY SLAB, the salvage of #35): a `core`-only spinlock mutex. #35
/// (`sync_cell`) was a deliberately-unsound strawman (`unsafe impl<T> Sync`, obligation **deleted**) that BROKE
/// (tally 8→9), but its *contribution* FAILED (r42): my "**no value-adding concurrency seam / DRF is floor-closed /
/// a re-import is categorically an unchecked axiom**" was refuted by one obvious omitted seam — a sound `SpinMutex<T>`
/// whose `unsafe impl<T: Send> Sync` is **DISCHARGED by a lock invariant**. This seam builds it: a
/// [`spin_mutex::SpinMutex`] wraps `UnsafeCell<T>` beside an `AtomicBool`; [`spin_mutex::SpinMutex::lock`] spins on a
/// `compare_exchange` (`Acquire`) and returns a [`spin_mutex::SpinGuard`] that vends `&T`/`&mut T` and releases
/// (`Release`) on `Drop`. The **same-shaped** `unsafe impl<T: Send> Sync`/`Send` as #35 is here **earned, not
/// asserted**: the CAS is a runtime witness that at most one guard is live, so every reference handed out is
/// exclusive (the obligation #35 deleted is here MET); the `T: Send` bound is #35's missing conjunct (CF1:
/// `SpinMutex<Rc<i32>>: !Sync`). VERDICT (cold panel, r43): **SEAM-BREAKS [horn1-sound-reimport], FAILED — THE
/// SALVAGE ITSELF BROKE (a DISCOVERED break, tally 9→10).** I discharged the LOCK obligation but OMITTED the
/// GUARD's: with no manual impl, `SpinGuard` auto-derives `Sync ⟺ SpinMutex<T>: Sync ⟺ T: Send`, so `&guard: Send`;
/// share one guard by reference across `thread::scope` over `T = Cell<i32>` (`Send + !Sync`, past CF1's `Rc` filter)
/// and two threads race `&Cell` — UB, zero consumer `unsafe`, lock never contended (VERIFIED: compiles under
/// `forbid(unsafe_code)`). `std` writes the stricter `unsafe impl<T: Sync> Sync for MutexGuard` (+ `!Send`); a
/// concurrency `unsafe impl` has TWO conjuncts (cross-thread exclusion AND guard-no-`&T`-leak), I discharged one.
/// SURVIVES the break: the #35-refuting **existential** ("a sound, value-adding, `core`-only concurrency seam
/// exists") stands on its own (a correctly-bounded SpinMutex IS sound — `spin::Mutex`, RustBelt), so the
/// **concentrate-behind-a-seam classification** is NOT refuted — my artifact had a hole, the thesis holds. Three
/// tradeoffs ride to #37: (1) §5 owes IRQ self-deadlock + priority inversion; (2) static-vs-dynamic discharge
/// asymmetry (SPSC/slab static; SpinMutex dynamic, like `RefCell` in the memory-aliasing bucket); (3) cite RustBelt
/// (POPL 2018) for discharged-vs-false. META: the "omit a conjunct" tic (as #35/#33) recurred INSIDE the salvage
/// meant to prove I'd fixed it — calibration NOT converging on the concurrency domain. Tally 9→10 (9th DISCOVERED +
/// #35's 1 conceded); N stays 9; rejection count STAYS 8 (a break). The #35→#36 pair does NOT settle — the chain
/// extends to **#37** (the correctly-bounded `SpinGuard<T: Sync>` + `!Send`, folding in the three tradeoffs). Lean
/// `spin_mutex_guard_sync_factors` (a `..._factors` — it breaks). See `spin_mutex`.
pub mod spin_mutex;

/// Thirty-seventh seam (Phase AJ — the CORRECTLY-BOUNDED SPINLOCK, the salvage of #36): closes the r43 break with
/// the one bound #36 omitted. #36 (`spin_mutex`) discharged the LOCK obligation but left `SpinGuard` auto-deriving
/// `Sync ⟺ T: Send` (too weak), so a shared `&guard` raced the interior of a `Send + !Sync` `T`. This seam enumerates
/// a mutex's **four** auto-trait obligations and discharges all four: (1) `SpinLock<T>: Send where T: Send`; (2)
/// `SpinLock<T>: Sync where T: Send` (exclusive access ⇒ `T` migrates, not shared — like `std::Mutex`); (3) **the
/// fix** `unsafe impl<T: Sync> Sync for SpinLockGuard` — a shared `&guard` hands `&T` to many threads, sound only when
/// `T: Sync`, which opts the guard out of the too-weak auto-`Sync` (CF1: `SpinLockGuard<Cell<i32>>: !Sync`, so the
/// r43 witness no longer compiles); (4) `SpinLockGuard<T>: Send where T: Send`, left auto-derived — sound for a
/// spinlock (`spin::MutexGuard` is `Send`; `std::MutexGuard`'s `!Send` is OS-futex conservatism, documented not
/// cargo-culted). [`spin_lock::SpinLock`]'s mechanics are identical to #36; only the guard bound differs. VERDICT
/// (cold panel, r44): **SEAM-HOLDS [none], SURVIVED (1/1) — the salvage HELD**, the cleanest verdict tier (a full
/// HOLD, no horn even costed; I predicted [horn3] and UNDER-claimed). The **#35→#36→#37 chain SETTLES.** The
/// pre-registered sharpest attack — hunt a FIFTH omitted conjunct (the #36 failure mode) — found NONE: the soundness
/// attacker ran six break attempts (`T: Sync + !Send`; the r43 `Send + !Sync` shape; a `Send`-migrated guard;
/// variance via the invariant `UnsafeCell`; ordering; poison/forget), all failed; three independent reconstructions
/// converged. Obligation-4 `Send` VALIDATED: a spinlock's `Release` store is thread-agnostic (unlike an OS futex),
/// so a migrated guard is sound and the CAS forbids two live guards. Horn 1 HOLDS (r43 break closed, CF1); Horn 2
/// (plain `lock`/`*guard`) + Horn 3 (`unsafe` in 3 impls + 2 `Deref`s, O(1) width) intact. Contribution confirmed:
/// a sound, value-adding, `core`-only concurrency seam EXISTS — DRF is **concentrate-behind-a-seam** (the
/// memory-aliasing bucket, *dynamic* end like `RefCell`); a **discharged** `unsafe impl` distinct from a **false**
/// one; NOT a fourth domain. TWO recorded tradeoffs (SURVIVED-*with*-tradeoffs, soundness not contingent): (1)
/// overstated `std`/`spin` parity — `SpinLock<T>` not `T: ?Sized`, no `into_inner`/`get_mut`, IRQ case deferred to an
/// unbuilt `irqsave`; (2) novelty ~NIL — obligation 3 is *verbatim* Ralf Jung's named June-2017 `MutexGuard`-`Sync`
/// std bug + shipped fix (RustBelt POPL 2018 proved it), so a famous fix copied, not folklore. Tally STAYS 10; N
/// stays 9. Lean `spin_lock_guard_sync_closes` (a `..._closes`: the correctly-bounded guard discharges the
/// re-import; the salvage held). See `spin_lock`.
pub mod spin_lock;

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A fixed-capacity SPSC ring buffer. Owns `N` slots and two atomic indices.
///
/// `N` must be `>= 2` (one slot is always kept empty to disambiguate full from empty).
pub struct RingBuffer<T, const N: usize> {
    /// Interior-mutable slots: the producer writes `tail`, the consumer reads `head`.
    /// Disjointness (P1) is guaranteed by the index protocol, not by the type system.
    slots: [UnsafeCell<MaybeUninit<T>>; N],
    /// Next index to pop (owned by the consumer side).
    head: AtomicUsize,
    /// Next index to push (owned by the producer side).
    tail: AtomicUsize,
}

// SAFETY (the trusted core, P5 at the type level): the buffer may be shared between the
// producer thread and the consumer thread. That is sound exactly when a `T` may itself be
// sent across threads, hence the `T: Send` bound. We never expose `&T` aliasing a live
// `&mut T` to the same slot (P1), so `Sync` does not additionally require `T: Sync`.
unsafe impl<T: Send, const N: usize> Sync for RingBuffer<T, N> {}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Create an empty ring buffer. Compile error if `N < 2`.
    pub fn new() -> Self {
        const { assert!(N >= 2, "RingBuffer capacity N must be >= 2") };
        RingBuffer {
            slots: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Split the buffer into its two affine endpoints.
    ///
    /// Takes `&'a mut self` (proving unique access at the split point), then reborrows it
    /// as a shared `&'a self` handed to *both* handles. The NLL floor ties both handles to
    /// `'a`, so the buffer provably outlives them — this is the no-UAF guarantee, carried
    /// by the floor rather than trusted here.
    pub fn split<'a>(&'a mut self) -> (Producer<'a, T, N>, Consumer<'a, T, N>) {
        let shared: &'a RingBuffer<T, N> = self;
        (
            Producer { rb: shared, cached_head: 0 },
            Consumer { rb: shared, cached_tail: 0 },
        )
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        // Drop any elements still in the queue. We have `&mut self`, so there is no
        // concurrency here and both indices are plain reads.
        let mut head = *self.head.get_mut();
        let tail = *self.tail.get_mut();
        while head != tail {
            // SAFETY: slots in [head, tail) are published and initialized (P1/P3); we own
            // `self` uniquely, so taking each once is sound.
            unsafe {
                let slot = &mut *self.slots[head].get();
                slot.assume_init_drop();
            }
            head = (head + 1) % N;
        }
    }
}

/// The unique producer endpoint. Affine (`!Clone`, `!Copy`); `Send` iff `T: Send`; `!Sync`.
pub struct Producer<'a, T, const N: usize> {
    rb: &'a RingBuffer<T, N>,
    /// A cache of `head` to avoid an atomic load on the fast path; refreshed on a full miss.
    cached_head: usize,
}

/// The unique consumer endpoint. Affine (`!Clone`, `!Copy`); `Send` iff `T: Send`; `!Sync`.
pub struct Consumer<'a, T, const N: usize> {
    rb: &'a RingBuffer<T, N>,
    /// A cache of `tail` to avoid an atomic load on the fast path; refreshed on an empty miss.
    cached_tail: usize,
}

// SAFETY: a handle may be moved to another thread (that is the whole point of SPSC), so
// long as the payload itself is sendable. We do NOT implement `Sync`: a shared `&Producer`
// must never reach a second thread, or two producers could race `tail` (would break P4).
unsafe impl<'a, T: Send, const N: usize> Send for Producer<'a, T, N> {}
unsafe impl<'a, T: Send, const N: usize> Send for Consumer<'a, T, N> {}

impl<'a, T, const N: usize> Producer<'a, T, N> {
    /// Push `x`. Returns `Err(x)` (handing `x` back) if the buffer is full.
    pub fn push(&mut self, x: T) -> Result<(), T> {
        let tail = self.rb.tail.load(Ordering::Relaxed);
        let next = (tail + 1) % N;
        if next == self.cached_head {
            // Possible-full: refresh the cached head with an Acquire load and re-check.
            self.cached_head = self.rb.head.load(Ordering::Acquire);
            if next == self.cached_head {
                return Err(x); // genuinely full
            }
        }
        // SAFETY (P1): the consumer never touches slot `tail` until we publish it by the
        // Release store below, so we hold exclusive access to it right now.
        unsafe {
            (*self.rb.slots[tail].get()).write(x);
        }
        // Release: publishes the payload write to a consumer that Acquire-loads `tail` (P2).
        self.rb.tail.store(next, Ordering::Release);
        Ok(())
    }
}

impl<'a, T, const N: usize> Consumer<'a, T, N> {
    /// Pop the oldest element, or `None` if the buffer is empty.
    pub fn pop(&mut self) -> Option<T> {
        let head = self.rb.head.load(Ordering::Relaxed);
        if head == self.cached_tail {
            // Possible-empty: refresh the cached tail with an Acquire load and re-check.
            self.cached_tail = self.rb.tail.load(Ordering::Acquire);
            if head == self.cached_tail {
                return None; // genuinely empty
            }
        }
        // SAFETY (P1/P2): `head != tail` means slot `head` was published by the producer's
        // Release store, which our Acquire load above synchronized-with; the producer will
        // not touch slot `head` again until we advance `head`, so we read it exclusively.
        let x = unsafe { (*self.rb.slots[head].get()).assume_init_read() };
        // Release: frees the slot for the producer, which Acquire-loads `head` (P2 mirror).
        self.rb.head.store((head + 1) % N, Ordering::Release);
        Some(x)
    }
}
