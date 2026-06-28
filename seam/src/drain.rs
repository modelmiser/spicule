//! # Tenth seam — drain-and-yield with a leaked guard (the leak-freedom axis).
//!
//! *(Project ordinal: the tenth distinct primitive, and a *ninth-axis* probe — the eighth axis,
//! unwind-safety, is already confirmed. This targets a sibling-but-distinct axis: leak-freedom.)*
//!
//! Same floor, same rules. This seam re-exports a different danger: **moving elements out of a buffer
//! through an iterator guard**. `drain(&mut buf)` yields each element by value (`ptr::read`), leaving
//! the slot logically moved-from. The danger is the canonical Rust **leakpocalypse** (`mem::forget` is
//! a *safe* function): if the `Drain` guard is leaked after yielding some elements, and the buffer
//! still counts those slots as live, the buffer's `Drop` will drop the moved-from slots — a
//! **double-free**. This is the historical pre-1.0 `Vec::drain` "leak amplification" bug.
//!
//! It isolates an axis distinct from the eight already recovered: **leak-freedom** — does sound
//! re-import survive the witness being *leaked* (its `Drop` never running)? The NLL floor's vocabulary
//! cannot express "this guard's destructor is load-bearing for soundness," because **leaking is safe**
//! (`mem::forget`, `Box::leak`, `Rc` cycles — all safe, all skip `Drop`). So a seam whose soundness
//! leans on a destructor must pay to stay sound under a leak.
//!
//! ## The witness: pre-pessimization (the sound `Vec::drain` pattern)
//!
//! [`Buffer::drain`] sets the buffer's length to **zero up front**, *before* yielding anything, and
//! the returned [`Drain`] tracks the un-yielded range to drop on its own `Drop`. The effect:
//! - **Normal drop** (guard consumed or dropped): the `Drain`'s `Drop` drops exactly the un-yielded
//!   remainder; the buffer (len 0) drops nothing. Every element is dropped exactly once.
//! - **Leaked guard** (`mem::forget(drain)`): the `Drain`'s `Drop` never runs, so the un-yielded
//!   remainder is **leaked** (memory leak — not UB); the buffer (len 0) still drops nothing, so the
//!   already-yielded, moved-from slots are *not* double-dropped. **Sound under leak.**
//!
//! ## The cost that isolates the axis
//!
//! Leak-safety is bought with **pre-pessimization**: the buffer is emptied before the work is done, so
//! a leak (or a panic that the caller `catch_unwind`s after forgetting the guard) **silently drops the
//! un-yielded data** rather than corrupting memory. That guarantee downgrade — "leak the guard, lose
//! the tail" — is the price of leak-freedom on an affine floor that *permits* leaks. It is **not** any
//! of the eight prior axes: the seam is sound, decidable, O(1)-concentrated, **brand-free** (a plain
//! `Drain<'a>`, no viral type and *no closure* — the off-diagonal the scoped-thread witness could not
//! reach), zero-cost, legible, `core`-only, and unwind-safe — it aces all eight and pays only here.
//!
//! ## Leak-freedom ⟂ unwind-safety (the off-diagonal vs the take seam)
//!
//! These two are siblings — both about a `Drop` not running *normally* — but distinct. The take seam
//! (unwind-safety) is **unwind-sensitive** (a panic mid-take must abort) yet **leak-safe** (forgetting
//! its abort-bomb on the happy path is fine). This drain seam is **leak-sensitive** (a forgotten guard
//! must not double-free) yet handles unwinding *for free* (a panic mid-iteration just runs the guard's
//! `Drop`, which the pre-pessimization already made safe). A leak is "the destructor *never* runs"; an
//! unwind is "the destructor runs at a *bad time*." A primitive can be robust to one and not the other.
//!
//! ### CF1 — the buffer cannot be touched while a `Drain` borrows it (no aliasing the half-moved state):
//! ```compile_fail
//! # use seam_spsc::drain::Buffer;
//! let mut b = Buffer::<u32, 4>::new();
//! b.push(1).ok();
//! let d = b.drain();
//! b.push(2).ok();   // ERROR: cannot borrow `b` mutably while `d` (a &mut borrow) is alive
//! let _ = d;
//! ```

use core::mem::MaybeUninit;

/// A fixed-capacity, `core`-only buffer (no heap — bare-metal). Holds up to `N` elements.
pub struct Buffer<T, const N: usize> {
    slots: [MaybeUninit<T>; N],
    /// Number of live, initialized elements in `slots[0..len]`. The drain guard drives this to 0
    /// *before* moving anything out, which is the whole leak-safety mechanism.
    len: usize,
}

impl<T, const N: usize> Buffer<T, N> {
    /// An empty buffer.
    pub const fn new() -> Self {
        Buffer { slots: [const { MaybeUninit::uninit() }; N], len: 0 }
    }

    /// Append `x`; returns `Err(x)` if full.
    pub fn push(&mut self, x: T) -> Result<(), T> {
        if self.len < N {
            self.slots[self.len].write(x);
            self.len += 1;
            Ok(())
        } else {
            Err(x)
        }
    }

    /// Number of live elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Drain all elements by value. **Pre-pessimization:** the buffer's length is set to 0 *now*, so a
    /// leaked [`Drain`] leaves a valid empty buffer (the un-yielded tail leaks, but no double-free).
    pub fn drain(&mut self) -> Drain<'_, T, N> {
        let end = self.len;
        self.len = 0; // the leak-safety mechanism: emptied before any element is moved out
        Drain { buf: self, idx: 0, end }
    }
}

impl<T, const N: usize> Default for Buffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for Buffer<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            // SAFETY: slots[0..len] are initialized by `push` and not yet moved out (a live `Drain`
            // would have set len = 0). Each is dropped exactly once.
            unsafe { self.slots[i].assume_init_drop() };
        }
    }
}

/// The drain guard: an iterator that yields the buffer's elements by value. Free-standing (a plain
/// `&mut` borrow — no brand, no closure). Its `Drop` is **load-bearing**: it drops the un-yielded
/// remainder. Leaking it (`mem::forget`) is sound but loses that remainder (the leak-freedom cost).
pub struct Drain<'a, T, const N: usize> {
    buf: &'a mut Buffer<T, N>,
    /// Next index to yield.
    idx: usize,
    /// Original length (one past the last drainable index).
    end: usize,
}

impl<T, const N: usize> Iterator for Drain<'_, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.idx < self.end {
            // SAFETY: slots[idx] is initialized (idx < end == original len) and has not been read yet
            // (idx is monotonic). After this read the slot is logically moved-from; `buf.len` is
            // already 0, so neither the buffer's nor this guard's `Drop` will touch slots[<idx].
            let x = unsafe { self.buf.slots[self.idx].assume_init_read() };
            self.idx += 1;
            Some(x)
        } else {
            None
        }
    }
}

impl<T, const N: usize> Drop for Drain<'_, T, N> {
    fn drop(&mut self) {
        for i in self.idx..self.end {
            // SAFETY: slots[idx..end] are the un-yielded, still-initialized elements; `next` has not
            // read them and `buf.len` is 0 so nothing else will. Each is dropped exactly once.
            unsafe { self.buf.slots[i].assume_init_drop() };
        }
    }
}
