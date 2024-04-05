use std::sync::Arc;

use rug::Integer;

pub const NERDS: &[fn(Arc<Integer>) -> Option<String>] = &[even_or_odd, triangular_number];

fn even_or_odd(n: Arc<Integer>) -> Option<String> {
    Some(if n.is_even() {
        "Is an even number.".to_string()
    } else {
        "Is an odd number.".to_string()
    })
}

fn triangular_number(n: Arc<Integer>) -> Option<String> {
    let (disc, rem) = (Arc::unwrap_or_clone(n) * 8_u8 + 1_u8).sqrt_rem(Integer::new());
    if !rem.is_zero() || disc.is_even() {
        return None;
    }
    let root = (disc - 1) / 2;
    Some(format!("Is the {root}th triangular number."))
}
