use crate::{
    net_types::{packet::Packet},
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq)]
pub struct KeepAliveServerPlay{
    pub alive_id: u64,
}
impl KeepAliveServerPlay {
    pub fn new() -> Self {
        KeepAliveServerPlay {
            alive_id:0,
        }
    }
}
impl Packet for KeepAliveServerPlay {
    fn id(&self) -> u32 {
        0x1B
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.alive_id);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.alive_id = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct KeepAliveClientPlay{
    pub alive_id: u64,
}
impl KeepAliveClientPlay {
    pub fn new() -> Self {
        KeepAliveClientPlay {
            alive_id:0,
        }
    }
}
impl Packet for KeepAliveClientPlay {
    fn id(&self) -> u32 {
        0x26
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.alive_id);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.alive_id = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
