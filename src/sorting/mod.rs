//! This module holds all the different sorting algorithms described in chapter 2 of the book.
//!
//! ## Examples
//!
//! Sort an array :)
//! ```
//! use algorithms4::MergeSort;
//!
//! let mut vec = vec![4, 1, 8, 10, 47, 2];
//! # let mut og_sorted_vec = vec.clone();
//! # og_sorted_vec.sort();
//! MergeSort::bottom_up_sort(&mut vec);
//! assert_eq!(vec, og_sorted_vec);
//! ```
//!
//! ## Notes
//!
//! A lot of these implementations will have extra code running during a non-release build.
//! This will contribute to increased running time in non-release builds. The amount depends on sorting algorithm and array size

use std::ops::RangeBounds;

/// Test if array is actually sorted in the given range
fn is_sorted<T: Ord, R: RangeBounds<usize> + IntoIterator<Item = usize>>(
    arr: &[T],
    range: R,
) -> bool {
    for i in range.into_iter().skip(1) {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

fn is_whole_sorted<T: Ord>(arr: &[T]) -> bool {
    is_sorted(arr, 0..arr.len())
}

#[cfg(test)]
macro_rules! test_sort {
        ($($name:ident),*) => ($(
            pastey::paste! {
                #[test]
                fn [<$name _works>]() {
                    let mut arr = vec![4, 5, 2, 6, 3, 1];
                    $name(&mut arr);
                    assert!(is_whole_sorted(&arr));
                    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
                }

                #[test]
                fn [<$name _big_works>]() {
                    let mut rand = rand::rng();

                    let mut arr = vec![0; 100];
                    for item in arr.iter_mut() {
                        *item = rand.random_range(0..1000);
                    }

                    let mut clone = arr.to_vec();
                    clone.sort(); // assume rust's sorting works

                    $name(&mut arr);

                    assert!(is_whole_sorted(&arr));
                    assert_eq!(arr, clone);
                }

                #[test]
                fn [<$name _single>]() {
                    let mut arr = vec![1];
                    $name(&mut arr);
                    assert!(is_whole_sorted(&arr));
                    assert_eq!(arr, vec![1]);
                }

                #[test]
                fn [<$name _pre_sorted>]() {
                    let mut arr = vec![1, 4, 5, 9, 14, 42];
                    $name(&mut arr);
                    assert!(is_whole_sorted(&arr));
                    assert_eq!(arr, vec![1, 4, 5, 9, 14, 42]);
                }
            }
        )*);
    }

mod elementary;
mod merge;

pub use elementary::*;
pub use merge::*;
