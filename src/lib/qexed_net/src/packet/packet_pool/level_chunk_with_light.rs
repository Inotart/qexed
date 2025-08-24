use crate::{
    net_types::{chunk::Chunk, light::Light, packet::Packet, subdata::Subdata, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq)]
pub struct LevelChunkWithLight {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data:Chunk,
    pub light:Light,
}

impl LevelChunkWithLight {
    pub fn new() -> Self {
        LevelChunkWithLight {
            chunk_x: 0,
            chunk_z: 0,
            data:Chunk::new(),
            light:Light::new(),
        }
    }
}
impl Packet for LevelChunkWithLight {
    fn id(&self) -> u32 {
        0x27
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.chunk_x);
        w.serialize(&self.chunk_z);
        w.serialize(&self.data);
        w.serialize(&self.light);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.chunk_x = r.deserialize();
        self.chunk_z = r.deserialize();
        self.data = r.deserialize();
        self.light = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
