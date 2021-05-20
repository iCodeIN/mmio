//! Abstractions for performing memory-mapped I/O.
//!
//! Memory-mapped I/O (MMIO) requires working with raw pointers and volatile
//! memory accesses, both of which require manually reasoning about safety.
//! This crate provides the [`VolBox`] (pronounced "volatile box") smart
//! pointer, which expresses unique ownership of a volatile memory location.
//! Additionally, it follows the "unsafe initialization, safe use" pattern to
//! offload safety reasoning to the borrow checker after [`VolBox::new`].
//!
//! Importantly, this crate is careful to [never create references to volatile
//! memory locations][volatile], and [never perform pointer arithmetic across
//! object boundaries][wrapping_add].
//!
//! [volatile]: https://lokathor.github.io/volatile/
//! [wrapping_add]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add
//!
//! # Examples
//! ```no_run
//! # use mmio::*;
//! let mut uart = unsafe { VolBox::<u8, Allow, Allow>::array::<8>(0x1000_0000 as *mut u8) };
//! loop {
//!     let lsr = uart[5].read();
//!     if lsr & 0x20 != 0x0 {
//!         break;
//!     }
//! }
//! uart[0].write(b'\n');
//! ```

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::{fmt, marker::PhantomData, mem::MaybeUninit};

/// Allow access to a memory location.
#[derive(Debug)]
pub enum Allow {}

/// Allow access to a memory location under additional safety rules.
#[derive(Debug)]
pub enum Warn {}

/// Deny access to a memory location.
#[derive(Debug)]
pub enum Deny {}

/// Types which control when and how a memory location can be accessed.
pub trait Access {}

impl Access for Allow {}
impl Access for Warn {}
impl Access for Deny {}

/// An owned memory location for volatile reads and writes.
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct VolBox<T, R, W> {
    loc: *mut T,
    r: PhantomData<R>,
    w: PhantomData<W>,
}

impl<T: Copy, R: Access, W: Access> VolBox<T, R, W> {
    /// Acquire ownership of a memory location.
    ///
    /// If either `R` or `W` are [`Warn`], this volatile box should document
    /// the additional safety requirements for [`Self::read`] and
    /// [`Self::write`] respectively.
    ///
    /// # Safety
    /// Behavior is undefined if any of the following conditions are violated
    /// during the lifetime of `self`:
    /// - `loc` must not be aliased by any reference or read/written thru any
    /// aliased pointer.
    /// - `loc` must be valid for reads if `R` is not [`Deny`].
    /// - `loc` must be valid for writes if `W` is not [`Deny`].
    /// - `loc` must be properly aligned.
    /// - `loc` must point to a properly initialized value of type `T` if `R`
    /// is not [`Deny`].
    pub unsafe fn new(loc: *mut T) -> Self {
        use ::core::{mem::align_of, ptr::null_mut};
        debug_assert_ne!(loc, null_mut());
        debug_assert_eq!((loc as usize) / align_of::<T>(), 0);
        Self {
            loc,
            r: PhantomData,
            w: PhantomData,
        }
    }

    /// Acquire ownership of a range of memory locations.
    ///
    /// # Safety
    /// Behavior is undefined if calling [`Self::new`] on each pointer in the
    /// half-open range `[base_loc, base_loc + LEN)` would cause behavior to be
    /// undefined.
    pub unsafe fn array<const LEN: usize>(base_loc: *mut T) -> [Self; LEN] {
        use ::core::{
            mem::{align_of, size_of, transmute_copy},
            ptr::null_mut,
        };
        debug_assert_ne!(base_loc, null_mut());
        let base_loc = base_loc as usize;
        debug_assert_eq!(base_loc / align_of::<T>(), 0);

        // SAFETY: `assume_init` is safe because `MaybeUninit`s themselves do
        // not require initialization.
        let mut vbs: [MaybeUninit<_>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, vb) in vbs.iter_mut().enumerate() {
            *vb = MaybeUninit::new(Self {
                // NOTE: We do not use `pointer::wrapping_add` or
                // `pointer::add` because we cannot know if this arithmetic
                // will cross an object boundary.
                loc: (base_loc + i * size_of::<T>()) as *mut T,
                r: PhantomData,
                w: PhantomData,
            })
        }
        // SAFETY: `transmute_copy` is safe because all elements of the array
        // are initialized and the source and destination arrays are the same
        // size.
        unsafe { transmute_copy(&vbs) }
    }

    /// Release ownership of the memory location.
    pub fn into_raw(self) -> *mut T {
        self.loc
    }
}

impl<T: Copy, W> VolBox<T, Warn, W> {
    /// Performs a volatile read on the owned memory location.
    ///
    /// # Safety
    /// Please consult the documentation on `self`.
    #[must_use]
    pub unsafe fn read(&self) -> T {
        // SAFETY: `read_volatile` is safe because the memory location is
        // owned, valid for reads, properly aligned, points to a properly
        // initialized value of type `T`, and `T` is `Copy`.
        unsafe { self.loc.read_volatile() }
    }
}

impl<T: Copy, W> VolBox<T, Allow, W> {
    /// Performs a volatile read on the owned memory location.
    #[must_use]
    pub fn read(&self) -> T {
        // SAFETY: `read_volatile` is safe because the memory location is
        // owned, valid for reads, properly aligned, points to a properly
        // initialized value of type `T`, and `T` is `Copy`.
        unsafe { self.loc.read_volatile() }
    }
}

impl<T: Copy, R> VolBox<T, R, Warn> {
    /// Performs a volatile write on the owned memory location.
    ///
    /// # Safety
    /// Please consult the documentation on `self`.
    pub unsafe fn write(&mut self, t: T) {
        // SAFETY: `write_volatile` is safe because the memory location is owned,
        // valid for writes, properly aligned, and `T` is `Copy`.
        unsafe { self.loc.write_volatile(t) };
    }
}

impl<T: Copy, R> VolBox<T, R, Allow> {
    /// Performs a volatile write on the owned memory location.
    pub fn write(&mut self, t: T) {
        // SAFETY: `write_volatile` is safe because the memory location is owned,
        // valid for writes, properly aligned, and `T` is `Copy`.
        unsafe { self.loc.write_volatile(t) };
    }
}

impl<T, R, W> fmt::Pointer for VolBox<T, R, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.loc.fmt(f)
    }
}

// SAFETY: `Send` is safe because `T` is `Send`.
unsafe impl<T: Send, R, W> Send for VolBox<T, R, W> {}

// SAFETY: `Sync` is safe because `T` is `Sync`.
unsafe impl<T: Sync, R, W> Sync for VolBox<T, R, W> {}
