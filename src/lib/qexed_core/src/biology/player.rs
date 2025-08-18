use uuid::Uuid;

pub struct Player {
    pub name: String,
    pub uuids: Uuid,
    pub entity_id: i32,

}
impl Player {
    pub fn new() -> Self {
        Player {
            name: String::new(),
            uuids: Uuid::new_v4(),
            entity_id: -1,
        }
    }

}