use crate::{net_types::packet::Packet, packet::packet_pool::{ LoginAcknowledged, LoginStart, NullPacket}};


pub fn id_to_packet(id: u32) -> Box<dyn Packet> {
    match id {
        0x00 => Box::new(LoginStart::new()),
        // 1和2暂时没实现
        0x03 =>Box::new(LoginAcknowledged::new()),
        _ => Box::new(NullPacket::new()),
    }
}