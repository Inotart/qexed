use crate::{
    net_types::{
        self, chunk::Chunk, light::Light, packet::Packet, subdata::Subdata, var_int::VarInt,
    },
    packet::{decode::PacketReader, encode::PacketWriter},
};

#[derive(Debug, Default, PartialEq)]
pub struct ChatMessageCtS {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub signature: Option<Vec<u8>>,
    pub message_count: VarInt,
    pub acknowledged: [u8; 3], // 20bit
    pub checksum: u8,
}

impl ChatMessageCtS {
    pub fn new() -> Self {
        ChatMessageCtS {
            message: "".to_owned(),
            timestamp: 0,
            salt: 0,
            signature: None,
            message_count: net_types::var_int::VarInt(0),
            acknowledged: [0u8; 3],
            checksum: 0,
        }
    }
}
impl Packet for ChatMessageCtS {
    fn id(&self) -> u32 {
        0x08
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.message);
        w.serialize(&self.timestamp);
        w.serialize(&self.salt);
        w.option(self.signature.as_ref());
        w.serialize(&self.message_count);
        w.serialize(&self.acknowledged);
        w.serialize(&self.checksum);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.message = r.deserialize();
        self.timestamp = r.deserialize();
        self.salt = r.deserialize();
        self.signature = r.option();
        self.message_count = r.deserialize();
        self.acknowledged = r.deserialize();
        self.checksum = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
// 暂未实现
/* 
#[derive(Debug, Default, PartialEq)]
pub struct PlayerChatStC {
    // Header
    pub Global_Index:VarInt,
    pub Sender:uuid::Uuid,
    pub Index:VarInt,
    pub Message_Signature_bytes:Option<Vec<u8>>,
    // Body
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    // 未知字段,也未命名
    pub unknown:Vec<UnknownPlayerChat>,
    // Other
    pub message_id:VarInt,
    pub signature: Option<Vec<u8>>,
    pub message_count: VarInt,
    pub acknowledged: [u8; 4], // 20bit
    pub checksum: u8,
}
impl Packet for PlayerChatStC {
    fn id(&self) -> u32 {
        0x3A
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.message);
        w.serialize(&self.timestamp);
        w.serialize(&self.salt);
        w.option(self.signature.as_ref());
        w.serialize(&self.message_count);
        w.serialize(&self.acknowledged);
        w.serialize(&self.checksum);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.message = r.deserialize();
        self.timestamp = r.deserialize();
        self.salt = r.deserialize();
        self.signature = r.option();
        self.message_count = r.deserialize();
        self.acknowledged = r.deserialize();
        self.checksum = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
*/
#[derive(Debug, Default, PartialEq)]
pub struct UnknownPlayerChat{
    pub message_id:VarInt,
    pub signature: Option<Vec<u8>>,
}
impl Subdata for UnknownPlayerChat {
    fn new() -> Self {
        UnknownPlayerChat {
            message_id:net_types::var_int::VarInt(0),
            signature:None,
        }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.message_id);
        w.option(self.signature.as_ref());
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.message_id = r.deserialize();
        self.signature = r.option();
    }
}
