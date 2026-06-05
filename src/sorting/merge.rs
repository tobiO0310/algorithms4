use crate::{
    sorting::is_sorted,
    utilities::{arithmetic_iter, geometric_iter},
};

fn merge<T: Clone + Ord>(arr: &mut [T], aux: &mut [T], mid: usize) {
    debug_assert!(is_sorted(&arr[0..=mid]));
    debug_assert!(is_sorted(&arr[(mid + 1)..arr.len()]));

    let mut i = 0;
    let mut j = mid + 1;
    let hi = arr.len() - 1;

    aux.clone_from_slice(arr);

    for item in arr.iter_mut() {
        if i > mid {
            *item = aux[j].clone();
            j += 1;
        } else if j > hi {
            *item = aux[i].clone();
            i += 1;
        } else if aux[j] < aux[i] {
            *item = aux[j].clone();
            j += 1;
        } else {
            *item = aux[i].clone();
            i += 1;
        }
    }

    debug_assert!(is_sorted(arr));
}

fn topdown_sort<T: Clone + Ord>(arr: &mut [T], aux: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = (arr.len() - 1) / 2;
    let hi = arr.len() - 1;
    topdown_sort(&mut arr[..=mid], &mut aux[..=mid]);
    topdown_sort(&mut arr[(mid + 1)..=hi], &mut aux[(mid + 1)..=hi]);
    merge(arr, aux, mid);
}

fn index_merge<T: Ord>(
    arr: &[T],
    index: &mut [usize],
    aux: &mut [usize],
    lo: usize,
    mid: usize,
    hi: usize,
) {
    // copy from index to aux
    aux[lo..=hi].clone_from_slice(&index[lo..=hi]);

    // merge into index
    let mut i = lo;
    let mut j = mid + 1;
    for item in index.iter_mut().take(hi + 1).skip(lo) {
        if i > mid {
            *item = aux[j];
            j += 1;
        } else if j > hi {
            *item = aux[i];
            i += 1;
        } else if arr[aux[j]] < arr[aux[i]] {
            *item = aux[j];
            j += 1;
        } else {
            *item = aux[i];
            i += 1;
        }
    }
}

fn index_sort<T: Ord>(arr: &[T], index: &mut [usize], aux: &mut [usize], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    index_sort(arr, index, aux, lo, mid);
    index_sort(arr, index, aux, mid + 1, hi);
    index_merge(arr, index, aux, lo, mid, hi);
}

/// Holds different mergesort implementations
pub struct MergeSort;

impl MergeSort {
    /// Sorts the array using top-down mergesort.
    ///
    /// In the worst case, this takes &Theta;(*n* log *n*) time
    /// to sort any array of length *n* (with the assumptions that comparisons take constant time)
    /// It does between ~ &frac12; *n* log<sub>2</sub> *n* and ~ *n* log<sub>2</sub> *n* compares.
    ///
    /// It is stable and uses &Theta;(*n*) extra space (not including input array).
    ///
    /// ## Notes
    ///
    /// This does use recursion, so to avoid stack overflows it is recommended
    /// to use [MergeSort::bottom_up_sort] instead, if mergesort is desired.
    pub fn top_down_sort<T: Clone + Ord>(arr: &mut [T]) {
        let mut aux: Vec<T> = arr.to_vec();
        topdown_sort(arr, &mut aux);
        debug_assert!(is_sorted(arr));
    }

    /// Sorts the array using bottom-up mergesort.
    ///
    /// In the worst case, this takes &Theta;(*n* log *n*) time
    /// to sort any array of length *n* (with the assumptions that comparisons take constant time)
    /// It does between ~ &frac12; *n* log<sub>2</sub> *n* and ~ *n* log<sub>2</sub> *n* compares.
    ///
    /// It is stable and uses &Theta;(*n*) extra space (not including input array).
    pub fn bottom_up_sort<T: Clone + Ord>(arr: &mut [T]) {
        let n = arr.len();
        let mut aux = arr.to_vec();
        for len in geometric_iter(1, 2).take_while(|&len| len < n) {
            for lo in arithmetic_iter(0, 2 * len).take_while(|&lo| lo < n - len) {
                let mid = lo + len - 1;
                let hi = (lo + len + len - 1).min(n - 1);
                merge(&mut arr[lo..=hi], &mut aux[lo..=hi], mid - lo);
            }
        }
        debug_assert!(is_sorted(arr));
    }

    /// Returns a permutation with elements from the array in a sorted order.
    ///
    /// It has the same time and space complexity as [merge_top_down_sort].
    #[must_use] // the result is the reason for calling this LOL
    pub fn index_sort<T: Ord>(arr: &[T]) -> Vec<usize> {
        let n = arr.len();
        let mut index = (0..n).collect::<Vec<_>>();

        let mut aux = (0..n).collect::<Vec<_>>();
        index_sort(arr, &mut index, &mut aux, 0, n - 1);

        index
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    fn top_down_sort<T: Clone + Ord>(arr: &mut [T]) {
        MergeSort::top_down_sort(arr)
    }

    fn bottom_up_sort<T: Clone + Ord>(arr: &mut [T]) {
        MergeSort::bottom_up_sort(arr)
    }

    test_sort!(top_down_sort, bottom_up_sort);

    #[test]
    fn index_sorted() {
        let mut rand = rand::rng();

        let mut arr = vec![1, 4, 5, 9, 14, 42];
        arr.shuffle(&mut rand);
        assert_ne!(arr, vec![1, 4, 5, 9, 14, 42]);
        let sorted = MergeSort::index_sort(&arr)
            .iter()
            .map(|&i| arr[i])
            .collect::<Vec<_>>();
        assert_eq!(sorted, vec![1, 4, 5, 9, 14, 42]);
    }
}
