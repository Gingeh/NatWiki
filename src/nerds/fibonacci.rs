use std::{mem::swap, sync::Arc};

use phf::phf_map;
use rug::{Assign, Complete, Integer};
use tokio::sync::mpsc;

use super::Fact;

pub async fn fibonacci(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    if is_fib(&n) {
        tx.send(Fact::Basic(format!(
            "Is the (#{})th fibonacci number.",
            fib_index(&n)
        )))
        .await
        .unwrap();
    }
}

fn is_fib(n: &Integer) -> bool {
    // x is a Fibonacci number if and only if
    // either 5x^2+4 or 5x^2−4 is a perfect square
    let test = n.clone().square() * 5_u8;
    (&test + 4_u8).complete().is_perfect_square() || (test - 4_u8).is_perfect_square()
}

static SMALL_FIBS: phf::Map<u32, u8> = phf_map! {
    0_u32 => 0,
    1_u32 => 1,
    2_u32 => 3,
    3_u32 => 4,
    5_u32 => 5,
    8_u32 => 6,
    13_u32 => 7,
    21_u32 => 8,
    34_u32 => 9,
    55_u32 => 10,
    89_u32 => 11,
    144_u32 => 12,
    233_u32 => 13,
    377_u32 => 14,
    610_u32 => 15,
    987_u32 => 16,
    1597_u32 => 17,
    2584_u32 => 18,
    4181_u32 => 19,
    6765_u32 => 20,
    10946_u32 => 21,
    17711_u32 => 22,
    28657_u32 => 23,
    46368_u32 => 24,
    75025_u32 => 25,
    121393_u32 => 26,
    196418_u32 => 27,
    317811_u32 => 28,
    514229_u32 => 29,
    832040_u32 => 30,
    1346269_u32 => 31,
    2178309_u32 => 32,
};

pub fn fib_index(n: &Integer) -> Integer {
    if let Some(&idx) = n.to_u32().and_then(|n| SMALL_FIBS.get(&n)) {
        return Integer::from(idx);
    }

    let mut idx = Integer::from(32);
    let mut a = Integer::fibonacci(64).complete(); // fib(2 * idx)
    let mut b = Integer::fibonacci(65).complete(); // fib(2 * idx + 1)
    let mut a_prev = Integer::fibonacci(32).complete(); // fib(idx)
    let mut b_prev = Integer::fibonacci(33).complete(); // fib(idx + 1)
    let mut tmp = Integer::new();

    // double idx until fib(2 * idx) overshoots n
    while *n > a {
        swap(&mut a_prev, &mut a);
        swap(&mut b_prev, &mut b);

        a.assign(&b_prev * 2_u8 - &a_prev);
        a *= &a_prev;

        b.assign(&a_prev);
        b.square_mut();
        tmp.assign(&b_prev);
        tmp.square_mut();
        b += &tmp;

        idx *= 2;
    }

    // take a step back
    swap(&mut a_prev, &mut a);
    swap(&mut b_prev, &mut b);

    // increment idx until fib(idx) >= n
    while *n > a {
        tmp.assign(&a + &b);
        swap(&mut a, &mut b);
        swap(&mut tmp, &mut b);
        idx += 1;
    }

    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    #[test]
    fn positive_match() {
        crate::test_harness!(|(n in 0..10000u32)| {
            let nth = Integer::fibonacci(n).complete();
            prop_assume!(&nth != Integer::ONE); // fib(n) = 1 has two solutions
            prop_assert!(is_fib(&nth));
            prop_assert_eq!(
                fib_index(&nth),
                Integer::from(n)
            );
        });
    }
}
