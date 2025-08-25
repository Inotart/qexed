use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}
impl EncryptionRequest {
    pub fn new() -> Self {
        EncryptionRequest {
            server_id: "Qexed".to_owned(),
            public_key: vec![],
            verify_token: vec![],
            should_authenticate: true,
        }
    }
}
impl Packet for EncryptionRequest {
    fn id(&self) -> u32 {
        0x01
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.server_id);
        w.serialize(&self.public_key);
        w.serialize(&self.verify_token);
        w.serialize(&self.should_authenticate);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.server_id = r.deserialize();
        self.public_key = r.deserialize();
        self.verify_token = r.deserialize();
        self.should_authenticate = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
impl EncryptionResponse {
    pub fn new() -> Self {
        EncryptionResponse {
            shared_secret: vec![],
            verify_token: vec![],
        }
    }
}
impl Packet for EncryptionResponse {
    fn id(&self) -> u32 {
        0x01
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.shared_secret);
        w.serialize(&self.verify_token);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.shared_secret = r.deserialize();
        self.verify_token = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
