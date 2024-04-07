use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

pub async fn triangular(n: Arc<Integer>, tx: mpsc::Sender<String>) {
    let (disc, rem) = (Arc::unwrap_or_clone(n) * 8_u8 + 1_u8).sqrt_rem(Integer::new());
    if !rem.is_zero() || disc.is_even() {
        return;
    }
    let root = (disc - 1) / 2;
    tx.send(format!("Is the (#{root})th triangular number."))
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
        fn positive_match(n in "[0-9]+") {
            runtime::Builder::new_current_thread().build().unwrap().block_on(async {
                let nth = Integer::parse(n).unwrap().complete();
                let x = (&nth + &nth*&nth).complete()/2;

                let (tx, mut rx) = mpsc::channel(1);
                triangular(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some(format!("Is the (#{nth})th triangular number."))
                );
            });
        }
    }
}
