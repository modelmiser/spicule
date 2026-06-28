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
