/// Test if function is actually sorted
fn is_sorted<T: Ord>(arr: &[T], lo: usize, hi: usize) -> bool {
    for i in (lo + 1)..hi {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

/// Sorts the array in-place, using insertion sort.
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

        debug_assert!(is_sorted(arr, j, 1));
    }
    debug_assert!(is_sorted(arr, 0, n));
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn sort_works() {
        let mut arr = vec![4, 5, 2, 6, 3, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn big_sort_works() {
        let mut rand = rand::rng();

        let mut arr = vec![0; 100];
        for item in arr.iter_mut() {
            *item = rand.random_range(0..1000);
        }

        let mut clone = arr.to_vec();
        clone.sort(); // assume rust's sorting works

        insertion_sort(&mut arr);

        assert_eq!(arr, clone);
    }
}
