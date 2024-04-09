use std::sync::Arc;

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
    let mut a = Integer::ZERO;
    let mut b = Integer::ONE.clone();
    let mut tmp = Integer::new();
    let mut idx = Integer::ZERO;
    while *n > a {
        tmp.assign(&a + &b);
        a.assign(&b);
        b.assign(&tmp);
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
