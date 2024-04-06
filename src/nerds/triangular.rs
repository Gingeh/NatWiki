use std::sync::Arc;

use rug::Integer;
use thingbuf::{mpsc::Sender, recycling::WithCapacity};

pub async fn triangular(n: Arc<Integer>, tx: Sender<String, WithCapacity>) {
    let (disc, rem) = (Arc::unwrap_or_clone(n) * 8_u8 + 1_u8).sqrt_rem(Integer::new());
    if !rem.is_zero() || disc.is_even() {
        return;
    }
    let root = (disc - 1) / 2;
    tx.send(format!("Is the {root}th triangular number."))
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
        fn positive_match(n in "[0-9]+") {
            Runtime::new().unwrap().block_on(async {
                let nth = Integer::parse(n).unwrap().complete();
                let x = (&nth + &nth*&nth).complete()/2;

                let (tx, rx) = mpsc::with_recycle(1, WithCapacity::new());
                triangular(Arc::new(x), tx).await;
                assert_eq!(
                    rx.recv().await,
                    Some(format!("Is the {nth}th triangular number."))
                );
            });
        }
    }
}
