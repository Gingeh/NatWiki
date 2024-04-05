use std::sync::Arc;

use rug::Integer;

pub fn triangular(n: Arc<Integer>) -> Option<String> {
    let (disc, rem) = (Arc::unwrap_or_clone(n) * 8_u8 + 1_u8).sqrt_rem(Integer::new());
    if !rem.is_zero() || disc.is_even() {
        return None;
    }
    let root = (disc - 1) / 2;
    Some(format!("Is the {root}th triangular number."))
}
