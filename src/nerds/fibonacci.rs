use std::{mem::swap, sync::Arc};

use rug::{Assign, Complete, Integer};
use tokio::sync::mpsc;

pub async fn fibonacci(n: Arc<Integer>, tx: mpsc::Sender<String>) {
    if is_fib(&n) {
        tx.send(format!("Is the (#{})th fibonacci number.", fib_index(&n)))
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

fn fib_index(n: &Integer) -> Integer {
    if n.is_zero() {
        return Integer::ZERO;
    }

    let mut idx = Integer::from(1);
    let mut a = Integer::from(1);      // fib(2 * idx)
    let mut b = Integer::from(2);      // fib(2 * idx + 1)
    let mut a_prev = Integer::from(1); // fib(idx)
    let mut b_prev = Integer::from(1); // fib(idx + 1)
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
            prop_assert!(is_fib(&nth));
            prop_assert_eq!(
                fib_index(&nth),
                Integer::from(n)
            );
        });
    }
}
