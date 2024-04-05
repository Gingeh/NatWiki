use std::sync::Arc;

use rug::Integer;

mod parity;
mod triangular;

pub const NERDS: &[fn(Arc<Integer>) -> Option<String>] = &[parity::parity, triangular::triangular];
