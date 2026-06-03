use super::{is_sorted, is_whole_sorted};

fn merge<T: Clone + Ord>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    debug_assert!(is_sorted(arr, lo..=mid));
    debug_assert!(is_sorted(arr, (mid + 1)..=hi));

    let mut i = lo;
    let mut j = mid + 1;

    aux[lo..=hi].clone_from_slice(&arr[lo..=hi]);

    for item in arr.iter_mut().take(hi + 1).skip(lo) {
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

    debug_assert!(is_sorted(arr, lo..=hi));
}

fn topdown_sort<T: Clone + Ord>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    topdown_sort(arr, aux, lo, mid);
    topdown_sort(arr, aux, mid + 1, hi);
    merge(arr, aux, lo, mid, hi);
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
    /// In the worst case, this makes &Theta;(<em>n</em> log <em>n</em>) time
    /// to sort any array of length <em>n</em> (with the assumptions that comparisons take constant time)
    /// It does between ~ &frac12; <em>n</em> log<sub>2</sub> <em>n</em> and ~ <em>n</em> log<sub>2</sub> <em>n</em> compares.
    ///
    /// It is stable and uses &Theta;(<em>n</em>) extra space (not including input array).
    ///
    /// ## Notes
    ///
    /// This does use recursion, so to avoid stack overflows it is recommended
    /// to use [merge_bottom_up_sort] instead, if mergesort is desired.
    pub fn top_down_sort<T: Clone + Ord>(arr: &mut [T]) {
        let mut aux: Vec<T> = arr.to_vec();
        topdown_sort(arr, &mut aux, 0, arr.len() - 1);
        debug_assert!(is_whole_sorted(arr));
    }

    /// Sorts the array using bottom-up mergesort.
    ///
    /// In the worst case, this makes &Theta;(<em>n</em> log <em>n</em>) time
    /// to sort any array of length <em>n</em> (with the assumptions that comparisons take constant time)
    /// It does between ~ &frac12; <em>n</em> log<sub>2</sub> <em>n</em> and ~ <em>n</em> log<sub>2</sub> <em>n</em> compares.
    ///
    /// It is stable and uses &Theta;(<em>n</em>) extra space (not including input array).
    pub fn bottom_up_sort<T: Clone + Ord>(arr: &mut [T]) {
        let n = arr.len();
        let mut aux = arr.to_vec();
        let mut len = 1;
        while len <= n {
            let mut lo = 0;
            while lo <= n - len {
                let mid = lo + len - 1;
                let hi = (lo + len + len - 1).min(n - 1);
                merge(arr, &mut aux, lo, mid, hi);
                lo += 2 * len;
            }
            len *= 2;
        }
        debug_assert!(is_whole_sorted(arr));
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
