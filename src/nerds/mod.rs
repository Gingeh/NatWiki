use std::sync::Arc;

use rug::Integer;

mod factors;
mod parity;
mod prime;
mod triangular;

pub const NERDS: &[fn(Arc<Integer>) -> Option<String>] = &[
    parity::parity,
    triangular::triangular,
    prime::prime,
    factors::factors,
];
