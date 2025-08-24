use crate::{
    net_types::{subdata::Subdata, var_int::VarInt},
    packet::{decode::PacketReader, encode::PacketWriter},
};
#[derive(Debug, Default, PartialEq)]
pub struct Bitset(pub Vec<u64>);
impl Subdata for Bitset {
    fn new() -> Self {
        Bitset(vec![])
    }
    fn serialize(&self, w: &mut PacketWriter) {
        w.serialize(&self.0);
    }

    fn deserialize(&mut self, r: &mut PacketReader) {
        *self = r.deserialize()
    }
}