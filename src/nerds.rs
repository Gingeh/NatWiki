use num::BigUint;
use std::sync::Arc;

pub const NERDS: &[fn(Arc<BigUint>) -> Option<String>] = &[even_or_odd, triangular_number];

pub fn even_or_odd(n: Arc<BigUint>) -> Option<String> {
    Some(if n.bit(0) {
        "Is an odd number.".to_string()
    } else {
        "Is an even number.".to_string()
    })
}

pub fn triangular_number(n: Arc<BigUint>) -> Option<String> {
    let disc_sq = Arc::unwrap_or_clone(n) * 8_u8 + 1_u8;
    let disc = disc_sq.sqrt();

    // 8x+1 must be a square
    if disc.pow(2) != disc_sq {
        return None;
    }

    // sqrt(8x+1)-1 must be even
    if !disc.bit(0) {
        return None;
    }

    let root = (disc - 1_u8) / 2_u8;
    Some(format!("Is the {root}th triangular number."))
}
