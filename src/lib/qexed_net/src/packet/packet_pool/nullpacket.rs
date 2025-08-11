use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct NullPacket {}
impl NullPacket {
    pub fn new() -> Self {
        NullPacket {}
    }
}
impl Packet for NullPacket {
    fn id(&self) -> u32 {
        0xfff
    }
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
