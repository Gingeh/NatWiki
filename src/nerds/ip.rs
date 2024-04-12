use std::sync::Arc;

use rug::Integer;
use tokio::sync::mpsc;

use super::Fact;

pub async fn ip(n: Arc<Integer>, tx: mpsc::Sender<Fact>) {
    if let Some(addr) = n.to_u128().map(std::net::Ipv6Addr::from) {
        tx.send(Fact::form("IPv6 address", format!("(`{addr})")))
            .await
            .unwrap();
    }

    if let Some(addr) = n.to_u32().map(std::net::Ipv4Addr::from) {
        tx.send(Fact::form("IPv4 address", format!("(`{addr})")))
            .await
            .unwrap();
    }
}
