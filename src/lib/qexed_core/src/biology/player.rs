use bson::{spec::BinarySubtype, Binary};
use mongodb::{bson::{self, doc, DateTime}, Collection, Database};
use qexed_net::net_types::{position::Position, subdata::Subdata};

use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use chrono::Utc;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(with = "uuid_as_binary")]
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
        let bin = Binary {
            subtype: BinarySubtype::Uuid,
            bytes: uuid.as_bytes().to_vec(),
        };
        let filter = doc! { "_id": bin  };
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
        let bin = Binary {
            subtype: BinarySubtype::Uuid,
            bytes: self.uuid.as_bytes().to_vec(),
        };
        let filter = doc! { "_id": bin  };
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
mod uuid_as_binary {
    use bson::{Binary, spec::BinarySubtype};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use uuid::Uuid;
    
    /// 将 UUID 序列化为 BSON Binary (subtype 4)
    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bin = Binary {
            subtype: BinarySubtype::Uuid,
            bytes: uuid.as_bytes().to_vec(),
        };
        // 直接序列化 Binary 对象
        bin.serialize(serializer)
    }
    
    /// 将 BSON Binary (subtype 4) 反序列化为 UUID
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bin = bson::Binary::deserialize(deserializer)?;
        
        // 验证二进制格式
        if bin.subtype != BinarySubtype::Uuid {
            return Err(D::Error::custom(format!(
                "Expected UUID binary subtype (4), got {:?}",
                bin.subtype
            )));
        }
        
        if bin.bytes.len() != 16 {
            return Err(D::Error::custom(format!(
                "Invalid UUID length: expected 16 bytes, got {}",
                bin.bytes.len()
            )));
        }
        
        // 转换为 UUID
        let bytes: [u8; 16] = bin.bytes
            .try_into()
            .map_err(|_| D::Error::custom("Failed to convert to UUID bytes"))?;
        
        Ok(Uuid::from_bytes(bytes))
    }
}