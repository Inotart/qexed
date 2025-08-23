use std::fmt::Display;

use crate::packet::decode::PacketReader;
use crate::packet::encode::PacketWriter;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PacketState {
    Configuration,
    Handshake,
    Login,
    Play,
    Status,
}

impl From<&str> for PacketState {
    fn from(value: &str) -> Self {
        match value {
            "configuration" => PacketState::Configuration,
            "handshake" => PacketState::Handshake,
            "login" => PacketState::Login,
            "play" => PacketState::Play,
            "status" => PacketState::Status,
            wrong => panic!("Invalid state: {wrong}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`"),
        }
    }
}
impl Display for PacketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketState::Configuration => write!(f, "configuration"),
            PacketState::Handshake => write!(f, "handshake"),
            PacketState::Login => write!(f, "login"),
            PacketState::Play => write!(f, "play"),
            PacketState::Status => write!(f, "status"),
        }
    }
}

// 定义 Packet trait 作为所有数据包的公共接口

pub trait Packet: std::fmt::Debug + Send + Sync{
    fn id(&self)->u32;
    fn serialize(&self, w: &mut PacketWriter);
    fn deserialize(&mut self, r: &mut PacketReader);
    fn as_any(&self) -> &dyn std::any::Any;
}