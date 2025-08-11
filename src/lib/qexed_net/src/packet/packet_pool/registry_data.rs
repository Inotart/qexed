use crate::{
    net_types::{packet::Packet, subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// Represents certain registries that are sent from the server and are applied on the client.
/// 服务端->客户端
#[derive(Debug, Default, PartialEq)]
pub struct RegistryData {
    pub registry_id: String,
    pub entries:Vec<Entries>,
}
impl RegistryData {
    pub fn new() -> Self {
        RegistryData {
            registry_id:"".to_owned(),
            entries:vec![],
        }
    }
}
impl Packet for RegistryData {
    fn id(&self) -> u32 {
        0x07
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.registry_id);
        w.serialize(&self.entries);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.registry_id = r.deserialize();
        self.entries = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct Entries {
    pub entry_id: String,
    pub data: Option<crab_nbt::Nbt>,
}
impl Subdata for Entries{
    fn new() -> Self {
        Entries {
            entry_id:"".to_owned(),
            data:None,
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.entry_id);
        w.option(self.data.as_ref());

    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.entry_id = r.string();
        self.data = r.option();
    }
}
