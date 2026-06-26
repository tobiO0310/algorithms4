/// This holds three different string-only sorting algorithms.
pub struct Sorting;

const ASCII_SIZE: usize = 256;
const INSERTION_CUTOFF: usize = 15;

/// Returns the UTF-8 byte at `d`, or -1 if `d == s.len()`
fn char_at(s: &str, d: usize) -> i16 {
    debug_assert!(d <= s.len());
    if d == s.len() {
        return -1;
    }
    s.as_bytes()[d] as i16
}

/// Insertion sort a[lo..hi], starting at `d`<sup>th</sup> character.
fn insertion(arr: &mut [&str], d: usize) {
    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && less(arr[j], arr[j - 1], d) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Is `v` less than `w`, starting at character `d`
fn less(v: &str, w: &str, d: usize) -> bool {
    debug_assert_eq!(&v[0..d], &w[0..d]);
    let v_bytes = v.as_bytes();
    let w_bytes = w.as_bytes();
    for i in d..(v.len().min(w.len())) {
        if v_bytes[i] < w_bytes[i] {
            return true;
        }
        if v_bytes[i] > w_bytes[i] {
            return false;
        }
    }

    v.len() < w.len()
}

fn msd_sort<'a>(arr: &mut [&'a str], d: usize, aux: &mut [&'a str]) {
    if arr.len() <= INSERTION_CUTOFF {
        insertion(arr, d);
        return;
    }

    let mut count = vec![0; ASCII_SIZE + 2];
    for i in 0..arr.len() {
        // Compute frequency counts
        count[char_at(arr[i], d) as usize + 2] += 1;
    }

    for r in 0..(ASCII_SIZE + 1) {
        // Transform counts to indices
        count[r + 1] += count[r];
    }

    for i in 0..arr.len() {
        // Distribute
        let char = char_at(arr[i], d) as usize + 1;
        aux[count[char]] = arr[i];
        count[char] += 1;
    }

    for i in 0..arr.len() {
        // Copy back
        arr[i] = aux[i];
    }

    for r in 0..ASCII_SIZE {
        msd_sort(
            &mut arr[count[r]..(count[r + 1] - 1)],
            d + 1,
            &mut aux[count[r]..(count[r + 1] - 1)],
        );
    }
}

fn quick_3_sort(arr: &mut [&str], d: usize) {
    if arr.len() <= INSERTION_CUTOFF {
        insertion(arr, d);
        return;
    }
    let hi = arr.len();

    let (mut lt, mut gt) = (0, hi);
    let v = char_at(arr[0], d);
    let mut i = 1;
    while i <= gt {
        let t = char_at(arr[i], d);
        if t < v {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if t > v {
            arr.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    // a[lo..(lt-1)] < v = a[lt..gt] < a[(gt+1)..hi]

    quick_3_sort(&mut arr[0..(lt - 1)], d);
    if v >= 0 {
        quick_3_sort(&mut arr[lt..gt], d + 1);
    }
    quick_3_sort(&mut arr[(gt + 1)..hi], d);
}

impl Sorting {
    /// Rearranges the array of same length strings in ascending order
    /// with least-significant-digit first (LSD) string sort.
    ///
    /// This sorting is stable, and uses &Theta;(*W*(*N* + *R*)) array accesses and &Theta;(*N* + *R*) extra space,
    /// where *N* is the amount of strings, *R* is the amount of characters in the extended ASCII alphabet and
    /// *W* is the amount of characters in each string.
    ///
    /// # Example
    ///
    /// ```
    /// # use algorithms4::strings::Sorting;
    /// let mut arr = vec!["hello", "every", "bloat", "flare", "elate", "float"];
    ///
    /// Sorting::lsd_sort(&mut arr);
    ///
    /// # assert_eq!(arr, vec!["bloat", "elate", "every", "flare", "float", "hello"]);
    /// ```
    ///
    /// # Panics
    ///
    /// If any string is NOT the same length,
    /// this function WILL panic as it tries to get the character to sort.
    /// This also means, that since UTF-8 is variable length for non-standard ASCII characters,
    /// the actual string length for these characters may differ between strings.
    pub fn lsd_sort(arr: &mut [&str]) {
        let len = arr.len(); // N
        if len == 0 {
            return;
        }

        let mut aux = vec![None; len];
        let str_len = arr[0].len(); // W
        for d in (0..str_len).rev() {
            let mut count = vec![0; ASCII_SIZE + 1];
            for i in 0..len {
                // Compute frequency counts
                let char = arr[i].as_bytes()[d] as usize;
                count[char + 1] += 1;
            }

            for r in 0..ASCII_SIZE {
                // Transform counts to indices
                count[r + 1] += count[r];
            }

            for i in 0..len {
                // Distribute
                let char = arr[i].as_bytes()[d] as usize;
                aux[count[char]] = Some(arr[i]);
                count[char] += 1;
            }

            for i in 0..len {
                // Copy back
                arr[i] = aux[i].unwrap();
            }
        }
    }

    /// Rearranges the array of same length strings in ascending order
    /// with most-significant-digit first (MSD) string sort.
    ///
    /// This sorting is stable, and uses O(*w* *N* *R*) array accesses and &Theta;(*N* + *w R*) extra space
    /// to sort N strings, where *R* is the amount of characters in the extended ASCII alphabet,
    /// *w* is the average amount of characters in each string and
    /// *W* is the amount of characters in longest string.
    ///
    /// With random strings, it falls from 2 *w* *N* to *N* log<sub>*R*</sub> *N* operations on character array.
    ///
    /// # Example
    ///
    /// ```
    /// # use algorithms4::strings::Sorting;
    /// let mut arr = vec!["she", "sells", "seashells", "by", "the", "sea", "shore", "the", "shells", "she", "sells", "are", "surely", "seashells"];
    ///
    /// Sorting::msd_sort(&mut arr);
    ///
    /// # assert_eq!(arr, vec!["are", "by", "sea", "seashells", "seashells", "sells", "sells", "she", "she", "shells", "shore", "surely", "the", "the"]);
    /// ```
    pub fn msd_sort(arr: &mut [&str]) {
        let mut aux = arr.iter().copied().collect::<Vec<_>>();
        msd_sort(arr, 0, &mut aux);
    }

    /// Rearranges the array of same length strings in ascending order
    /// with 3-way quick string sort.
    /// 
    /// This sorting is stable, and has a running time between O(*N*) and O(*N* *w*) and uses &Theta;(1) extra space
    /// to sort N strings, where *w* is the average amount of characters in each string.
    /// 
    /// # Example
    ///
    /// ```
    /// # use algorithms4::strings::Sorting;
    /// let mut arr = vec!["she", "sells", "seashells", "by", "the", "sea", "shore", "the", "shells", "she", "sells", "are", "surely", "seashells"];
    ///
    /// Sorting::quick_3_way_sort(&mut arr);
    ///
    /// # assert_eq!(arr, vec!["are", "by", "sea", "seashells", "seashells", "sells", "sells", "she", "she", "shells", "shore", "surely", "the", "the"]);
    /// ```
    pub fn quick_3_way_sort(arr: &mut [&str]) {
        quick_3_sort(arr, 0);
    }
}
