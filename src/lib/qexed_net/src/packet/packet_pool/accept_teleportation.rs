use crate::{
    net_types::{packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 空数据包,处理报错的
#[derive(Debug, Default, PartialEq)]
pub struct AcceptTeleportation{
    pub teleport_id: VarInt,
}
impl AcceptTeleportation {
    pub fn new() -> Self {
        AcceptTeleportation {
            teleport_id:VarInt(0),
        }
    }
}
impl Packet for AcceptTeleportation {
    fn id(&self) -> u32 {
        0x00
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.teleport_id);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.teleport_id = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
