use std::cmp::Ordering;

use rand::prelude::*;

use super::is_sorted;

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

impl QuickSort {
    /// Sorts the array in-place using quicksort
    ///
    /// In the worst case, this makes ~ 2<em>n</em> ln <em>n</em> compares on average—and
    /// &frac16; that many exchanges—to sort any array of length <em>n</em> with distinct keys.
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
    /// This implementation makes ~ (2ln 2) <em>N H</em> compares on an array of length <em>N</em>,
    /// where <em>H</em> is the Shannon entropy, defined by the frequencies of the key values.
    /// In the worst case <em>H</em> is equal to lg <em>N</em>, and this happens when all keys are distinct.
    ///
    /// It is stable and uses &Theta;(1) extra space (not including input array).
    pub fn three_way_sort<T: Clone + Ord>(arr: &mut [T]) {
        let mut rand = rand::rng();
        arr.shuffle(&mut rand);
        three_way_sort(arr, 0, arr.len() - 1);
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

    test_sort!(quick_sort, three_way_quick_sort);
}
