use std::sync::Arc;

use rug::Integer;

mod factors;
mod parity;
mod power_form;
mod prime;
mod triangular;

pub const NERDS: &[fn(Arc<Integer>) -> Option<String>] = &[
    parity::parity,
    triangular::triangular,
    prime::prime,
    factors::factors,
    power_form::power_form,
];
