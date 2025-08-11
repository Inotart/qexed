use crate::{
    net_types::{packet::Packet},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 登录阶段0号数据包
/// 由客户端发至服务端
#[derive(Debug, Default, PartialEq)]
pub struct DisconnectLogin {
    pub reason:serde_json::Value,
}
impl DisconnectLogin {
    pub fn new() -> Self {
        DisconnectLogin {
            reason:serde_json::Value::Null,
        }
    }
}
impl Packet for DisconnectLogin {
    fn id(&self) -> u32 {
        0x00
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
