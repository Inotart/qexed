use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};
/// 插件消息包？(服务端->客户端)
/// id是0x01
#[derive(Debug, Default, PartialEq)]
pub struct PluginMessageServer {
    pub channel:String,
    pub data:Vec<u8>,
}
impl PluginMessageServer {
    pub fn new() -> Self {
        PluginMessageServer {
            channel:"".to_owned(),
            data:vec![]
        }
    }
}
impl Packet for PluginMessageServer {
    fn id(&self) -> u32 {
        0x01
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self.channel);
        w.byte_all(self.data.clone());

    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.channel = r.string();
        self.data = r.byte_all();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// 插件消息包？(客户端->服务端)
/// id是0x02
#[derive(Debug, Default, PartialEq)]
pub struct PluginMessage {
    pub channel:String,
    pub data:Vec<u8>,
}
impl PluginMessage {
    pub fn new() -> Self {
        PluginMessage {
            channel:"".to_owned(),
            data:vec![]
        }
    }
}
impl Packet for PluginMessage {
    fn id(&self) -> u32 {
        0x02
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self.channel);
        w.byte_all(self.data.clone());

    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.channel = r.string();
        self.data = r.byte_all();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
