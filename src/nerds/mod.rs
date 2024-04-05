use std::sync::Arc;

use rug::Integer;

mod parity;
mod prime;
mod triangular;

pub const NERDS: &[fn(Arc<Integer>) -> Option<String>] =
    &[parity::parity, triangular::triangular, prime::prime];
