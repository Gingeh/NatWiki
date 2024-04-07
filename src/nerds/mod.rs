use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

mod encodings;
mod factors;
mod parity;
mod power_form;
mod prime;
mod triangular;

pub async fn ask_nerds(n: Arc<Integer>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(encodings::encodings(n.clone(), tx.clone()));
    tokio::spawn(factors::factors(n.clone(), tx.clone()));
    tokio::spawn(parity::parity(n.clone(), tx.clone()));
    tokio::spawn(power_form::power_form(n.clone(), tx.clone()));
    tokio::spawn(prime::prime(n.clone(), tx.clone()));
    tokio::spawn(triangular::triangular(n.clone(), tx.clone()));
    drop((n, tx));

    let mut facts = Vec::new();
    while let Some(fact) = rx.recv().await {
        facts.push(fact);
    }
    facts
}

#[macro_export]
macro_rules! test_harness {
    (|| $body:block) => {
        // don't use proptest
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async move $body);
    };
    (|($($v:pat in $e:expr),+)| $body:block) => {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        proptest!(|($($v in $e),*)| {
            rt.block_on(async move {
                $body;
                Ok(())
            })?;
        });
    };
}
