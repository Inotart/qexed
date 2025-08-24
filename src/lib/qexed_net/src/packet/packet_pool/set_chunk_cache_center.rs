use crate::{
    net_types::{packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct SetChunkCacheCenter {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
}
impl SetChunkCacheCenter {
    pub fn new() -> Self {
        SetChunkCacheCenter {
            chunk_x: VarInt(0),
            chunk_z: VarInt(0),
        }
    }
}
impl Packet for SetChunkCacheCenter {
    fn id(&self) -> u32 {
        0x57
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.chunk_x);
        w.serialize(&self.chunk_z);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.chunk_x = r.deserialize();
        self.chunk_z = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
