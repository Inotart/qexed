use crate::{
    net_types::{packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 握手阶段0号数据包
#[derive(Debug, Default, PartialEq)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
impl Handshake {
    pub fn new() -> Self {
        Handshake {
            protocol_version: VarInt(-1),
            server_address: "localhost".to_string(),
            server_port: 25565,
            next_state: VarInt(2),
        }
    }
}
impl Packet for Handshake {
    fn id(&self) -> u32 {
        0x00
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.varint(&self.protocol_version);
        w.string(&self.server_address);
        w.u16(self.server_port);
        w.varint(&self.next_state);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.protocol_version = r.varint();
        self.server_address = r.string();
        self.server_port = r.u16();
        self.next_state = r.varint();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
