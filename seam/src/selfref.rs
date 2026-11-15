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
//! ## The axis it probes — and the cold panel's ruling (REPORT_SEAM_SPEC_selfref)
//!
//! **move-stability** (address stability; self-referential structs; `Pin`; the async/`Future` use case)
//! is a genuinely distinct danger from the nine recovered axes — no other axis concerns a value's
//! *address*. The cold panel ruled it **real but STRUCTURALLY INSEPARABLE (outcome 3)** — a *new
//! verdict category*: neither a clean tenth axis nor a confounded non-axis (the `typed_nat` outcome).
//! **N stays 9.** The reasoning, and three corrections the panel extracted (now part of the record):
//!
//! - **Not a clean axis (1).** `Pin<&Self>` is the viral-*restrictive* (`'id`-flavor) wrapper: it
//!   infects every receiver **and revokes `&mut`** — and that revocation is the load-bearing mechanism
//!   of the soundness proof. So the seam cannot ace API-locality; the cost cannot be isolated.
//! - **Not a reduction-to-API-locality (2).** The cost is **non-uniform across witnesses**: stack-pin
//!   (`pin!`) spends API-locality, `Box::pin` spends **bare-metal**, a hypothetical `?Move` immovable
//!   type would spend an ambient bound. One danger, several different costs ⇒ not one axis "wearing a hat."
//! - **The real cost is *immovability*, not "API-locality alone".** *(Correction to this module's
//!   earlier framing.)* What M2 pays is the **revocation of the move/return/store capability** —
//!   move-stability funding itself — which is distinct from and *heavier* than API-locality (a merely
//!   leaked type surface).
//! - **Inseparability is *contingent*, not necessary.** *(Correction.)* RFC 2349's **rejected**
//!   `?Move`/`Move` immovable-types design *was* a free-standing witness; Rust **chose** to deny
//!   address-stability a free-standing form (hookless `memcpy` moves + a library `Pin`). So "no
//!   free-standing witness can exist" overstates it — "this floor chose not to provide one" is exact.
//! - **The crisp isolation is a *toy*.** *(Correction.)* The `core`-only, `unsafe`-free safe fragment
//!   here covers only a **single-frame, non-composable** cell. Any real use move-stability exists to
//!   serve — an intrusive node, a `Future`, a slab entry — forces `unsafe` pin-projection or `Box::pin`
//!   (bare-metal). The crispness the adjudication leans on is the crispness of this minimal example.
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
        // SAFETY: two conjuncts, and the second one is the easy-to-miss half.
        //
        // WHERE it points: `ptr == &self.value` (set by `init`) and `self` is pinned, so `value`
        // has not moved; `ptr` therefore still names a live, initialized `u64` owned by `self`.
        //
        // WHAT BORROW it derives from (provenance): `init` mints `ptr` from `&this.value`, a
        // shared reborrow of the `&mut Self` that existed at that instant.
        //
        // CORRECTED 2026-11-14 (cross-vendor panel, Grok 4.7). This comment used to say that a
        // later mutable reborrow of the cell would pop that tag and make this read UB, and that
        // this "cannot happen" because `init` is the ONLY safe route to a `Pin<&mut Self>`. Both
        // halves were wrong. `Pin::as_mut` is a safe route to a `Pin<&mut Self>` that does not
        // go through `init`, and `tests/selfref_provenance.rs` performs exactly that reborrow
        // between `init` and `get`. The read is nevertheless accepted by Miri under BOTH Stacked
        // Borrows and Tree Borrows (rerun 2026-11-14). The reason is in the aliasing models, not
        // in this API, and it differs between them — measured the same day with a two-type probe
        // (the identical struct with and without `PhantomPinned`; init, a `&mut` reborrow passed
        // to a function, then the read):
        //   - Stacked Borrows: the `!Unpin` copy passes, the `Unpin` copy is UB. So under SB the
        //     `PhantomPinned` field is LOAD-BEARING for provenance, not only for address
        //     stability: Miri does not treat a `&mut` to a `!Unpin` type as unique, which is
        //     what lets pinned self-referential values exist at all.
        //   - Tree Borrows: both copies pass. A reborrow that is never written through does not
        //     invalidate the read-only `ptr`, with or without the marker.
        // What this API contributes is narrower: no safe method WRITES `value` after `init`, so
        // no write through a fresh reference ever conflicts with `ptr`.
        //
        // This is an argument from what Miri implements, not from a ratified rule: Rust has no
        // settled aliasing model, and the `!Unpin` carve-out is the part most likely to be
        // restated (`UnsafePinned`). (Adding any `pub fn ...(self: Pin<&mut Self>)` that writes
        // `value` without re-running `init` would break the narrower argument too.)
        unsafe { *self.ptr }
    }

    /// The value read *directly* (no self-pointer) — for contrast: this is always sound, move or not.
    /// The seam's danger is entirely in [`get`](SelfRef::get)'s pointer indirection.
    pub fn value(self: Pin<&Self>) -> u64 {
        self.value
    }
}
