use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct FinishConfigurationStoC {}
impl FinishConfigurationStoC {
    pub fn new() -> Self {
        FinishConfigurationStoC {}
    }
}
impl Packet for FinishConfigurationStoC {
    fn id(&self) -> u32 {
        0x03
    }
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct FinishConfigurationCtoS {}
impl FinishConfigurationCtoS {
    pub fn new() -> Self {
        FinishConfigurationCtoS {}
    }
}
impl Packet for FinishConfigurationCtoS {
    fn id(&self) -> u32 {
        0x03
    }
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
