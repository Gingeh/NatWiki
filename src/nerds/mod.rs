use std::sync::Arc;

use rug::Integer;
use thingbuf::{mpsc, recycling::WithCapacity};

mod factors;
mod parity;
mod power_form;
mod prime;
mod triangular;

pub async fn ask_nerds(n: Arc<Integer>) -> Vec<String> {
    let (tx, rx) = mpsc::with_recycle(1, WithCapacity::new());

    tokio::spawn(factors::factors(n.clone(), tx.clone()));
    tokio::spawn(parity::parity(n.clone(), tx.clone()));
    tokio::spawn(power_form::power_form(n.clone(), tx.clone()));
    tokio::spawn(prime::prime(n.clone(), tx.clone()));
    tokio::spawn(triangular::triangular(n.clone(), tx.clone()));
    drop(tx);

    let mut facts = Vec::new();
    while let Some(fact) = rx.recv().await {
        facts.push(fact);
    }
    facts
}
