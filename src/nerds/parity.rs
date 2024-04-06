use std::sync::Arc;

use rug::Integer;
use thingbuf::{mpsc::Sender, recycling::WithCapacity};

pub async fn parity(n: Arc<Integer>, tx: Sender<String, WithCapacity>) {
    tx.send(if n.is_even() {
        "Is an even number.".to_string()
    } else {
        "Is an odd number.".to_string()
    })
    .await
    .unwrap();
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
        fn even(n in "[0-9]+") {
            Runtime::new().unwrap().block_on(async {
                let x = Integer::parse(n).unwrap().complete() * 2;
                let (tx, rx) = mpsc::with_recycle(1, WithCapacity::new());
                parity(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some("Is an even number.".to_string())
                )
            });
        }

        #[test]
        fn odd(n in "[0-9]+") {
            Runtime::new().unwrap().block_on(async {
                let x = Integer::parse(n).unwrap().complete() * 2 + 1;
                let (tx, rx) = mpsc::with_recycle(1, WithCapacity::new());
                parity(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some("Is an odd number.".to_string())
                )
            });
        }
    }
}
