use crate::{net_types::packet::Packet, packet::packet_pool::{ NullPacket, PingRequest, StatusRequest}};

pub fn id_to_packet(id: u32) -> Box<dyn Packet> {
    match id {
        0x00=>Box::new(StatusRequest::new()),
        0x01=>Box::new(PingRequest::new()),
        _ => Box::new(NullPacket::new()),
    }
}