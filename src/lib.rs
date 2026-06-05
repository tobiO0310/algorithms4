//! This crate is a rust implementation of the algorithms described in
//! the 4th edition of Algorithms by R. Sedgewick and K. Wayne.

#![warn(missing_docs)]

pub mod collections;
pub mod sorting;
pub mod union_find;
mod utilities;

pub use sorting::*;
pub use union_find::*;
