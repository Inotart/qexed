use crate::{
    net_types::{self, packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq)]
pub struct LoginCompression {
    pub threshold: VarInt,
}
impl LoginCompression {
    pub fn new() -> Self {
        LoginCompression {
            threshold: net_types::var_int::VarInt(-1),
        }
    }
}
impl Packet for LoginCompression {
    fn id(&self) -> u32 {
        0x03
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.threshold);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.threshold = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}