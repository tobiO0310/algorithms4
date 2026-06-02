//! This module holds all the different sorting algorithms described in chapter 2 of the book.
//!
//! ## Examples
//!
//! Sort an array :)
//! ```
//! use algorithms4::merge_bottom_up_sort;
//!
//! let mut vec = vec![4, 1, 8, 10, 47, 2];
//! # let mut og_sorted_vec = vec.clone();
//! # og_sorted_vec.sort();
//! merge_bottom_up_sort(&mut vec);
//! assert_eq!(vec, og_sorted_vec);
//! ```
//!
//! ## Notes
//!
//! A lot of these implementations will have extra code running during a non-release build.
//! This will contribute to increased running time in non-release builds. The amount depends on sorting algorithm and array size

/// Test if array is actually sorted from lo..hi
fn is_sorted<T: Ord>(arr: &[T], lo: usize, hi: usize) -> bool {
    for i in (lo + 1)..hi {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

mod elementary;
mod merge;

pub use elementary::*;
pub use merge::*;
