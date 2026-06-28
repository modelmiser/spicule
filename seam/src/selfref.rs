//! # Eleventh seam — a self-referential cell (the move-stability probe).
//!
//! *(Project ordinal: the eleventh distinct primitive, and a *tenth-axis* probe. The nine confirmed
//! axes are all about aliasing, bounds, ownership, cost, type-surface, or destructor timing — none is
//! about a value's **address** being load-bearing. This probes that.)*
//!
//! Same floor, same rules. This seam re-exports a different danger: a **self-referential struct** — a
//! value that contains a pointer *into itself*. [`SelfRef`] holds a `u64` and a `*const u64` that, after
//! [`SelfRef::init`], points at its own `value` field. Reading through that pointer is sound **only if
//! the struct never moves after `init`**: a move relocates `value` but leaves the pointer aimed at the
//! old address — a **dangling read**, instant UB. The danger is *neither* aliasing nor bounds nor
//! destructor-timing; it is **address stability**.
//!
//! The floor's witness for this is `Pin`: a `Pin<&mut T>` / `Pin<&T>` over a `!Unpin` type (here via
//! [`core::marker::PhantomPinned`]) statically forbids moving the pointee. The seam re-exports the raw
//! self-pointer deref as a safe [`SelfRef::get`], sound because `Pin` guarantees no move.
//!
//! ## The axis it probes — and the confound the builder is being upfront about
//!
//! **move-stability** (address stability; self-referential structs; `Pin`; the async/`Future` use case)
//! is a genuinely distinct danger from the nine recovered axes. *But* — and this is the load-bearing
//! question for the cold panel — every Rust witness for it appears to spend *another* axis:
//! - `Pin<&mut Self>` / `Pin<&Self>` are **viral in every signature** ⇒ it looks like **API-locality**.
//! - `Box::pin` (a stable heap address) would spend **bare-metal**; this module uses stack pinning
//!   (`core::pin::pin!`) to stay `core`-only, *isolating the confound to API-locality alone*.
//! - a `with_pinned(value, |p| …)` scope would be a **closure** ⇒ API-locality again.
//!
//! So unlike the drain seam (which dodged its confound by owning its data — a *free-standing* guard),
//! move-stability may have **no** free-standing, non-viral, `core`-only witness. The honest hypotheses
//! for §6: move-stability is (a) a clean tenth axis, (b) *reduces to* API-locality (the `Pin` wrapper
//! *is* the cost), or (c) **real but structurally inseparable** — a distinct danger reachable only by
//! spending API-locality (or bare-metal), which would be a *new kind* of verdict for the lattice.
//!
//! ### CF1 — a pinned `SelfRef` cannot be moved out (that move would dangle the self-pointer):
//! ```compile_fail
//! # use seam_spsc::selfref::SelfRef;
//! use core::pin::pin;
//! let mut a = pin!(SelfRef::new(1));
//! a.as_mut().init();
//! let mut b = pin!(SelfRef::new(2));
//! core::mem::swap(a.as_mut().get_mut(), b.as_mut().get_mut()); // ERROR: get_mut needs Unpin; SelfRef is !Unpin
//! ```

use core::marker::PhantomPinned;
use core::pin::Pin;
use core::ptr;

/// A self-referential cell: a `u64` plus a `*const u64` that points at its own `value` after [`init`].
/// `!Unpin` (via [`PhantomPinned`]), so once pinned it can never be moved — which is exactly the
/// guarantee that keeps the self-pointer valid.
///
/// [`init`]: SelfRef::init
pub struct SelfRef {
    value: u64,
    /// Null until `init`; then points at `self.value`. Valid only while `self` does not move — which
    /// `Pin` enforces.
    ptr: *const u64,
    _pin: PhantomPinned,
}

impl SelfRef {
    /// A fresh, un-initialized, movable cell. (It is movable *until* pinned-and-`init`-ed: the danger
    /// only exists once the self-pointer is set up.)
    pub fn new(value: u64) -> Self {
        SelfRef { value, ptr: ptr::null(), _pin: PhantomPinned }
    }

    /// Establish the self-reference: point `ptr` at `value`. Requires `Pin<&mut Self>` — you can only
    /// call this on an already-pinned cell, so the address `ptr` captures is the address it will keep.
    pub fn init(self: Pin<&mut Self>) {
        // SAFETY: we do not move out of or replace `*self`; we only write the `ptr` field to point at
        // the (now address-stable, because pinned) `value` field. Setting a field does not move the
        // pinned value.
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.value as *const u64;
    }

    /// Read the value *through the self-pointer* (the dangerous primitive, re-exported safe). Sound
    /// because `init` aimed `ptr` at `value` and `Pin` guarantees `self` has not moved since, so `ptr`
    /// is still valid. Panics (a caught logic error, never UB) if `init` was not called.
    pub fn get(self: Pin<&Self>) -> u64 {
        assert!(!self.ptr.is_null(), "SelfRef::get called before init");
        // SAFETY: `ptr == &self.value` (set by `init`) and `self` is pinned, so `value` has not moved;
        // `ptr` therefore still points at a live, initialized `u64` owned by `self`. The read is valid.
        unsafe { *self.ptr }
    }

    /// The value read *directly* (no self-pointer) — for contrast: this is always sound, move or not.
    /// The seam's danger is entirely in [`get`](SelfRef::get)'s pointer indirection.
    pub fn value(self: Pin<&Self>) -> u64 {
        self.value
    }
}
