use std::sync::Arc;

use rug::{integer::IsPrime, Complete, Integer};
use tokio::sync::mpsc;

use super::Fact;

pub async fn prime(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    match n.is_probably_prime(30) {
        IsPrime::Yes => tx
            .send(Fact::Basic("Is a prime number.".to_string()))
            .await
            .unwrap(),
        IsPrime::Probably => tx
            .send(Fact::Basic(
                "Is almost certainly a prime number.".to_string(),
            ))
            .await
            .unwrap(),
        IsPrime::No => return,
    }

    if (&*n + 1_u8).complete().is_power_of_two() {
        let power = n.count_ones().unwrap();
        tx.send(Fact::Basic(format!(
            "Is a Mersenne prime: (#2)(^(#{power}))-(#1)"
        )))
        .await
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    #[test]
    fn no_composites() {
        crate::test_harness!(|(a in "[0-9]+", b in "[0-9]+")| {
            let a = Integer::parse(a).unwrap().complete() + 2;
            let b = Integer::parse(b).unwrap().complete() + 2;
            let x = a*b;

            let (tx, mut rx) = mpsc::channel(2);
            prime(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                None
            );
        });
    }

    #[test]
    fn mersenne() {
        crate::test_harness!(|| {
            let (tx, mut rx) = mpsc::channel(2);
            macro_rules! check {
                ($a:expr, $b:expr) => {
                    prime(Arc::new(Integer::from($a)), tx.clone()).await;
                    assert!(rx.recv().await.is_some());
                    assert_eq!(
                        rx.recv().await,
                        Some(Fact::Basic(format!(
                            "Is a Mersenne prime: (#2)(^(#{}))-(#1)",
                            $b
                        )))
                    );
                };
            }
            check!(3, 2);
            check!(7, 3);
            check!(31, 5);
            check!(127, 7);
            check!(8191, 13);
        });
    }
}
