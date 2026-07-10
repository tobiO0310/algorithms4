use std::iter::repeat_with;

use num::{One, Zero};
use num_bigint::{BigRng010, BigUint, ToBigUint};

/// Returns an iterator following the geometric series,
/// with *a* being the initial value and *r* being the common ratio
pub(crate) fn geometric_iter(
    a: usize,
    r: usize,
) -> impl Iterator<Item = usize> {
    (0..).map(move |i| a * r.pow(i))
}

/// Returns an iterator following the arithmetic series,
/// with *a* being the initial value and *d* being the common difference
pub(crate) fn arithmetic_iter(
    a: usize,
    d: usize,
) -> impl Iterator<Item = usize> {
    (0..).map(move |i| a + d * i)
}

macro_rules! biguint {
    ($e:expr) => {
        ($e).to_biguint().unwrap()
    };
}

fn miller_rabin_decompose(n: &BigUint) -> (u64, BigUint) {
    assert!(!n.is_zero());

    let n = n - 1u8;
    let s = n.trailing_zeros().unwrap(); // n is not zero as per assertion

    (s, n >> s)
}
fn miller_rabin(a: &BigUint, n: &BigUint, s: u64, d: &BigUint) -> bool {
    let n_minus_1 = n - 1u8;
    let one = BigUint::one();
    let two = biguint!(2);

    let mut x = a.modpow(d, n);
    let mut y = one.clone();

    for _ in 0..s {
        y = x.modpow(&two, n);
        if y == one && x != one && x != n_minus_1 {
            return false;
        }
        x = y.clone();
    }
    y == one
}

/// Return `true` if `n` is a probable prime.
///
/// Uses the Miller-Rabin primality test, testing `k` times.
/// The false positive risk is bounded by 4^<sup>-*k*</sup>.
pub fn is_prime<T: ToBigUint>(n: &T, k: usize) -> bool {
    let n = &n.to_biguint().unwrap();
    let n_minus_one: BigUint = n - 1u8;
    let two = &biguint!(2);
    let (s, d) = miller_rabin_decompose(n);

    if n <= &BigUint::one() {
        return false;
    } else if n <= &biguint!(3) {
        return true;
    } else if n <= &biguint!(0xFFFF_FFFF_FFFF_FFFFu64) {
        // if n less than u64, simply use 16 small known primes
        let samples: Vec<u8> =
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

        return samples
            .iter()
            .filter(|&&m| biguint!(m) < n_minus_one)
            .find(|&&a| miller_rabin(&biguint!(a), n, s, &d))
            .is_none();
    }

    let mut rng = rand::rng();
    let min = two.pow(n_minus_one.bits() as u32 - 1);
    let max = two.pow(n_minus_one.bits() as u32) - 1u8;
    let samples: Vec<BigUint> =
        repeat_with(|| rng.random_biguint_range(&min, &max))
            .filter(|m| m < &n_minus_one)
            .take(k)
            .collect();

    samples.iter().find(|&a| miller_rabin(a, n, s, &d)).is_none()
}
