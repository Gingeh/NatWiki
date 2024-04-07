/// This module finds numbers of the form x^y for x,y >= 2 integer.
/// For any N = x^y, `y` is upper bounded by ln(N)/ln(x) <= ln(N)/ln(2) <= log_2(N),
/// so finding `y` can be done by simply looping through all numbers 2 <= i <= log_2(N).
/// and testing if the `y`th root of `N` is an integer.
///
/// This is quite slow, so we do have optimizations.
///
/// If a prime factor of the form p^k divides N, then we can test for a valid `y` simply by testing
/// if `y` divides `k` and (N/p^k)^(1/y) is an integer.
/// We can therefore speed this up significantly for most composite numbers by testing against
/// a pre-programmed list of prime numbers.
use std::sync::Arc;

use num_traits::identities::One;
use rug::{Complete, Integer};
use tokio::sync::mpsc;

const SMALL_PRIMES: &[u32] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

/// Returns (x,y) such that x^y = N, x > 1, y > 1 if such a pair exists.
/// Because y <= log2(N), and N fits in less than 4GB because otherwise our server would crash,
/// `y` fits in a u32.
/// Expects n > 1.
fn power_form_impl(mut n: Integer) -> Option<(Integer, u32)> {
    let mut rem = Integer::new();

    // First, test: Does N have a prime factor of the form p^k?
    let mut prime_factor: Option<(u32, u32)> = None;
    for &prime in SMALL_PRIMES.iter() {
        if n.is_divisible_u(prime) {
            // By how much?
            n.div_exact_u_mut(prime);
            let mut k = 1;
            while n.is_divisible_u(prime) {
                n.div_exact_u_mut(prime);
                k += 1;
            }
            prime_factor = Some((prime, k));
            break;
        }
    }

    macro_rules! check_integer_root {
        ($n:ident, $y:expr) => {
            let y = $y;
            let mut x = $n.clone();
            x.root_rem_mut(&mut rem, y);
            if rem.is_zero() {
                if let Some((p, k)) = prime_factor {
                    // We reconstruct the real `x` by
                    // computing x * p^(k/y).
                    x *= Integer::i_pow_u(p as i32, k / y).complete();
                }
                return Some((x, y));
            }
        };
    }

    match prime_factor {
        None => {
            // We have no choice, do it the slow way.
            let log2_n_floored = n.significant_bits();
            // We do it in reverse so we can find the highest exponents first,
            // since it's preferable to output 2^32 instead of 65536^2.
            for y in (2..log2_n_floored).rev() {
                check_integer_root!(n, y);
            }
            None
        }
        Some((_, k)) => {
            // Note that in this branch,
            // `n` is actually just the original N divided by p^k.
            // The test still works because
            // (N/p^k)^(1/y) = (N^(1/y)) / (p^(k/y))
            // so N^(1/y) is guaranteed to be an integer if k%y==0 and (N/p^k)^(1/y) is an integer.

            // First get rid of the small cases.
            if k == 1 {
                // Definitely can't be a power.
                return None;
            }
            check_integer_root!(n, k);
            if k == 2 {
                // Can only be a square.
                // We've already checked that above, so no other options.
                return None;
            }

            // We step through divisors 2-by-2 (skipping over evens) if `k` is odd,
            // or 1-by-1 if `k` is even.
            let step = 1 + k % 2;
            // Start at 3 if k is odd, 2 if k is even.
            let mut y_quot = 2 + k % 2;
            // First test the higher powers.
            while y_quot * y_quot <= k {
                if k % y_quot == 0 {
                    check_integer_root!(n, k / y_quot);
                }
                y_quot += step;
            }
            // Then test the lower powers.
            y_quot = 2 + k % 2;
            while y_quot * y_quot <= k {
                if k % y_quot == 0 {
                    check_integer_root!(n, y_quot);
                }
                y_quot += step;
            }
            // No candidates remaining.
            None
        }
    }
}

pub async fn power_form(n: Arc<Integer>, tx: mpsc::Sender<String>) {
    if n.is_zero() || n.as_ref().is_one() {
        return;
    }
    if let Some((x, y)) = power_form_impl(Arc::unwrap_or_clone(n)) {
        tx.send(format!("This number is a perfect power: (#{x})(^(#{y}))."))
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use rug::ops::Pow;

    #[test]
    fn simpler_tests() {
        macro_rules! check {
            ($a:expr, ($b1:expr,$b2:expr)) => {
                assert_eq!(power_form_impl($a.into()), Some(($b1.into(), $b2)))
            };
        }
        check!(65536, (2, 16));
        check!(2147483648u32, (2, 31));
        check!(25, (5, 2));
        check!(625, (5, 4));
        check!(3125, (5, 5));
        check!(
            Integer::parse("1000000028000000294000001372000002401").unwrap(),
            (1_000_000_007, 4)
        );
    }

    proptest! {
        #[test]
        // exclude a=1
        fn reconstruct_powers_large_base(a in "[1-9][0-9]+|[2-9]", b in 2..256u32) {
            let a = Integer::parse(a).unwrap().complete();
            let n = a.pow(b);
            let form = power_form_impl(n.clone());
            prop_assert!(form.is_some(), "couldn't find form for {n}");
            let Some((x, y)) = form else { unreachable!() };
            prop_assert_eq!(x.clone().pow(y), n.clone(), "failed equal: {}^{} != {}", x, y, n);
        }

        #[test]
        fn reconstruct_powers_small_base(a in 2..256u32, b in 2..1024u32) {
            let a = Integer::from(a);
            let n = a.pow(b);
            let form = power_form_impl(n.clone());
            prop_assert!(form.is_some(), "couldn't find form for {n}");
            let Some((x, y)) = form else { unreachable!() };
            prop_assert_eq!(x.clone().pow(y), n.clone(), "failed equal: {}^{} != {}", x, y, n);
        }
    }
}
