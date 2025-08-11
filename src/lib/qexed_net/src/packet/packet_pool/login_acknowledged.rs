use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct LoginAcknowledged{}
impl LoginAcknowledged {
    pub fn new() -> Self {
        LoginAcknowledged {}
    }
}
impl Packet for LoginAcknowledged {
    fn id(&self) -> u32 {
        0x03
    }
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
