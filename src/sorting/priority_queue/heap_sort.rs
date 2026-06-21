use super::{left_child, parent};

/// Holds the implementation for sorting via heapsort.
pub struct HeapSort;

impl HeapSort {
    /// Sorts the array ascending in-place using heapsort
    ///
    /// In the worst case, this makes ~ 2*n* lg *n* + 2 *n* compares on average—and
    /// half that many exchanges—to sort any array of length *n*.
    ///
    /// It is not stable and uses &Theta;(1) extra space (not including input array).
    pub fn sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let n = arr.len() - 1;

        // heap construction
        for k in (0..=(parent(n))).rev() {
            Self::sink(arr, k, n);
        }

        // sort down
        let mut k = n;
        while k > 0 {
            arr.swap(0, k);
            k -= 1;
            Self::sink(arr, 0, k);
        }
    }

    fn sink<T: Ord>(arr: &mut [T], mut pos: usize, n: usize) {
        while left_child(pos) <= n {
            let mut j = left_child(pos);
            if j < n && arr[j] < arr[j + 1] {
                j += 1; // right child
            }
            if arr[pos] >= arr[j] {
                // !(arr[pos] < arr[j])
                break;
            }
            arr.swap(pos, j);
            pos = j;
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;
    use crate::sorting::is_sorted;

    fn heap_sort<T: Ord>(arr: &mut [T]) {
        HeapSort::sort(arr);
    }

    test_sort!(heap_sort);
}
