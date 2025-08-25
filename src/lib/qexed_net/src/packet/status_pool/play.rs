use crate::net_types::packet::Packet;




pub fn id_to_packet(id: u32) -> Box<dyn Packet> {
    match id {
        0x00 => Box::new(crate::packet::packet_pool::AcceptTeleportation::new()),
        0x0C => Box::new(crate::packet::packet_pool::ChunkBatchStart::new()),
        0x1B => Box::new(crate::packet::packet_pool::KeepAliveServerPlay::new()),
        0x1D => Box::new(crate::packet::packet_pool::MovePlayerPos::new()),
        0x1E => Box::new(crate::packet::packet_pool::MovePlayerPosRot::new()),
        _ => Box::new(crate::packet::packet_pool::NullPacket::new()),
    }
}