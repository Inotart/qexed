use crate::{
    net_types::packet::Packet,
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 查询状态数据包
/// 也是客户端发的第一个数据包,这个数据包是什么内容都没有
#[derive(Debug, Default, PartialEq)]
pub struct PingRequest {
    pub payload:i64,
}
impl PingRequest {
    pub fn new() -> Self {PingRequest {payload:0}}
}
impl Packet for PingRequest {
    
    fn id(&self) -> u32 {0x01}
    fn serialize(&self, w: &mut PacketWriter) {
        w.i64(self.payload);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.payload = r.i64();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
/// 服务端响应数据包
#[derive(Debug, Default, PartialEq)]
pub struct PingResponse {
    pub payload:i64,
}
impl PingResponse {
    pub fn new() -> Self {PingResponse {payload:0}}
}
impl Packet for PingResponse {
    
    fn id(&self) -> u32 {0x01}
    fn serialize(&self, w: &mut PacketWriter) {
        w.i64(self.payload);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.payload = r.i64();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
