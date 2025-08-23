use anyhow::Ok;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// 全局缓存 (线程安全)
static PROFILE_CACHE: Lazy<Arc<Mutex<HashMap<Uuid, Option<MojangProfile>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// 定义 Mojang 用户资料结构
#[derive(Debug, Clone, Deserialize)]
pub struct MojangProfile {
    pub id: String,
    pub name: String,
    // 其他可能的字段如 properties 等
}

// 异步查询 Mojang API 获取用户名（带缓存）
pub async fn query_mojang_for_usernames(uuids: Uuid) -> Result<MojangProfile, anyhow::Error> {
    if uuids.is_nil() {
        return Err(NoMojangPlayer::NoMojang.into());
    }

    let client = Client::new();

    // 先尝试从缓存获取
    if let Some(cached) = get_from_cache(&uuids) {
        if let Some(profile) = cached {
            return Ok(profile);
        }
    }

    // 缓存未命中，查询 Mojang API
    let profile = fetch_single_profile(&client, uuids).await;

    // 存储结果到缓存
    add_to_cache(uuids, profile.clone());
    if let Some(profile) = profile {
        return Ok(profile);
    } 

    Err(NoMojangPlayer::NoMojang.into())
}
#[derive(Error, Debug)]
enum NoMojangPlayer {
    #[error("非Mojang用户")]
    NoMojang
}

// 从缓存获取数据
fn get_from_cache(uuid: &Uuid) -> Option<Option<MojangProfile>> {
    let cache = PROFILE_CACHE.lock().unwrap();
    cache.get(uuid).cloned()
}

// 添加数据到缓存
fn add_to_cache(uuid: Uuid, profile: Option<MojangProfile>) {
    let mut cache = PROFILE_CACHE.lock().unwrap();
    cache.insert(uuid, profile);
}

// 获取单个用户资料
async fn fetch_single_profile(client: &Client, uuid: Uuid) -> Option<MojangProfile> {
    // 克隆需要移动的值
    let client = client.clone();
    
    // 直接使用 async move 而不是额外封装 spawn
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid.as_simple()
    );

    // 发送异步请求
    match client.get(&url).send().await {
        std::result::Result::Ok(response) if response.status().is_success() => {
            match response.json::<MojangProfile>().await {
                std::result::Result::Ok(profile) => Some(profile),
                Err(e) => {
                    log::warn!("Failed to parse profile for {}: {}", uuid, e);
                    None
                }
            }
        }
        std::result::Result::Ok(response) => {
            log::warn!("API error for {}: Status {}", uuid, response.status());
            None
        }
        Err(e) => {
            log::warn!("Request failed for {}: {}", uuid, e);
            None
        }
    }
}
// 添加缓存清除功能（可选）
pub fn clear_cache() {
    let mut cache = PROFILE_CACHE.lock().unwrap();
    cache.clear();
    log::info!("Mojang profile cache cleared");
}

// 添加缓存统计功能（可选）
pub fn cache_stats() -> (usize, usize) {
    let cache = PROFILE_CACHE.lock().unwrap();
    let total = cache.len();
    let hits = cache.values().filter(|v| v.is_some()).count();
    (total, hits)
}
