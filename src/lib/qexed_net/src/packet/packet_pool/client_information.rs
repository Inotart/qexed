use crate::{
    net_types::{packet::Packet, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 由客户端发至服务端
#[derive(Debug, Default, PartialEq)]
pub struct ClientInformationCtoS {
    pub locale: String,    // e.g. en_GB.
    pub view_distance: i8, // Client-side render distance, in chunks.
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool, // 事实上这个没用(你又不是机器人)
    pub particle_status: VarInt,
}
impl ClientInformationCtoS {
    pub fn new() -> Self {
        ClientInformationCtoS {
            locale: "".to_owned(),
            view_distance: 0,
            chat_mode: VarInt(0),
            chat_colors: true,
            displayed_skin_parts: 0,
            main_hand: VarInt(0),
            enable_text_filtering: true,
            allow_server_listings: true,
            particle_status: VarInt(0),
        }
    }
}
impl Packet for ClientInformationCtoS {
    fn id(&self) -> u32 {
        0x00
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self.locale);
        w.i8(self.view_distance);
        w.varint(&self.chat_mode);
        w.bool(self.chat_colors);
        w.u8(self.displayed_skin_parts);
        w.varint(&self.main_hand);
        w.bool(self.enable_text_filtering);
        w.bool(self.allow_server_listings);
        w.varint(&self.particle_status);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.locale = r.string();
        self.view_distance = r.i8();
        self.chat_mode = r.varint();
        self.chat_colors = r.bool();
        self.displayed_skin_parts = r.u8();
        self.main_hand = r.varint();
        self.enable_text_filtering = r.bool();
        self.allow_server_listings = r.bool();
        self.particle_status = r.varint();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
