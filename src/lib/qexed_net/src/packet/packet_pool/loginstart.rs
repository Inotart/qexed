use crate::{
    net_types::{packet::Packet},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 登录阶段0号数据包
/// 由客户端发至服务端
#[derive(Debug, Default, PartialEq)]
pub struct LoginStart {
    pub player_name:String,
    pub player_uuid:uuid::Uuid,
}
impl LoginStart {
    pub fn new() -> Self {
        LoginStart {
            player_name:"".to_string(),
            player_uuid:uuid::Uuid::nil(),
        }
    }
}
impl Packet for LoginStart {
    fn id(&self) -> u32 {
        0x00
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self.player_name);
        w.uuid(&self.player_uuid);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.player_name = r.string();
        self.player_uuid = r.uuid();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
