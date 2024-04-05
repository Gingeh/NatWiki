use std::sync::Arc;

use rug::Integer;

pub fn parity(n: Arc<Integer>) -> Option<String> {
    Some(if n.is_even() {
        "Is an even number.".to_string()
    } else {
        "Is an odd number.".to_string()
    })
}
