use rust_event::GLOBAL_EVENT_BUS;
use std::sync::Arc;
use tokio::sync::Mutex;
// rust_event::event!(
//     #[doc = "原始插件消息事件,处理客户端发来的全部插件数据包,将对其进行处理后中转到其他的隧道处理单元"]
//     RawPluginMessageEvent<'a>(&Arc<qexed_net::PacketListener>,String,Vec<u8>)
// );
/// 原始插件消息事件,处理客户端发来的全部插件数据包,将对其进行处理后中转到其他的隧道处理单元
// 修改1: 为 Event trait 实现显式关联结构体的生命周期 'a
pub struct RawPluginMessageEvent {}

// 修改2: 在 impl 块中声明生命周期 'a 并绑定到 trait
impl<'a> rust_event::Event for RawPluginMessageEvent {
    // 修改3: 避免引用 Arc，直接存储 Arc 或按需克隆
    type Data = (Arc<Mutex<qexed_net::PacketListener>>, String, Vec<u8>);
}
rust_event::event_global_async!( 
    RawPluginMessageEvent,
    handle_plugin_message,
    (packet_socket, name, data)
);
pub async fn handle_plugin_message(
    packet_socket: Arc<Mutex<qexed_net::PacketListener>>,
    name: String,
    data: Vec<u8>,
) {
    let bus = GLOBAL_EVENT_BUS.clone();
    let channel: &str = &name;
    match channel {
        "minecraft:brand" => {
            let text = {
                // Create a BytesMut from the data
                let mut buf = bytes::BytesMut::new();
                buf.extend_from_slice(&data);
                // Create a reader that borrows the buf
                let mut reader = qexed_net::packet::decode::PacketReader::new(Box::new(&mut buf));
                // Read the string and return it
                reader.string()
            };
            bus.emit::<super::plugin_channels::minecraft::brand::BrandEvent>((
                text.clone(),
                Arc::clone(&packet_socket),
            )).await;
            
        }
        _ => {}
    }
    
}
