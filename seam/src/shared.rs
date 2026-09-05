//! # Seventh seam — reference-counted shared ownership (a minimal `Rc`): the bare-metal axis.
//!
//! Same floor, same rules. This primitive isolates the one seed property the first six left
//! untouched: **bare-metal / runtime-freedom (#5)** — does the seam's sound re-import need a
//! *runtime memory manager* (a global allocator + a reference-count lifecycle), or does it work in
//! `#![no_std]` with no heap? Seams 1–6 are all `core`-only; this one forces the crate's lone
//! `extern crate alloc` (see `lib.rs`). That is the axis, made compile-enforced.
//!
//! ## Why this is a genuinely new precondition shape
//!
//! Every prior seam had a *statically bounded* notion of ownership: SPSC has exactly one producer
//! (affinity), the slab owns its pool (one owner, fixed capacity), a branded/refined index borrows
//! a slice it does not own. Shared ownership is different: **the set of live owners is
//! data-dependent** — `clone` makes another owner at run time, and the value must be freed exactly
//! when the *last* one drops, an instant no static analysis can name. Affinity (one owner) cannot
//! express it; NLL lifetimes (static, lexical-ish) cannot; the slab's generation check tracks
//! validity but still has a single owner (the pool). The only sound witness is a **runtime
//! reference count** — and a refcount needs storage that outlives every owner and is freed on the
//! last drop, i.e. a **manager**: here, the global allocator.
//!
//! ## The axis, and the trade that proves it independent
//!
//! - **Bare-metal (#5) ✗:** `Shared::new` allocates on the global heap; the crate needs `alloc`.
//! - **Off-diagonal vs the slab (proves #5 ⟂ #6):** the slab pays *zero-cost* (#6 ✗ — a runtime
//!   generation compare) yet is *bare-metal* (#5 ✓ — a fixed `[Slot; N]` pool, `core`-only). This
//!   `Shared` *also* pays zero-cost (#6 ✗ — the refcount inc/dec is runtime work) but is *not*
//!   bare-metal (#5 ✗). They **agree on #6 and differ on #5** ⇒ bare-metal is not zero-cost wearing
//!   a hat; it is its own axis.
//! - **The #5 trade (honest):** shared ownership does not *force* the global heap — a pool-backed
//!   refcount (an arena `Rc`, `heapless`-style) would be bare-metal but *capacity-bounded*. So #5
//!   trades against unbounded dynamic count: take the global allocator and grow without limit, or
//!   take a fixed pool and stay bare-metal. (This `Shared` takes the allocator, like `std::rc::Rc`.)
//!
//! ## The soundness mechanism (why safe code cannot reach UB)
//!
//! Shared ownership is vended **read-only**: `Shared<T>: Deref<Target = T>` but *not* `DerefMut`.
//! Aliased mutation — the classic shared-mutable-aliasing UB — is therefore a *compile error*
//! (CF1). Interior mutability would need a `RefCell` (its own runtime borrow flag, the slab's #6
//! cost again); that is out of scope here, and does not change the #5 axis this seam isolates. The
//! refcount lives in `Cell`, so `Shared` is single-threaded (`!Sync`, `!Send`) exactly like `Rc`.
//!
//! ## Honest scope
//!
//! This is the minimal faithful core of `Rc`: shared *immutable* ownership, no `Weak`, no cycle
//! collection. Cycles leak (the documented `Rc` caveat — a *logic*/resource bug, never UB); `Weak`
//! and `RefCell` are real but orthogonal to the bare-metal axis. The `unsafe` is concentrated in
//! this module; consumer code (`tests/shared_use.rs`) is `#![forbid(unsafe_code)]`.
//!
//! ### CF1 — shared ownership is read-only: no aliased `&mut`, so no shared-mutable UB:
//! ```compile_fail
//! # use seam_spsc::shared::Shared;
//! let s = Shared::new(5u32);
//! *s += 1; // ERROR: `Shared<u32>` is `Deref` but not `DerefMut` — cannot mutate shared data
//! ```

use alloc::boxed::Box;
use core::cell::Cell;
use core::ops::Deref;
use core::ptr::NonNull;

/// The heap-allocated, reference-counted payload: a strong count plus the shared value. Private —
/// safe code can neither name it nor forge a `Shared` pointing at a fake one.
struct Inner<T> {
    strong: Cell<usize>,
    value: T,
}

/// A reference-counted shared owner of a `T` (a minimal `Rc`). `Clone` makes another owner; the
/// value is dropped and the allocation freed exactly when the last `Shared` drops.
pub struct Shared<T> {
    // Invariant: `ptr` points at a live `Inner<T>` whose `strong` counts the live `Shared`s
    // (including this one), allocated by `Shared::new` via `Box`. Never null, never dangling while
    // any `Shared` of this allocation exists.
    ptr: NonNull<Inner<T>>,
}

impl<T> Shared<T> {
    /// Allocate `value` on the global heap with a strong count of 1. **This is the `alloc`
    /// dependency** — the syntactic root of the bare-metal cost.
    pub fn new(value: T) -> Self {
        let boxed = Box::new(Inner { strong: Cell::new(1), value });
        // SAFETY: `Box::into_raw` yields a non-null, properly-aligned, live pointer.
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) };
        Shared { ptr }
    }

    #[inline]
    fn inner(&self) -> &Inner<T> {
        // SAFETY: by the type invariant, `ptr` points at a live `Inner<T>` for as long as `self`
        // (a `Shared`) exists — its existence keeps `strong >= 1`, so the allocation is not freed.
        unsafe { self.ptr.as_ref() }
    }

    /// The current number of live owners (for observing the manager's runtime state in tests).
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.inner().strong.get()
    }
}

impl<T> Clone for Shared<T> {
    /// Make another owner: bump the strong count, share the same allocation. This runtime increment
    /// is the manager at work (and the per-clone cost that keeps `Shared` off the zero-cost axis).
    #[inline]
    fn clone(&self) -> Self {
        let c = self.inner().strong.get();
        // Overflow is a SOUNDNESS boundary, not a numeric nicety: `Drop` treats `strong == 1` as "I am
        // the last owner" and frees. A wrapped count therefore frees while owners are still live, which
        // is a use-after-free for every survivor. `std::rc::Rc` aborts here for exactly this reason;
        // this seam dropped the check while describing itself as Rc's minimal faithful core. On a 32-bit
        // target (`usize` = 32 bits — and the crate builds clean for wasm32-unknown-unknown) the safe
        // loop `mem::forget(s.clone())` reaches 2^32 in seconds, not geological time.
        //
        // Panicking rather than aborting is sound here because the check happens BEFORE any state
        // change: on this path `strong` is untouched and no `Shared` has been minted, so unwinding
        // leaves the allocation exactly as it was.
        let next = c.checked_add(1).expect("Shared: strong count overflow (would free while owners live)");
        self.inner().strong.set(next);
        Shared { ptr: self.ptr }
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T> Drop for Shared<T> {
    /// Drop one owner: decrement the strong count, and when it reaches zero, drop the value and free
    /// the allocation. This is the lifecycle a static system cannot schedule — the runtime manager.
    fn drop(&mut self) {
        let c = self.inner().strong.get();
        if c == 1 {
            // Last owner: reclaim the box, running `T`'s destructor and freeing the heap storage.
            // SAFETY: `strong == 1` means no other `Shared` references this allocation, so taking
            // unique ownership back via `Box::from_raw` cannot alias, and the pointer came from
            // `Box::into_raw` in `new`. After this, `self` is dropped and `ptr` is never reused.
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        } else {
            self.inner().strong.set(c - 1);
        }
    }
}
