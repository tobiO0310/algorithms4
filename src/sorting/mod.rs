//! This module holds all the different sorting algorithms described in chapter 2 of the book.

/// Test if array is actually sorted
#[cfg(debug_assertions)]
#[inline] // it is fine to inline when only run during non-release
fn is_sorted<T: Ord>(arr: &[T], lo: usize, hi: usize) -> bool {
    for i in (lo + 1)..hi {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

mod elementary;

pub use elementary::*;
