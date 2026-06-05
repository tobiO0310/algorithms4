use std::cmp::Ordering;

use rand::prelude::*;

use super::{insertion_sort_x, is_sorted};

/// Holds different quicksort implementations
pub struct QuickSort;

fn partition<T: Clone + Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi + 1;
    let v = arr[i].clone();
    loop {
        loop {
            i += 1;
            if arr[i] >= v || i == hi {
                break;
            }
        }
        loop {
            j -= 1;
            if v >= arr[j] || j == lo {
                break;
            }
        }
        if i >= j {
            break;
        }
        arr.swap(i, j);
    }
    arr.swap(lo, j);

    j
}

fn sort<T: Clone + Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let j = partition(arr, lo, hi);
    sort(arr, lo, j.saturating_sub(1));
    sort(arr, j + 1, hi);
    debug_assert!(is_sorted(&arr[lo..=hi]));
}

/// see diagram on page 298 in chapter 2.3 of the book, or their website
fn three_way_sort<T: Clone + Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let v = arr[lo].clone();

    let mut lt = lo;
    let mut gt = hi;
    let mut i = lo + 1;
    while i <= gt {
        match arr[i].cmp(&v) {
            Ordering::Less => {
                arr.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                arr.swap(i, gt);
                gt -= 1;
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }

    three_way_sort(arr, lo, lt.saturating_sub(1));
    three_way_sort(arr, gt + 1, hi);
    debug_assert!(is_sorted(&arr[lo..=hi]));
}

fn median3<T: Ord>(arr: &mut [T], i: usize, j: usize, k: usize) -> usize {
    if arr[i] < arr[j] {
        // i < j
        if arr[j] < arr[k] {
            // i < j < k -> j is median
            j
        } else if arr[i] < arr[k] {
            // i < k <= j -> k is median
            k
        } else {
            // k <= i < j -> i is median
            i
        }
    } else {
        // j <= i
        if arr[k] < arr[j] {
            // k < j <= i -> j is median
            j
        } else if arr[k] < arr[i] {
            // j <= k < i -> k is median
            k
        } else {
            // j <= i <= k -> i is median
            i
        }
    }
}

const INSERTION_SORT_CUTOFF: u8 = 8;
const MEDIAN_OF_3_CUTOFF: u8 = 40;

fn optimized_sort<T: Clone + Ord>(arr: &mut [T], lo: usize, hi: usize) {
    let n = hi + 1 - lo;

    // cutoff to insertion sort  :3
    if n <= INSERTION_SORT_CUTOFF as usize {
        insertion_sort_x(arr);
        return;
    }
    // use the median-of-3 as partitioning element :)
    else if n <= MEDIAN_OF_3_CUTOFF as usize {
        let m = median3(arr, lo, lo + n / 2, hi);
        arr.swap(m, lo);
    }
    // use the Tukey ninther as partitioning element
    else {
        let eps = n / 8;
        let mid = lo + n / 2;
        let m1 = median3(arr, lo, lo + eps, lo + 2 * eps);
        let m2 = median3(arr, mid - eps, mid, mid + eps);
        let m3 = median3(arr, hi - 2 * eps, hi - eps, hi);
        let ninther = median3(arr, m1, m2, m3);
        arr.swap(ninther, lo);
    }

    // Bentley-McIlroy 3-way partitioning
    let (mut i, mut j) = (lo, hi + 1);
    let (mut p, mut q) = (lo, hi + 1);
    let v = arr[lo].clone();
    loop {
        loop {
            i += 1;
            if !(arr[i] < v) || i == hi {
                break;
            }
        }
        loop {
            j -= 1;
            if !(v < arr[j]) || j == lo {
                break;
            }
        }

        // pointers cross
        if i == j && arr[i] == v {
            p += 1;
            arr.swap(p, i);
        }
        if i >= j {
            break;
        }

        arr.swap(i, j);
        if arr[i] == v {
            p += 1;
            arr.swap(p, i);
        }
        if arr[j] == v {
            q -= 1;
            arr.swap(q, j);
        }
    }

    i = j + 1;
    for k in lo..=p {
        arr.swap(k, j);
        j = j.saturating_sub(1)
        // lo == p may make j = 0, therefore we need to saturate sub
    }
    for k in (q..=hi).rev() {
        arr.swap(k, i);
        i += 1;
    }

    optimized_sort(arr, lo, j);
    optimized_sort(arr, i, hi);
}

impl QuickSort {
    /// Sorts the array in-place using quicksort
    ///
    /// In the worst case, this makes ~ 2*n* ln *n* compares on average—and
    /// &frac16; that many exchanges—to sort any array of length *n* with distinct keys.
    ///
    /// It is stable and uses &Theta;(1) extra space (not including input array).
    pub fn sort<T: Clone + Ord>(arr: &mut [T]) {
        let mut rand = rand::rng();
        arr.shuffle(&mut rand);
        sort(arr, 0, arr.len() - 1);
        debug_assert!(is_sorted(arr))
    }

    /// Sorts the array in-place using three-way quicksort
    ///
    /// This implementation makes ~ (2ln 2) *N H* compares on an array of length *N*,
    /// where *H* is the Shannon entropy, defined by the frequencies of the key values.
    /// In the worst case *H* is equal to lg *N*, and this happens when all keys are distinct.
    ///
    /// It is stable and uses &Theta;(1) extra space (not including input array).
    pub fn three_way_sort<T: Clone + Ord>(arr: &mut [T]) {
        let mut rand = rand::rng();
        arr.shuffle(&mut rand);
        three_way_sort(arr, 0, arr.len() - 1);
        debug_assert!(is_sorted(arr));
    }

    /// Sorts the array in-place using an optimized version of quick sort with
    /// Bentley-McIlroy 3-way partitioning, Tukey's ninther, and a cutoff to insertion sort.
    ///
    /// It is stable and uses &Theta;(1) extra space (not including input array).
    pub fn optimized<T: Clone + Ord>(arr: &mut [T]) {
        optimized_sort(arr, 0, arr.len() - 1);
        debug_assert!(is_sorted(arr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quick_sort<T: Clone + Ord>(arr: &mut [T]) {
        QuickSort::sort(arr)
    }

    fn three_way_quick_sort<T: Clone + Ord>(arr: &mut [T]) {
        QuickSort::three_way_sort(arr)
    }

    fn optimized_quick_sort<T: Clone + Ord>(arr: &mut [T]) {
        QuickSort::optimized(arr);
    }

    test_sort!(quick_sort, three_way_quick_sort, optimized_quick_sort);
}
