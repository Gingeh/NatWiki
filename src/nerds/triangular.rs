use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

use super::Fact;

pub async fn triangular(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    let (disc, rem) = (Arc::unwrap_or_clone(n) * 8_u8 + 1_u8).sqrt_rem(Integer::new());
    if !rem.is_zero() || disc.is_even() {
        return;
    }
    let root = (disc - 1) / 2;
    tx.send(Fact::Basic(format!(
        "Is the (#{root})th triangular number."
    )))
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness;

    use proptest::prelude::*;
    use rug::Complete;

    #[test]
    fn positive_match() {
        test_harness!(|(n in "[0-9]+")| {
            let nth = Integer::parse(n).unwrap().complete();
            let x = (&nth + &nth*&nth).complete()/2;

            let (tx, mut rx) = mpsc::channel(1);
            triangular(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                Some(Fact::Basic(format!("Is the (#{nth})th triangular number.")))
            );
        });
    }
}
