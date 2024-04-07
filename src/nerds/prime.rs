use std::sync::Arc;

use rug::{integer::IsPrime, Integer};
use tokio::sync::mpsc;

pub async fn prime(n: Arc<Integer>, tx: mpsc::Sender<String>) {
    match n.is_probably_prime(30) {
        IsPrime::Yes => tx.send("Is a prime number.".to_string()).await.unwrap(),
        IsPrime::Probably => tx
            .send("Is almost certainly a prime number.".to_string())
            .await
            .unwrap(),
        IsPrime::No => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use rug::Complete;

    #[test]
    fn no_composites() {
        crate::test_harness!(|(a in "[0-9]+", b in "[0-9]+")| {
            let a = Integer::parse(a).unwrap().complete() + 2;
            let b = Integer::parse(b).unwrap().complete() + 2;
            let x = a*b;

            let (tx, mut rx) = mpsc::channel(1);
            prime(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                None
            );
        });
    }
}
