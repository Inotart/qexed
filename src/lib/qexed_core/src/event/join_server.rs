use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::{Utc,DateTime};
pub async fn alive_fn(alive_id: Arc<Mutex<u64>>,send_pk:Arc<Mutex<qexed_net::PacketListener>>){
    loop {
        let mut guard = alive_id.lock().await;
        let mut packet_socket = send_pk.lock().await;
        let now: DateTime<Utc> = Utc::now();
        *guard= now.timestamp_millis() as u64;
        let mut alive_pk = qexed_net::packet::packet_pool::KeepAliveClientPlay::new();
        alive_pk.alive_id = *guard;
        let _ = packet_socket.send(&alive_pk).await;
        drop(guard);
        drop(packet_socket);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}