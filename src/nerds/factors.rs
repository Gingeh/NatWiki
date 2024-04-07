use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

const LIMIT: u32 = 100_000_000;

/// Expects 2 <= n <= LIMIT.
fn factors_impl(mut n: u32) -> Vec<(u32, u32)> {
    let mut factors = Vec::new();
    let mut two_count = 0;
    while n % 2 == 0 {
        n /= 2;
        two_count += 1;
    }
    if two_count != 0 {
        factors.push((2, two_count));
    }

    // Note: `factors` will only contain prime numbers.
    // This is because no composite factor of the number
    // can be encountered before the factors of that composite factor.
    let mut i = 3;
    while n != 1 && i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count != 0 {
            factors.push((i, count));
        }
        i += 2;
    }
    // The remainder must be a prime number.
    if n != 1 {
        factors.push((n, 1));
    }
    factors
}

/// The sum of divisors of a number N is multiplicative,
/// so to compute it we can simply compute it for all prime factors
/// ans(p^k) = 1 + p^1 + p^2 + p^3 + ... + p^k
/// and multiply together the result.
fn sum_of_divisors(factors: &[(u32, u32)]) -> Integer {
    factors
        .iter()
        .map(|(p, k)| {
            let mut res = Integer::ONE.clone();
            let mut prod = Integer::ONE.clone();
            for _ in 0..*k {
                prod *= p;
                res += &prod;
            }
            res
        })
        .product()
}

pub async fn factors(n: Arc<Integer>, tx: mpsc::Sender<String>) {
    let Some(n) = n.to_u32() else {
        return;
    };
    if n <= 1 || n > LIMIT {
        return;
    }
    let factors = factors_impl(n);
    if factors.is_empty() {
        return;
    }

    let factors_text: Vec<_> = factors
        .iter()
        .map(|(k, count)| {
            if *count == 1 {
                format!("(#{k})")
            } else {
                format!("(#{k})(^(#{count}))")
            }
        })
        .collect();
    let formatted = factors_text.join("×");
    tx.send(format!("The prime factors of this number are {formatted}."))
        .await
        .unwrap();

     let divisor_sum = sum_of_divisors(&factors);
     let (aliquot_order, aliquot_characteristic) = match divisor_sum.partial_cmp(&(2 * n)).unwrap() {
         std::cmp::Ordering::Less => ("less than", "a deficient"),
         std::cmp::Ordering::Equal => ("equal to", "a perfect"),
         std::cmp::Ordering::Greater => ("greater than", "an abundant"),
    };
    tx.send(format!(
        "Is {aliquot_order} half the sum of its divisors ((#{divisor_sum})), making it {aliquot_characteristic} number.",
    ))
    .await
    .unwrap();
    if divisor_sum == 2 * n - 1 {
        tx.send(format!("Is an almost perfect number."))
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    #[test]
    fn factors_format_properly() {
        crate::test_harness!(|| {
            macro_rules! check {
                ($a:expr, $b:expr) => {
                    let (tx, mut rx) = mpsc::channel(1);
                    tokio::select! {
                        _ = factors(Arc::new(Integer::from($a)), tx.clone()) => {},
                        msg = rx.recv() => assert_eq!(
                            msg,
                            Some(concat!("The prime factors of this number are ", $b, ".").to_owned())
                        )
                    }
                };
            }
            check!(19, "(#19)");
            check!(198900, "(#2)(^(#2))×(#3)(^(#2))×(#5)(^(#2))×(#13)×(#17)");
        });
    }

    #[test]
    fn sum_of_divisors_cases() {
        macro_rules! check {
            ($a:expr, $b:expr) => {
                let fs = factors_impl($a);
                assert_eq!(sum_of_divisors(&fs), $b);
            };
        }

        check!(2, 3);
        check!(5, 6);
        check!(4, 7);
        check!(6, 6 * 2);
        check!(12, 28);
        check!(16, 31);
        check!(28, 28 * 2);
    }

    proptest! {
        #[test]
        fn factors_multiply_to_num(n in 2..100_000_000u32) {
            let factors = factors_impl(n);
            let multiplied: u32 = factors.into_iter().map(|(p, count)| p.pow(count)).product();
            prop_assert_eq!(n, multiplied);
        }

        #[test]
        fn only_prime_factors(n in 2..100_000_000u32) {
            let factors = factors_impl(n);
            for (p, _) in factors.iter() {
                let p_factors = factors_impl(*p);
                prop_assert!(p_factors == vec![(*p, 1)], "factor {p} was not a prime number: {:?}", p_factors);
            }
        }

        #[test]
        fn factors_distribute_over_mul(a in 2..10_000u32, b in 2..10_000u32) {
            let fa = factors_impl(a);
            let fb = factors_impl(b);
            let fab = factors_impl(a*b);
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;
            loop {
                match (fa.get(i), fb.get(j)) {
                    (Some(&(x, cx)), Some(&(y, cy))) => {
                        if x == y {
                            merged.push((x, cx+cy));
                            i += 1;
                            j += 1;
                        } else if x < y {
                            merged.push((x, cx));
                            i += 1;
                        } else {
                            merged.push((y, cy));
                            j += 1;
                        }
                    },
                    (Some(t), None) => {
                        merged.push(*t);
                        i += 1;
                    },
                    (None, Some(t)) => {
                        merged.push(*t);
                        j += 1;
                    }
                    (None, None) => break,
                }
            }
            prop_assert_eq!(fab, merged);
        }

        #[test]
        fn sum_of_divisors_is_multiplicative(mut a in 2..10_000u32, mut b in 2..10_000u32) {
            let mut tmp = Integer::from(a);
            tmp.gcd_u_mut(b);
            let gcd = tmp.to_u32().unwrap();
            a /= gcd;
            b /= gcd;
            // Now they're both coprime, so we can properly test multiplicativity.
            let a_sum = sum_of_divisors(&factors_impl(a));
            let b_sum = sum_of_divisors(&factors_impl(b));
            prop_assert_eq!(a_sum*b_sum, sum_of_divisors(&factors_impl(a*b)))
        }
    }
}
