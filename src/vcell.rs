use ::core::{
    cell::UnsafeCell,
    ptr::{read_volatile, write_volatile},
};

/// A mutable, volatile memory location suitable for MMIO.
#[repr(transparent)]
pub struct VCell<T> {
    value: UnsafeCell<T>,
}

// Implicit due to `UnsafeCell`.
// impl<T> !Sync for VCell<T>

impl<T> VCell<T> {
    /// Conjures up a reference to a `VCell<T>` with lifetime `'a` from
    /// `address`.
    ///
    /// # Safety
    /// Behavior is undefined if any of the following conditions are violated
    /// during `'a`:
    /// - `address` must be valid for reads and writes.
    /// - `address` must be properly aligned.
    /// - `address` must point to a properly initialized value of type `T`.
    pub unsafe fn conjure<'a>(address: *mut T) -> &'a VCell<T> {
        // TODO: This function could probably use a better name. It may be
        // desirable to have `address` be `usize` instead. It is unclear how a
        // caller can ensure that `address` points to a properly initialized
        // value of `T`, unless this function also takes and writes an initial
        // value.

        // Safety: `address` is suitably aligned, is not dangling during `'a`,
        // and points to a valid value because of the safety requirements on
        // the caller.
        &*(address as *mut VCell<T>)
    }

    // TODO: Add a `conjure_many` that returns `&'a [VCell<T>]`.
}

impl<T: Copy> VCell<T> {
    /// Sets the contained value.
    pub fn set(&self, val: T) {
        let dst = self.value.get();
        // Safety: `dst` is valid for writes, and properly aligned because
        // of the safety requirements on `conjure`.
        unsafe { write_volatile(dst, val) };
    }

    /// Replaces the contained value, and returns it.
    pub fn replace(&self, val: T) -> T {
        let old = self.get();
        self.set(val);
        old
    }

    /// Returns a copy of the contained value.
    pub fn get(&self) -> T {
        let src = self.value.get();
        // Safety: `src` is valid for reads, properly aligned, and points to a
        // properly initialized value of type `T` because of the safety
        // requirements on `conjure`.
        unsafe { read_volatile(src) }
    }
}
