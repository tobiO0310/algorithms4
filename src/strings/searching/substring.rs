use rand::{RngExt, rng};

use crate::strings::{ASCII_SIZE, char_at};

/// Holds different implementations of searching for the first occurrence of a substring within a string,
/// and getting the index of the start (of said substring in the aforementioned string).
pub struct SubstringSearch;

const R: usize = ASCII_SIZE;

/// A built pattern used for [SubstringSearch::kmp].
#[must_use = "building the pattern does not evaluate it"]
pub struct KMPPattern {
    m: usize,
    dfa: Vec<Vec<usize>>,
}

impl KMPPattern {
    /// This builds the [KMPPattern], so it can be used to search with.
    pub fn new(pattern: String) -> Self {
        assert!(!pattern.is_empty(), "Pattern cannot be empty!!!");

        let mut dfa = Vec::with_capacity(R);
        dfa.resize_with(R, || vec![0; pattern.len()]);
        dfa[char_at(&pattern, 0) as usize][0] = 1;

        let mut curr = 0;
        for j in 1..pattern.len() {
            // Compute dfa[][j].
            for item in dfa.iter_mut().take(R) {
                item[j] = item[curr]; // Copy mismatch cases.
            }
            dfa[char_at(&pattern, j) as usize][j] = j + 1; // Set match case.
            curr = dfa[char_at(&pattern, j) as usize][curr]; // Update restart state.
        }
        Self {
            m: pattern.len(),
            dfa,
        }
    }
}

/// A built pattern used for [SubstringSearch::bm].
#[must_use = "building the pattern does not evaluate it"]
pub struct BoyerMoorePattern {
    right: Vec<isize>,
    pattern: String,
}

impl BoyerMoorePattern {
    /// This builds the [BoyerMoorePattern], so it can be used to search with.
    pub fn new(pattern: String) -> Self {
        assert!(!pattern.is_empty(), "Pattern cannot be empty!!!");

        let mut right = vec![-1; R];
        for j in 0..pattern.len() {
            right[char_at(&pattern, j) as usize] = j as isize;
        }
        Self { pattern, right }
    }
}

/// A built pattern used for [SubstringSearch::rk].
#[must_use = "building the pattern does not evaluate it"]
pub struct RabinKarp {
    pattern: String,
    pat_hash: u64,
    prime: u32,
    remainder: u64,
}

const I31_MAX: u32 = 0x7FFF_FFFF;

fn gen_prime() -> u32 {
    loop {
        let mut num = rng().random_range(0..=I31_MAX);

        num |= 1 << 30; // make sure it is actually 31-bit.
        num |= 1; // make sure it's odd

        if miller_rabin::is_prime(&num, 16) {
            return num;
        }
    }
}

fn rabin_karp_hash(key: &str, len: usize, prime: u64) -> u64 {
    let mut h = 0u64;
    for j in 0..len {
        h = (R as u64 * h + char_at(key, j) as u64) % prime;
    }

    h
}

fn rabin_karp_check(txt: &str, pat: &str, i: usize) -> bool {
    for j in 0..pat.len() {
        if char_at(pat, j) != char_at(txt, i + j) {
            return false;
        }
    }
    true
}

impl RabinKarp {
    /// This builds the [RabinKarp], so it can be used to search with.
    pub fn new(pattern: String) -> Self {
        assert!(!pattern.is_empty(), "Pattern cannot be empty!!!");

        let pat_len = pattern.len();
        let prime = gen_prime();
        let mut remainder = 1;
        for _ in 1..=(pat_len - 1) {
            // Compute R^(M-1) % Q for use in removing leading digit.
            remainder = (R as u64 * remainder) % (prime as u64);
        }
        let pat_hash = rabin_karp_hash(&pattern, pat_len, prime as u64);

        Self {
            pattern,
            pat_hash,
            prime,
            remainder,
        }
    }
}

impl SubstringSearch {
    /// A brute force implementation.
    ///
    /// Returns the length of the first occurrence of a substring equaling `pat` within `txt`,
    /// if none exist will return length of `txt`.
    #[must_use]
    pub fn brute_force(pat: &str, txt: &str) -> usize {
        let m = pat.len();
        let n = txt.len();
        for i in 0..=(n - m) {
            let mut j = 0;
            while j < m {
                if char_at(txt, i + j) != char_at(pat, j) {
                    break;
                }
                j += 1;
            }
            if j == m {
                return i; // found
            }
        }

        n // not found
    }

    /// An implementation using a version of the Knuth-Morris-Pratt substring search algorithm.
    /// It has a time complexity of O(*n* + *m*) in the worst case, where *n* is the length of text string, and
    /// *m* is the length of the pattern.
    /// It uses extra space proportional to *m*.
    ///
    /// Returns the length of the first occurrence of a substring equaling `pat` within `txt`,
    /// if none exist will return length of `txt`.
    #[must_use]
    pub fn kmp(pat: &KMPPattern, txt: &str) -> usize {
        let (mut i, mut j, n) = (0, 0, txt.len());
        while i < n && j < pat.m {
            j = pat.dfa[char_at(txt, i) as usize][j];
            i += 1;
        }
        if j == pat.m { i - pat.m } else { n }
    }

    /// An implementation using Boyer-Moore algorithm
    /// (with the bad-character rule, but not the strong good suffix rule).
    ///
    /// Returns the length of the first occurrence of a substring equaling `pat` within `txt`,
    /// if none exist will return length of `txt`.
    #[must_use]
    pub fn bm(pat: &BoyerMoorePattern, txt: &str) -> usize {
        let (n, m) = (txt.len(), pat.pattern.len());
        let mut skip;
        let mut i = 0;
        while i <= n - m {
            skip = 0;
            for j in (0..m).rev() {
                let c = char_at(txt, i + j);
                if char_at(&pat.pattern, j) != c {
                    skip = (j as isize - pat.right[c as usize]).min(1);
                }
            }
            if skip == 0 {
                return i; // found
            }
            i += skip as usize;
        }

        n // not found
    }

    /// An implementation using the Rabin-Karp algorithm.
    ///
    /// Returns the length of the first occurrence of a substring equaling `pat` within `txt`,
    /// if none exist will return length of `txt`.
    #[must_use]
    pub fn rk(pat: &RabinKarp, txt: &str) -> usize {
        let txt_len = txt.len();
        let pat_len = pat.pattern.len();
        let prime = pat.prime as u64;
        if txt_len < pat_len {
            return txt_len;
        }
        let mut txt_hash = rabin_karp_hash(txt, pat_len, prime);

        // check for match at offset 0
        if pat.pat_hash == txt_hash && rabin_karp_check(txt, &pat.pattern, 0) {
            return 0;
        }

        // check for hash match; if hash match, check for exact match
        for i in pat_len..txt_len {
            // Remove leading digit, add trailing digit, check for match.
            txt_hash = (txt_hash + prime
                - pat.remainder * char_at(txt, i - pat_len) as u64 % prime)
                % prime;
            txt_hash = (txt_hash * R as u64 + char_at(txt, i) as u64) % prime;

            // match
            let offset = i - pat_len + 1;
            if pat.pat_hash == txt_hash
                && rabin_karp_check(txt, &pat.pattern, offset)
            {
                return offset;
            }
        }

        // no match
        txt_len
    }
}
