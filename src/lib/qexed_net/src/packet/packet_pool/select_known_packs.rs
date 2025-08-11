use crate::{
    net_types::{packet::Packet, subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// select_known_packs 数据包:服务端->客户端
#[derive(Debug, Default, PartialEq)]
pub struct SelectKnownPacks {
    pub known_packs: Vec<KnownPacks>,
}


impl SelectKnownPacks {
    pub fn new() -> Self {
        SelectKnownPacks {
            known_packs: vec![],
        }
    }
}
impl Packet for SelectKnownPacks {
    fn id(&self) -> u32 {
        0x0E
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.known_packs);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.known_packs = r.deserialize()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct SelectKnownPacksCtoS {
    pub known_packs: Vec<KnownPacks>,
}
impl SelectKnownPacksCtoS {
    pub fn new() -> Self {
        SelectKnownPacksCtoS {
            known_packs: vec![],
        }
    }
}
impl Packet for SelectKnownPacksCtoS {
    fn id(&self) -> u32 {
        0x07
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.known_packs);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.known_packs = r.deserialize()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct KnownPacks {
    pub namespace:String,
    pub id:String,
    pub version:String,
}
impl Subdata for KnownPacks{
    fn new() -> Self {
        KnownPacks {
            namespace:"".to_owned(),
            id:"".to_owned(),
            version:"".to_owned(),
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.namespace);
        w.serialize(&self.id);
        w.serialize(&self.version);

    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.namespace = r.string();
        self.id = r.string();
        self.version = r.string();
    }
}
