use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub event_status: u8,
}
impl EntityEvent {
    pub fn new() -> Self {
        EntityEvent {
            entity_id: 0,
            event_status: 0,
        }
    }
}
impl Packet for EntityEvent {
    fn id(&self) -> u32 {
        0x1E
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.entity_id);
        w.serialize(&self.event_status);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.entity_id = r.deserialize();
        self.event_status = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
