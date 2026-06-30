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
/// borrowed slot (it returns `owned` while the slot keeps the read duplicate — two owners). Same packaging,
/// flipped decoupling, flipped horn (cost 3 → break 1). The spec poses: is the horn *forced* (confirming the
/// fence at owned-vs-borrowed) or merely *movable to a different API* (the `into_inner`-less subset appears to
/// hold with a horn-3 leak cost, pushing the fence to soundness-load-bearing-vs-leak-tolerant)? See `reseat`.
pub mod reseat;

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
