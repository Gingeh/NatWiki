use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

use super::Fact;

pub async fn parity(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    tx.send(Fact::basic(if n.is_even() {
        "Is an even number."
    } else {
        "Is an odd number."
    }))
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use rug::Complete;

    #[test]
    fn even() {
        crate::test_harness!(|(n in "[0-9]+")| {
            let x = Integer::parse(n).unwrap().complete() * 2;
            let (tx, mut rx) = mpsc::channel(1);
            parity(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                Some(Fact::Basic("Is an even number.".to_string()))
            )
        });
    }

    #[test]
    fn odd() {
        crate::test_harness!(|(n in "[0-9]+")| {
            let x = Integer::parse(n).unwrap().complete() * 2 + 1;
            let (tx, mut rx) = mpsc::channel(1);
            parity(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                Some(Fact::Basic("Is an odd number.".to_string()))
            )
        });
    }
}
