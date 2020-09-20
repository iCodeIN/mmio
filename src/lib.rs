//! # mmio
//! Abstractions for performing memory-mapped I/O.

#![no_std]
#![deny(missing_docs)]

mod vcell;

pub use vcell::VCell;
