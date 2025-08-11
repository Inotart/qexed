use std::sync::Arc;

use qexed_net::packet::packet_pool::{PluginMessageServer};
use tokio::sync::Mutex;

rust_event::event!(
    #[doc = "在玩家登录后立即宣布服务器和客户端实现名称。"] 
    BrandEvent(String,Arc<Mutex<qexed_net::PacketListener>>)
);
rust_event::event_global_async!(BrandEvent, handle_plugin_message,(a,b));
pub async fn handle_plugin_message(data:String,packet_socket:Arc<Mutex<qexed_net::PacketListener>>) {
    let mut socket = packet_socket.lock().await;
    if let Some(player) = &mut socket.player{
       player.client_type = data;
       // 暂时不关注客户端类型的实现
       let mut pk = PluginMessageServer::new();
       pk.channel = "minecraft:brand".to_owned();
       pk.data = {
                let mut buf = bytes::BytesMut::new();
                let mut writer = qexed_net::packet::encode::PacketWriter::new(&mut buf);
                // Read the string and return it
                writer.string("vanilla");
                buf.freeze().to_vec()
            };
       let _ = socket.send(&pk).await;
    }
}
