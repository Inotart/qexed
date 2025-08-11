use crate::{net_types::packet::Packet, packet::{packet_pool::{Handshake, NullPacket}}};


pub fn id_to_packet(id: u32) -> Box<dyn Packet> {
    match id {
        0x00 => Box::new(Handshake::new()),
        _ => Box::new(NullPacket::new()),
    }
}