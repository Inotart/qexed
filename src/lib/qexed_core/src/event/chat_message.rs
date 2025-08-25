use rust_event::GLOBAL_EVENT_BUS;
use std::sync::Arc;
use tokio::sync::Mutex;
// rust_event::event!(
//     #[doc = "原始插件消息事件,处理客户端发来的全部插件数据包,将对其进行处理后中转到其他的隧道处理单元"]
//     RawPluginMessageEvent<'a>(&Arc<qexed_net::PacketListener>,String,Vec<u8>)
// );
/// 原始插件消息事件,处理客户端发来的全部插件数据包,将对其进行处理后中转到其他的隧道处理单元
// 修改1: 为 Event trait 实现显式关联结构体的生命周期 'a
pub struct RawChatMessageEvent {}

// 修改2: 在 impl 块中声明生命周期 'a 并绑定到 trait
impl<'a> rust_event::Event for RawChatMessageEvent {
    // 修改3: 避免引用 Arc，直接存储 Arc 或按需克隆
    type Data = (Arc<Mutex<qexed_net::PacketListener>>, String, Vec<u8>);
}
