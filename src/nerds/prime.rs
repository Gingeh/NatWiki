use std::sync::Arc;

use rug::{integer::IsPrime, Integer};
use thingbuf::{mpsc::Sender, recycling::WithCapacity};

pub async fn prime(n: Arc<Integer>, tx: Sender<String, WithCapacity>) {
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
    use thingbuf::mpsc;
    use tokio::runtime::Runtime;

    proptest! {
        #[test]
        fn no_composites(a in "[0-9]+", b in "[0-9]+") {
            Runtime::new().unwrap().block_on(async {
                let a = Integer::parse(a).unwrap().complete() + 2;
                let b = Integer::parse(b).unwrap().complete() + 2;
                let x = a*b;

                let (tx, rx) = mpsc::with_recycle(1, WithCapacity::new());
                prime(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    None
                );
            });
        }
    }
}
