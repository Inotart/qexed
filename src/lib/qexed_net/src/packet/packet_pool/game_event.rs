use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct GameEvent {
    pub event:u8,
    pub value:f32,
}
impl GameEvent {
    pub fn new() -> Self {
        GameEvent {
            event:0,
            value:0.0
        }
    }
}
impl Packet for GameEvent {
    fn id(&self) -> u32 {
        0x22
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.event);
        w.serialize(&self.value);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.event = r.deserialize();
        self.value = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
