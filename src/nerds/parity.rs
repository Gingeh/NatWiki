use std::sync::Arc;

use rug::Integer;

pub fn parity(n: Arc<Integer>) -> Option<String> {
    Some(if n.is_even() {
        "Is an even number.".to_string()
    } else {
        "Is an odd number.".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use rug::Complete;

    proptest! {
        #[test]
        fn even(n in "[0-9]+") {
            let x = Integer::parse(n).unwrap().complete() * 2;
            prop_assert_eq!(
                parity(Arc::new(x)),
                Some("Is an even number.".to_string())
            );
        }

        #[test]
        fn odd(n in "[0-9]+") {
            let x = Integer::parse(n).unwrap().complete() * 2 + 1;
            prop_assert_eq!(
                parity(Arc::new(x)),
                Some("Is an odd number.".to_string())
            );
        }
    }
}
