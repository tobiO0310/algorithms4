use super::{is_sorted, is_whole_sorted};

/// Sorts the array in-place, using elementary sort.
///
/// In the worst case, this makes ~ &frac12; <em>n</em><sup>2</sup> compares
/// and ~ &frac12; <em>n</em><sup>2</sup> exchanges to sort an array of length <em>n</em>.
/// On average, both of the aforementioned values are halved.
///
/// It is stable and uses &Theta;(1) extra space (not including input array).
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }

        debug_assert!(is_sorted(arr, 0..i));
    }
    debug_assert!(is_whole_sorted(arr));
}

/// Sorts the array in-place, using an optimized version of elementary sort.
/// This uses half exchanges, instead of full exchanges, and a sentinel.
///
/// In the worst case, this makes ~ &frac12; <em>n</em><sup>2</sup> compares to sort an array of length <em>n</em>.
/// On average, both of the aforementioned values are halved.
///
/// It is stable and uses &Theta;(1) extra space (not including input array).
pub fn insertion_sort_x<T: Clone + Ord>(arr: &mut [T]) {
    let n = arr.len();

    // set i=0 to be sentinel
    let mut exchanges = 0;
    for i in (1..n).rev() {
        if arr[i] < arr[i - 1] {
            arr.swap(i, i - 1);
            exchanges += 1;
        }
    }
    if exchanges == 0 {
        return;
    }

    // elementary sort with half-exchanges
    for i in 2..n {
        let v = arr[i].clone();

        let mut j = i;
        while v < arr[j - 1] {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = v;
    }

    debug_assert!(is_whole_sorted(arr));
}

/// Sorts the array in-place, using binary search and insertion sort with half exchanges.
///
/// In the worst case, this makes ~ &frac12; <em>n</em><sup>2</sup> exchanges to sort an array of length <em>n</em>.
/// On average, both of the aforementioned values are halved.
///
/// It is stable and uses &Theta;(1) extra space (not including input array).
pub fn binary_insertion_sort<T: Clone + Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        // binary search in the sorted area
        let v = arr[i].clone();
        let mut lo = 0;
        let mut hi = i;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if v < arr[mid] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        // insertion sort with half exchanges
        for j in (lo + 1..=i).rev() {
            arr[j] = arr[j - 1].clone();
        }
        arr[lo] = v;
    }

    debug_assert!(is_whole_sorted(arr));
}

/// Sorts the array in-place, using selection sort.
///
/// This implementation makes ~ &frac12; <em>n</em><sup>2</sup> exchanges to sort an array of length <em>n</em>.
/// It performs exactly <em>n</em> exchanges.
///
/// It is stable and uses &Theta;(1) extra space (not including input array).
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min = i;
        for j in i + 1..n {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
        debug_assert!(is_sorted(arr, 0..=i));
    }
    debug_assert!(is_whole_sorted(arr));
}

/// Sorts the array in-place, using Shellsort with
/// <a href = "https://oeis.org/A003462"> Knuth's increment sequence</a>
/// (1, 4, 13, 40, ...). In the worst case, this implementation makes
/// &Theta;(<em>n</em><sup>3/2</sup>) compares and exchanges to sort
/// an array of length <em>n</em>.
///
/// This sorting algorithm is not stable.
/// It uses &Theta;(1) extra memory (not including the input array).
pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // get the highest usable number in the Knuth's increment sequence
    let mut increment = 1;
    while increment < n / 3 {
        increment = 3 * increment + 1;
    }

    while increment >= 1 {
        // h-sort the array
        for i in increment..n {
            let mut j = i;
            while j >= increment && arr[j] < arr[j - increment] {
                arr.swap(j, j - increment);
                j -= increment;
            }
        }
        if cfg!(debug_assertions) {
            // only run in non-release state
            for i in increment..n {
                if arr[i] < arr[i - increment] {
                    panic!("did not sort correctly {}", increment)
                }
            }
        }
        increment /= 3;
    }
    debug_assert!(is_whole_sorted(arr));
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    test_sort!(
        insertion_sort,
        insertion_sort_x,
        binary_insertion_sort,
        selection_sort,
        shell_sort
    );
}
