//! # mmio
//! Abstractions for performing memory-mapped I/O.

#![no_std]
#![deny(missing_docs)]

pub mod reg;
mod vcell;

pub use crate::{
    reg::{ReadReg, Reg, WriteReg},
    vcell::VCell,
};
