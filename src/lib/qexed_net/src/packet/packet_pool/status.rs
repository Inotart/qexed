use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 查询状态数据包
/// 也是客户端发的第一个数据包,这个数据包是什么内容都没有
#[derive(Debug, Default, PartialEq)]
pub struct StatusRequest {}
impl StatusRequest {
    pub fn new() -> Self {StatusRequest {}}
}
impl Packet for StatusRequest {
    
    fn id(&self) -> u32 {0x00}
    fn serialize(&self, _w: &mut PacketWriter) {}
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
/// 服务端响应数据包
#[derive(Debug, Default, PartialEq)]
pub struct StatusResponse {
    pub json_response:serde_json::Value,
}
impl StatusResponse {
    pub fn new() -> Self {StatusResponse {json_response:serde_json::Value::String("{}".to_owned())}}
}
impl Packet for StatusResponse {
    
    fn id(&self) -> u32 {0x00}
    fn serialize(&self, w: &mut PacketWriter) {
        w.json(&self.json_response);
    }
    fn deserialize(&mut self, _r: &mut PacketReader) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
