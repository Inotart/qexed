use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct MovePlayerPos {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: i8,
}
impl MovePlayerPos {
    pub fn new() -> Self {
        MovePlayerPos {
            x: 0.0,
            feet_y: 0.0,
            z: 0.0,
            flags: 0,
        }
    }
}
impl Packet for MovePlayerPos {
    fn id(&self) -> u32 {
        0x1D
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.x);
        w.serialize(&self.feet_y);
        w.serialize(&self.z);
        w.serialize(&self.flags);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.x = r.deserialize();
        self.feet_y = r.deserialize();
        self.z = r.deserialize();
        self.flags = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
