use mongodb::{bson::{self, doc, DateTime}, Collection, Database};
use qexed_net::net_types::{position::Position, subdata::Subdata};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use chrono::Utc;
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "_id")] // 使用 MongoDB 的 _id 字段
    pub uuid: Uuid,
    pub username: String,
    #[serde(skip)]
    pub entity_id: i32,
    pub level: u32,
    pub last_login: DateTime,
    // 暂时没有物品
    // pub inventory: Vec<Item>,
    pub position: Position,
    pub health: f32,
    pub is_online: bool,
}
impl Player {
    pub fn new() -> Self {
        Player {
            username: String::new(),
            uuid: Uuid::new_v4(),
            entity_id: -1,
            level: 0,
            last_login: DateTime::now(),
            position: Position::new(),
            health: 20.0, // 默认生命值
            is_online: false, // 默认不在线
            // inventory: Vec::new(), // 暂时没有物品
        }
    }

    /// 创建一个指定 UUID 的新玩家
    pub fn with_uuid(uuid: Uuid) -> Self {
        Player {
            username: String::new(),
            uuid,
            entity_id: -1,
            level: 0,
            last_login: DateTime::now(),
            position: Position::new(),
            health: 20.0,
            is_online: false,
            // inventory: Vec::new(),
        }
    }
    /// 获取 MongoDB 玩家集合
    pub fn get_player_collection(db: &Database) -> Collection<Player> {
        db.collection("players")
    }
    /// 根据 UUID 加载玩家，如果不存在则创建新玩家
    pub async fn load_or_create(
        db: &Database,
        uuid: Uuid,
    ) -> Result<Self> {
        let collection = Self::get_player_collection(db);
        
        // 尝试通过 UUID 查找玩家
        if let Some(player) = Self::find_by_uuid(&collection, uuid).await? {
            return Ok(player);
        }
        
        // 如果不存在，创建新玩家
        let new_player = Self::create_player(&collection, uuid).await?;
        Ok(new_player)
    }

    /// 根据 UUID 查找玩家
    pub async fn find_by_uuid(
        collection: &Collection<Player>,
        uuid: Uuid,
    ) -> Result<Option<Player>> {
        let filter = doc! { "_id": uuid.to_string() };
        collection.find_one(filter).await.map_err(|e| e.into())
    }

    /// 创建新玩家（使用指定 UUID）
    pub async fn create_player(
        collection: &Collection<Player>,
        uuid: Uuid,
    ) -> Result<Player> {
        let new_player = Player::with_uuid(uuid);
        collection.insert_one(&new_player).await?;
        Ok(new_player)
    }

    /// 保存玩家信息到数据库
    pub async fn save(&self, db: &Database) -> Result<()> {
        let collection = Self::get_player_collection(db);
        let filter = doc! { "_id": self.uuid.to_string() };
        collection.replace_one(filter, self).await?;
        Ok(())
    }

    /// 更新玩家位置
    pub async fn update_position(
        &mut self,
        db: &Database,
        position: Position,
    ) -> Result<()> {
        self.position = position;
        self.save(db).await
    }

    /// 设置玩家在线状态
    pub async fn set_online_status(
        &mut self,
        db: &Database,
        is_online: bool,
    ) -> Result<()> {
        self.is_online = is_online;
        self.last_login = bson::DateTime::now(); // 更新最后登录时间
        self.save(db).await
    }
}