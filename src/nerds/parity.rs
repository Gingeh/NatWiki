use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

pub async fn parity(n: Arc<Integer>, tx: mpsc::Sender<String>) {
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
    use tokio::runtime;

    proptest! {
        #[test]
        fn even(n in "[0-9]+") {
            runtime::Builder::new_current_thread().build().unwrap().block_on(async {
                let x = Integer::parse(n).unwrap().complete() * 2;
                let (tx, mut rx) = mpsc::channel(1);
                parity(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some("Is an even number.".to_string())
                )
            });
        }

        #[test]
        fn odd(n in "[0-9]+") {
            runtime::Builder::new_current_thread().build().unwrap().block_on(async {
                let x = Integer::parse(n).unwrap().complete() * 2 + 1;
                let (tx, mut rx) = mpsc::channel(1);
                parity(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some("Is an odd number.".to_string())
                )
            });
        }
    }
}
