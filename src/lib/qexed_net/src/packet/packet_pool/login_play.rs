use crate::{
    net_types::{packet::Packet, position::Position, var_int::{ VarInt}},
    packet::{decode::PacketReader, encode::PacketWriter},
};

/// play阶段第一个数据包
#[derive(Debug, Default, PartialEq)]
pub struct LoginPlay {
    pub entity_id: VarInt,
    pub is_hardcore: bool,
    pub dimension_names: Vec<String>,
    pub max_player: VarInt,
    pub view_distance: VarInt,// 视距
    pub simulation_distance: VarInt,// 模拟距离
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type:VarInt,
    pub dimension_name:String,
    pub hashed_seed:i64,
    pub game_mode:u8,
    pub previous_game_mode:i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name:Option<String>,
    pub death_position: Option<Position>,
    pub portal_cooldown: VarInt,
    pub is_telemetry_enabled: bool,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,



}
impl LoginPlay {
    pub fn new() -> Self {
        LoginPlay {
            entity_id: VarInt(0),
            is_hardcore: false,
            dimension_names: Vec::new(),
            max_player: VarInt(0),
            view_distance:VarInt(0),
            simulation_distance: VarInt(0),
            reduced_debug_info: false,
            enable_respawn_screen: false,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: "".to_owned(),
            hashed_seed: 0,
            game_mode: 0,
            previous_game_mode: 0,
            is_debug: false,
            is_flat: false,
            has_death_location:false,
            death_dimension_name: None,
            death_position: None,
            portal_cooldown: VarInt(0),
            is_telemetry_enabled: false,
            sea_level: VarInt(0),
            enforces_secure_chat: false,
        }
    }
}
impl Packet for LoginPlay {
    fn id(&self) -> u32 {
        0x2b
    }
    fn serialize(&self,w: &mut PacketWriter) {
        w.serialize(&self.entity_id);
        w.serialize(&self.is_hardcore);
        w.serialize(&self.dimension_names);
        w.serialize(&self.max_player);
        w.serialize(&self.view_distance);
        w.serialize(&self.simulation_distance);
        w.serialize(&self.reduced_debug_info);
        w.serialize(&self.enable_respawn_screen);
        w.serialize(&self.do_limited_crafting);
        w.serialize(&self.dimension_type);
        w.serialize(&self.dimension_name);
        w.serialize(&self.hashed_seed);
        w.serialize(&self.game_mode);
        w.serialize(&self.previous_game_mode);
        w.serialize(&self.is_debug);
        w.serialize(&self.is_flat);
        w.serialize(&self.has_death_location);
        w.option_string(self.death_dimension_name.as_deref());
        w.option(self.death_position.as_ref());
        w.serialize(&self.portal_cooldown);
        w.serialize(&self.is_telemetry_enabled);
        w.serialize(&self.sea_level);
        w.serialize(&self.enforces_secure_chat);
    }
    fn deserialize(&mut self, r: &mut PacketReader) {
        self.entity_id = r.deserialize();
        self.is_hardcore = r.deserialize();
        self.dimension_names = r.deserialize();
        self.max_player = r.deserialize();
        self.view_distance = r.deserialize();
        self.simulation_distance = r.deserialize();
        self.reduced_debug_info = r.deserialize();
        self.enable_respawn_screen = r.deserialize();
        self.do_limited_crafting = r.deserialize();
        self.dimension_type = r.deserialize();
        self.dimension_name = r.deserialize();
        self.hashed_seed = r.deserialize();
        self.game_mode = r.deserialize();
        self.previous_game_mode = r.deserialize();
        self.is_debug = r.deserialize();
        self.is_flat = r.deserialize();
        self.has_death_location = r.deserialize();
        self.death_dimension_name = r.option_string();
        self.death_position = r.option();
        self.portal_cooldown = r.deserialize();
        self.is_telemetry_enabled = r.deserialize();
        self.sea_level = r.deserialize();
        self.enforces_secure_chat = r.deserialize();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
