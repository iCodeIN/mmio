//! Typesafe MMIO register abstractions.

use crate::vcell::VCell;

/// A register wrapper over a volatile cell.
pub trait Reg {
    /// The value type of the volatile cell.
    type Val: Copy;

    /// Access the volatile cell.
    fn access_with<F, R>(&self, f: F) -> R
    where
        F: Fn(&VCell<Self::Val>) -> R;
}

/// A readable MMIO register.
pub trait ReadReg: Reg {
    /// The value type read.
    type Read: From<Self::Val>;

    /// Reads a value.
    fn read(&self) -> Self::Read {
        let v = self.access_with(|vc| vc.get());
        // TODO: should this be `try_into`?
        v.into()
    }
}

/// A writable MMIO register.
pub trait WriteReg: Reg {
    /// The value type written.
    type Write: Into<Self::Val>;

    /// Writes a value.
    fn write(&self, w: Self::Write) {
        // TODO: should this be `try_into`?
        let v = w.into();
        self.access_with(|vc| vc.set(v));
    }
}
