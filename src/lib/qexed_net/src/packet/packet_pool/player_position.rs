use crate::{
    net_types::{packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct PlayerPosition{
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32,
}
impl PlayerPosition {
    pub fn new() -> Self {
        PlayerPosition {
            teleport_id:VarInt(0),
            x    :0.0,
            y    :0.0,
            z    :0.0,
            vel_x:0.0,
            vel_y:0.0,
            vel_z:0.0,
            yaw  :0.0,
            pitch:0.0,
            flags:0,
        }
    }
}
impl Packet for PlayerPosition {
    fn id(&self) -> u32 {
        0x41
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.teleport_id);
        w.serialize(&self.x    );
        w.serialize(&self.y    );
        w.serialize(&self.z    );
        w.serialize(&self.vel_x);
        w.serialize(&self.vel_y);
        w.serialize(&self.vel_z);
        w.serialize(&self.yaw  );
        w.serialize(&self.pitch);
        w.serialize(&self.flags);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.teleport_id = r.deserialize();
        self.x     = r.deserialize();
        self.y     = r.deserialize();
        self.z     = r.deserialize();
        self.vel_x = r.deserialize();
        self.vel_y = r.deserialize();
        self.vel_z = r.deserialize();
        self.yaw   = r.deserialize();
        self.pitch = r.deserialize();
        self.flags = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
