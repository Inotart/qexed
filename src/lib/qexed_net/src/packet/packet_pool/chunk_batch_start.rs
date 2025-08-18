use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 数据包请求开始
#[derive(Debug, Default, PartialEq)]
pub struct ChunkBatchStart {}
impl ChunkBatchStart {
    pub fn new() -> Self {
        ChunkBatchStart {}
    }
}
impl Packet for ChunkBatchStart {
    fn id(&self) -> u32 {
        0x0C
    }
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
