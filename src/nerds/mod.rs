use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

mod encodings;
mod factors;
mod fibonacci;
mod parity;
mod power_form;
mod prime;
mod triangular;

#[derive(Default, Debug, Clone)]
pub struct NumberInfo {
    pub facts: Vec<String>,
    /// Alternate forms of the number, e.g. its binary or hex representation.
    /// Stored as tuple (description, alternate form).
    pub forms: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Fact {
    Basic(String),
    Form(String, String),
}

pub async fn ask_nerds(n: Arc<Integer>) -> NumberInfo {
    let (tx, mut rx) = mpsc::channel::<Fact>(1);

    tokio::spawn(encodings::encodings(n.clone(), tx.clone()));
    tokio::spawn(factors::factors(n.clone(), tx.clone()));
    tokio::spawn(fibonacci::fibonacci(n.clone(), tx.clone()));
    tokio::spawn(parity::parity(n.clone(), tx.clone()));
    tokio::spawn(power_form::power_form(n.clone(), tx.clone()));
    tokio::spawn(prime::prime(n.clone(), tx.clone()));
    tokio::spawn(triangular::triangular(n.clone(), tx.clone()));
    drop((n, tx));

    let mut info = NumberInfo::default();
    while let Some(fact) = rx.recv().await {
        match fact {
            Fact::Basic(s) => info.facts.push(s),
            Fact::Form(desc, form) => info.forms.push((desc, form)),
        }
    }
    info
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
