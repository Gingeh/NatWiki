use std::sync::Arc;

use num_traits::Euclid;
use rug::Integer;
use tokio::sync::mpsc;

use super::Fact;

pub async fn encodings(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    tx.send(Fact::Form("Binary".to_owned(), format!("{:b}", n.as_ref())))
        .await
        .unwrap();
    tx.send(Fact::Form(
        "Hexadecimal".to_owned(),
        format!("{:X}", n.as_ref()),
    ))
    .await
    .unwrap();

    if let Some(roman) = n.to_u16().and_then(encode_roman) {
        tx.send(Fact::Form("Roman numerals".to_owned(), roman))
            .await
            .unwrap();
    }
}

fn encode_roman(n: u16) -> Option<String> {
    if n == 0 || n >= 4000 {
        return None;
    }

    let (n, u) = n.div_rem_euclid(&10);
    let (n, t) = n.div_rem_euclid(&10);
    let (n, h) = n.div_rem_euclid(&10);
    let (_, th) = n.div_rem_euclid(&10);

    let r_u = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"][u as usize];
    let r_t = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"][t as usize];
    let r_h = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"][h as usize];
    let r_th = ["", "M", "MM", "MMM"][th as usize];

    Some([r_th, r_h, r_t, r_u].into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    #[test]
    fn roundtrip_bin() {
        crate::test_harness!(|(n in "0|1[0-1]*")| {
            let x = Integer::from_str_radix(&n, 2).unwrap();
            let (tx, mut rx) = mpsc::channel(3);
            encodings(Arc::new(x), tx).await;
            prop_assert_eq!(
                rx.recv().await,
                Some(Fact::Form("Binary".into(), format!("{n}")))
            )
        });
    }

    #[test]
    fn roundtrip_hex() {
        crate::test_harness!(|(n in "0|[1-9A-F][0-9A-F]*")| {
            let x = Integer::from_str_radix(&n, 16).unwrap();
            let (tx, mut rx) = mpsc::channel(3);
            encodings(Arc::new(x), tx).await;
            rx.recv().await.unwrap();
            prop_assert_eq!(
                rx.recv().await,
                Some(Fact::Form("Hexadecimal".into(), format!("{n}")))
            )
        });
    }

    #[test]
    fn roman_examples() {
        assert_eq!(encode_roman(0), None);
        assert_eq!(encode_roman(123), Some("CXXIII".into()));
        assert_eq!(encode_roman(2024), Some("MMXXIV".into()));
        assert_eq!(encode_roman(3999), Some("MMMCMXCIX".into()));
        assert_eq!(encode_roman(4000), None);
    }
}
