use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct MovePlayerPosRot {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
}
impl MovePlayerPosRot {
    pub fn new() -> Self {
        MovePlayerPosRot {
            x: 0.0,
            feet_y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        }
    }
}
impl Packet for MovePlayerPosRot {
    fn id(&self) -> u32 {
        0x1E
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.x);
        w.serialize(&self.feet_y);
        w.serialize(&self.z);
        w.serialize(&self.yaw);
        w.serialize(&self.pitch);
        w.serialize(&self.flags);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.x = r.deserialize();
        self.feet_y = r.deserialize();
        self.z = r.deserialize();
        self.yaw = r.deserialize();
        self.pitch = r.deserialize();
        self.flags = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
