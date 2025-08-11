use crate::{net_types::packet::Packet, packet::packet_pool::{ClientInformationCtoS, NullPacket, PluginMessage}};


pub fn id_to_packet(id: u32) -> Box<dyn Packet> {
    match id {
        0x00=> Box::new(ClientInformationCtoS::new()),
        0x02 => Box::new(PluginMessage::new()),
        _ => Box::new(NullPacket::new()),
    }
}