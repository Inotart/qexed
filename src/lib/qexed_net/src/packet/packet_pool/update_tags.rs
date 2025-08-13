use crate::{
    net_types::{packet::Packet, subdata::Subdata, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq)]
pub struct UpdateTags {
    pub tags:Vec<Tags>,
}
impl UpdateTags {
    pub fn new() -> Self {
        UpdateTags {
            tags:vec![],
        }
    }
}
impl Packet for UpdateTags {
    fn id(&self) -> u32 {
        0x0D
    }
    fn serialize(&self, w: &mut PacketWriter) {
         w.serialize(&self.tags);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.tags = r.vec();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct Tags {
    pub registry: String,
    pub tags: Vec<Tag>,
}
impl Subdata for Tags{
    fn new() -> Self {
        Tags {
            registry:"".to_owned(),
            tags:vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.registry);
        w.serialize(&self.tags);

    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.registry = r.string();
        self.tags = r.vec();
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct Tag {
    pub name: String,
    pub entries: Vec<VarInt>,
}
impl Subdata for Tag{
    fn new() -> Self {
        Tag {
            name:"".to_owned(),
            entries:vec![],
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.name);
        w.serialize(&self.entries);

    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.name = r.string();
        self.entries = r.vec();
    }
}
