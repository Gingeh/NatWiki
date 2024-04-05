use std::sync::Arc;

use rug::{integer::IsPrime, Integer};

pub fn prime(n: Arc<Integer>) -> Option<String> {
    match n.is_probably_prime(30) {
        IsPrime::Yes => Some("Is a prime number.".to_string()),
        IsPrime::Probably => Some("Is almost certainly a prime number.".to_string()),
        IsPrime::No => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use rug::Complete;

    proptest! {
        #[test]
        fn no_composites(a in "[0-9]+", b in "[0-9]+") {
            let a = Integer::parse(a).unwrap().complete() + 2;
            let b = Integer::parse(b).unwrap().complete() + 2;
            let x = a*b;
            prop_assert_eq!(
                prime(Arc::new(x)),
                None
            );
        }
    }
}
