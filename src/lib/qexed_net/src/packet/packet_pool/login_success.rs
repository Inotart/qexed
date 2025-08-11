use crate::{
    net_types::{packet::Packet, subdata::Subdata},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// 登录阶段2号数据包
/// 登录成功数据包(离线模式直接发送)
#[derive(Debug, Default, PartialEq)]
pub struct LoginSuccess {
    pub uuids: uuid::Uuid,
    pub name: String,
    pub property: Vec<Property>,
}
impl LoginSuccess {
    pub fn new() -> Self {
        LoginSuccess {
            uuids: uuid::Uuid::nil(),
            name: "".to_string(),
            property: vec![],
        }
    }
}
impl Packet for LoginSuccess {
    fn id(&self) -> u32 {
        0x02
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.uuid(&self.uuids);
        w.string(&self.name);
        w.vec(&self.property);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.uuids = r.uuid();
        self.name = r.string();
        self.property.deserialize(r);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
impl Subdata for Property {
    fn new()->Self{
        Property { name: "".to_string(), value:  "".to_string(), signature: None }
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.string(&self.name);
        w.string(&self.value);
        w.option_string(self.signature.as_deref());
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        self.name = r.string();
        self.value = r.string();
        self.signature = r.option_string();
    }
}
