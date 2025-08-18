use crate::{
    net_types::{packet::Packet},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 配置阶段取消连接数据包
/// 由客户端发至服务端
#[derive(Debug, Default, PartialEq)]
pub struct disconnect_configuration {
    pub reason:serde_json::Value,
}
impl disconnect_configuration {
    pub fn new() -> Self {
        disconnect_configuration {
            reason:serde_json::Value::Null,
        }
    }
}
impl Packet for disconnect_configuration {
    fn id(&self) -> u32 {
        0x02
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.json(&self.reason);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.reason = r.json();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
