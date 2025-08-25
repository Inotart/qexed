use anyhow::{Ok, Result};
use log::info;
use mongodb::Collection;
use qexed_core::utils::alloci32::ALLOC;
use qexed_net::net_types::packet::Packet;
use qexed_net::net_types::var_int::VarInt;
use qexed_net::packet::packet_pool::{GameEvent, KeepAliveServerPlay, LevelChunkWithLight, LoginCompression, PlayerPosition};
use qexed_net::{
    mojang_online::query_mojang_for_usernames, net_types::packet::PacketState,
    packet::packet_pool::DisconnectLogin, player::Player, read_packet,
};
use rand::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{self, net::TcpListener};
use uuid::Uuid;
use chrono::{Utc,DateTime};

pub async fn main() -> Result<(), anyhow::Error> {
    log::info!(
        "启动我的世界服务器版本:{}",
        mc_config::version::version_text()
    );
    let config = qexed_config::get_global_config()?;
    let tcplistener = qexed_net::new_tcp_server(&config.network.ip, config.network.port).await?;
    start_task(tcplistener).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}

pub async fn start_task(tcplistener: TcpListener) -> Result<(), anyhow::Error> {
    let mut players: i64 = 0;
    let config = Arc::new(Mutex::new(qexed_config::get_global_config()?));
    // 维护实体id信息
    let alloc_entity_id: Arc<Mutex<qexed_core::utils::alloci32::Alloci32>> =
        Arc::new(Mutex::new(qexed_core::utils::alloci32::Alloci32::new()));
    // 连接monggodb
    // 创建连接池
    let pool = Arc::new(Mutex::new(
        qexed_core::utils::mongo_dbconnection_pool::MongoDBConnectionPool::new().await?,
    ));
    qexed_core::registry::get_registry_data_packets();
    if let Some(p2) = qexed_core::update_tags::get_update_tags_packet()
        .as_any()
        .downcast_ref::<qexed_net::packet::packet_pool::UpdateTags>()
    {
        for p3 in &p2.tags {
            // log::info!("{:?}", p3);
        }
    }

    // 打印连接池状态（MongoDB Rust driver does not expose pool status directly）
    log::info!("创建mongodb连接成功");
    pool.lock().await.health_check().await?;

    // 获取默认数据库
    let player_map_raw  = Arc::new(Mutex::new(HashMap::new()));
    while let std::result::Result::Ok((socket, socketaddr)) = tcplistener.accept().await {
        let player_map = Arc::clone(&player_map_raw);
        let alloc_entity_id = Arc::clone(&alloc_entity_id);
        let config = Arc::clone(&config);
        let pool = Arc::clone(&pool);
        
        tokio::spawn(async move {
            let mongo_pool = pool.lock().await;
            let packet_socket_raw: Arc<Mutex<qexed_net::PacketListener>> = Arc::new(Mutex::new(qexed_net::PacketListener::new(
                socket, socketaddr,
            )));
            // log::info!("客户端 {}:{} 创建连接", socketaddr.ip(), socketaddr.port());
            let mut client_status = PacketState::Handshake;
            let mut player_conn_raw = Arc::new(Mutex::new(qexed_core::biology::player::Player::new()));
            let db = mongo_pool.default_db();
            let now: DateTime<Utc> = Utc::now();
            let mut alive_id: Arc<Mutex<u64>> = Arc::new(Mutex::new(now.timestamp_millis() as u64));
            let mut is_login_finish = false;
            let mut teleport_id_i32: i32 = 0;
            let mut first_tp: bool = false;
            let mut first_move: bool = false;
            loop {
                let mut packet_socket = packet_socket_raw.lock().await;
                let raw_packets = packet_socket.read().await;
                let mut player_conn = player_conn_raw.lock().await;
                if raw_packets.is_err() {
                    if player_conn.uuid != Uuid::nil() {
                        player_conn.is_online = false;
                        player_conn.save(&db).await.unwrap();
                    }
                    player_conn.is_online = false;
                    player_conn.save(&db).await.unwrap();
                    player_map.lock().await.remove(&player_conn.uuid);
                    if is_login_finish{
                        log::info!("玩家 {}[{}] 退出了游戏",player_conn.username,player_conn.uuid);
                    }
                                        
                    // log::info!("客户端 {}:{} 断开连接", socketaddr.ip(), socketaddr.port());
                    return;
                }
                let packets = raw_packets.unwrap();
                // log::info!("数据包内容:{:?}", packets.clone());
                let packet2 = read_packet(packets.clone(), client_status);
                if packet2.is_err() {
                    log::error!(
                        "客户端 {}:{} 数据包解析错误",
                        socketaddr.ip(),
                        socketaddr.port()
                    );
                    return;
                }
                let packet3 = packet2.unwrap();
                //log::info!("数据包:{:?}", packet3);

                
                if !is_login_finish {
                    match client_status {
                        PacketState::Handshake => match packet3.id() {
                            0x00 => {
                                if let Some(handshake) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::Handshake>(
                                ) {
                                    match handshake.next_state.0 {
                                        1 => client_status = PacketState::Status,
                                        2 => client_status = PacketState::Login,
                                        3 => {
                                            // qexed
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        },
                        PacketState::Status => match packet3.id() {
                            0x00 => {
                                if let Some(_) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::StatusRequest>(
                                ) {
                                    let mut pk =
                                        qexed_net::packet::packet_pool::StatusResponse::new();
                                    let json_response = qexed_core::utils::build_server_status::build_server_status(players);
                                    if json_response.is_err() {
                                        log::error!(
                                            "客户端 {}:{} 数据包解析错误",
                                            socketaddr.ip(),
                                            socketaddr.port()
                                        );
                                        return;
                                    }
                                    pk.json_response = json_response.unwrap();
                                    let _ = packet_socket.send(&pk).await;
                                }
                            }
                            0x01 => {
                                if let Some(pingpk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::PingRequest>(
                                ) {
                                    let mut pk =
                                        qexed_net::packet::packet_pool::PingResponse::new();
                                    pk.payload = pingpk.payload;
                                    let _ = packet_socket.send(&pk).await;
                                }
                            }

                            _ => {}
                        },
                        PacketState::Login => match packet3.id() {
                            0x00 => {
                                if let Some(loginpk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::LoginStart>(
                                ) {
                                    let config = qexed_config::get_global_config().unwrap();
                                    if !config.game.online_mode {
                                        // 压缩设置数据包
                                        if config.game.network_compression_threshold>=0{
                                            let mut pk = LoginCompression::new();
                                            pk.threshold = qexed_net::net_types::var_int::VarInt(config.game.network_compression_threshold);
                                            let _ = packet_socket.send(&pk).await;
                                            // 配置压缩
                                            packet_socket.set_compression(true,config.game.network_compression_threshold.try_into().unwrap());
                                        }

                                        // 登录成功数据包
                                        let mut pk =
                                            qexed_net::packet::packet_pool::LoginSuccess::new();
                                        pk.name = loginpk.player_name.clone();
                                        pk.uuids = loginpk.player_uuid.clone();
                                        let mut player = Player::new();
                                        player.name = loginpk.player_name.clone();
                                        player.uuids = loginpk.player_uuid.clone();
                                        packet_socket.player = Some(player);
                                        player_conn.username = loginpk.player_name.clone();
                                        player_conn.uuid = loginpk.player_uuid.clone();
                                        let _ = packet_socket.send(&pk).await;
                                        continue;
                                    };
                                    let is_check_true =
                                        is_online(&loginpk.player_name, loginpk.player_uuid).await;
                                    if is_check_true.is_err() {
                                        let mut pk = DisconnectLogin::new();
                                        pk.reason = json!({"text": "请使用正版《我的世界》账户登录","color": "red","bold": true});
                                        let _ = packet_socket.send(&pk).await;
                                        log::info!("断开连接");
                                        let _ = packet_socket.shutdown().await;
                                        return;
                                    }
                                    if is_check_true.unwrap() == false {
                                        let mut pk = DisconnectLogin::new();
                                        pk.reason = json!({"text": "请使用正版《我的世界》账户登录","color": "red","bold": true});
                                        let _ = packet_socket.send(&pk).await;
                                        log::info!("断开连接");
                                        let _ = packet_socket.shutdown().await;
                                        return;
                                    }
                                }
                            }
                            // 0x02
                            0x03 => {
                                client_status = PacketState::Configuration;
                            }
                            _ => {}
                        },
                        PacketState::Configuration => match packet3.id() {
                            0x00 => {
                                if let Some(pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::ClientInformationCtoS>(
                            ) {
                                // 暂时只处理这些,因为其他的暂时没用上
                                if let Some(player) = &mut packet_socket.player{
                                    player.locale = pk.locale.clone();
                                    player.view_distance = pk.view_distance;
                                    // 发 SelectKnownPacks 数据包
                                    let mut select_known_packs = qexed_net::packet::packet_pool::SelectKnownPacks::new();
                                    select_known_packs.known_packs =vec![
                                        qexed_net::packet::packet_pool::KnownPacks {
                                            namespace: "minecraft".to_string(),
                                            id: "core".to_string(),
                                            version: "1.21.8".to_string(),
                                        }
                                    ];
                                    let _ = packet_socket.send(&select_known_packs).await;
                                }
                            }
                            }
                            0x02 => {
                                if let Some(pk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::PluginMessage>(
                                ) {
                                    use rust_event::GLOBAL_EVENT_BUS;
                                    let bus = GLOBAL_EVENT_BUS.clone();
                                    drop(packet_socket);
                                    bus.emit::<qexed_core::event::plugin_message::RawPluginMessageEvent>((Arc::clone(&packet_socket_raw),pk.channel.clone(),pk.data.clone())).await;
                                }
                            }
                            0x03 => {
                                if let Some(_pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::FinishConfigurationCtoS>(
                            ) {
                                // 切换到play状态
                                client_status = PacketState::Play;
                            }
                            }
                            0x07 => {
                                if let Some(_pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::SelectKnownPacksCtoS>(
                            ) {
                                // 处理已知包
                                // 对抗注册
                                let pks = qexed_core::registry::get_registry_data_packets();
                                for p in pks {
                                    if let Some(p2) = p.as_any().downcast_ref::<qexed_net::packet::packet_pool::RegistryData>() {
                                        let _ = packet_socket.send(p2).await;
                                    }
                                }
                                // 发送 UpdateTags 数据包
                                if let Some(p2) = qexed_core::update_tags::get_update_tags_packet().as_any().downcast_ref::<qexed_net::packet::packet_pool::UpdateTags>() {
                                    let _ = packet_socket.send(p2).await;
                                }
                                let entity_result = alloc_entity_id.lock().await.get();
                                if entity_result.is_err() {
                                    log::error!("获取实体ID失败");
                                    // 发送disconnected 数据包,这里暂时没写
                                    return;
                                }
                                let username = player_conn.username.clone();
                                let entity_id = entity_result.unwrap();
                                *player_conn = match qexed_core::biology::player::Player::load_or_create(&db, player_conn.uuid).await {
                                    Result::Ok(player) => player,
                                    Err(e) => {
                                        log::error!("加载或创建玩家失败: {:?}", e);
                                        // 发送disconnected 数据包,这里暂时没写
                                        return;
                                    }
                                };
                                player_conn.entity_id = entity_id;
                                player_conn.username = username;
                                player_conn.is_online = true;

                                // 发送 FinishConfigurationStoC 数据包
                                let finish_configuration = qexed_net::packet::packet_pool::FinishConfigurationStoC::new();
                                let _ = packet_socket.send(&finish_configuration).await;
                                // 发送 LoginPlay 数据包
                                let mut login_play = qexed_net::packet::packet_pool::LoginPlay::new();
                                login_play.entity_id = entity_id;
                                login_play.is_hardcore = false;// 硬核模式对我们暂时没啥用
                                login_play.dimension_names = vec!["minecraft:overworld".to_string(),"minecraft:the_end".to_string(),"minecraft:the_nether".to_string()];
                                login_play.max_player = qexed_net::net_types::var_int::VarInt(config.lock().await.game.max_player as i32);
                                login_play.view_distance = qexed_net::net_types::var_int::VarInt(config.lock().await.game.chunk_render_distance as i32);
                                login_play.simulation_distance = qexed_net::net_types::var_int::VarInt(config.lock().await.game.chunk_render_distance as i32);
                                login_play.reduced_debug_info = false; // 减少调试信息
                                login_play.enable_respawn_screen = false; // 启用重生界面
                                // TODO: 这里需要后续代码实现,这里暂时快速开发不做设置兼容
                                login_play.do_limited_crafting = false; // 限制制作，Doc都说没使用那就不管
                                login_play.dimension_type = VarInt(0); // 维度类型,暂时不管
                                login_play.dimension_name = "minecraft:overworld".to_string();
                                // 不显示哈希
                                login_play.hashed_seed = 0;
                                // 游戏模式,暂时只写生存,后面支持
                                login_play.game_mode = 0; // 0:生存,1:创造,2:冒险,3:旁观
                                login_play.previous_game_mode = -1; // 上一个游戏模式,暂时不管
                                login_play.is_debug = false; // 是否调试模式
                                login_play.is_flat = false; // 是否平坦世界
                                login_play.has_death_location = false; // 是否有死亡位置
                                login_play.portal_cooldown = qexed_net::net_types::var_int::VarInt(0); // 传送门冷却时间
                                login_play.sea_level = qexed_net::net_types::var_int::VarInt(63); // 海平面高度
                                login_play.enforces_secure_chat = false; // 强制安全聊天
                                let _ = packet_socket.send(&login_play).await;
                                // Player::load_by_uuid(
                                // let collection: Collection<Player> = db.collection("players");
                                // let p = mongo_pool.lock().await;
                                // 保存数据库
                                player_conn.save(&db).await.unwrap();
                                // 传送玩家到指定位置
                                let mut pp = PlayerPosition::new();
                                teleport_id_i32 = 1;
                                pp.teleport_id = qexed_net::net_types::var_int::VarInt(teleport_id_i32);
                                player_map.lock().await.insert(player_conn.uuid, Arc::clone(&player_conn_raw));
                                let _ = packet_socket.send(&pp).await;

                            }
                            }
                            _ => {
                                log::info!("未知的数据包与内容:{:?}", packets.clone());
                            }
                        },
                        PacketState::Play => match packet3.id() {
                            0x00 => {
                                // 玩家在接受服务端后的tp逻辑,我们这里显然是登录逻辑的继续。

                                if let Some(pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::AcceptTeleportation>(
                                ) {
                                    if !first_tp{
                                        first_tp = true

                                    }
                                    // 暂时没想好如果失败的逻辑
                                    // if pk.teleport_id == qexed_net::net_types::var_int::VarInt(teleport_id_i32){
                                    //     
                                    // }
                                }
                            }
                            0x0c => {
                                if let Some(_pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::ChunkBatchStart>(
                                ) {
                                    // 处理 ChunkBatchStart 数据包
                                    // log::info!("ChunkBatchStart 数据包: {:?}", pk);
                                }
                            }
                            0x1E => {
                                if let Some(_pk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::MovePlayerPosRot>(
                                ) {
                                    // 处理 MovePlayerPosRot 数据包
                                    if !first_move{
                                        first_move=true;
                                        // 构建 GameEventPacket 数据包
                                        let mut ge = GameEvent::new();
                                        ge.event = 13;
                                        let _ = packet_socket.send(&ge).await;
                                        // 构建 SetChunkCacheCenter 数据包
                                        let sccc = qexed_net::packet::packet_pool::SetChunkCacheCenter::new();
                                        let _ = packet_socket.send(&sccc).await;
                                        // 发送玩家附近区块
                                        let radius= config.lock().await.game.chunk_render_distance as i32;
                                        for x in -radius..=radius {
                                            for z in -radius..=radius {
                                                // 创建空区块
                                                let p_q = LevelChunkWithLight {
                                                    chunk_x: x,
                                                    chunk_z: z,
                                                    data: qexed_net::net_types::chunk::Chunk {
                                                        // 高度图 - 使用修复后的高度图
                                                        heightmaps: create_heightmaps(),
                                                        // 空的区块数据 - 使用修复后的编码函数
                                                        data: encode_empty_chunk_data_1_21(),
                                                        // 无方块实体
                                                        block_entities: vec![],
                                                    },
                                                    light: qexed_net::net_types::light::Light {
                                                        // 空的光照掩码
                                                        sky_light_mask: qexed_net::net_types::bitset::Bitset(vec![]),
                                                        block_light_mask: qexed_net::net_types::bitset::Bitset(vec![]),
                                                        empty_sky_light_mask: qexed_net::net_types::bitset::Bitset(vec![]),
                                                        empty_block_light_mask: qexed_net::net_types::bitset::Bitset(vec![]),
                                                        // 空的照明数据
                                                        sky_light_arrays: vec![],
                                                        block_light_arrays: vec![],
                                                    },
                                                };
                                            
                                                // 发送数据包
                                                let _ = packet_socket.send(&p_q).await;
                                            }
                                        }
                                        is_login_finish = true;
                                        log::info!("玩家 {}[{}] 加入了游戏",player_conn.username,player_conn.uuid);
                                        player_conn.conn = Some(Arc::clone(&packet_socket_raw));
                                        tokio::spawn(qexed_core::event::join_server::alive_fn(Arc::clone(&alive_id),Arc::clone(&packet_socket_raw)));
                                        

                                    }
                                }
                            }
                            _ => {
                                log::info!("未知的数据包与内容:{:?}", packets.clone());
                            }
                        },
                    }
                } else {
                    // 暂时还没处理完成登录,完全登录后再实现这里的逻辑,这一行开始
                    // 为了巫妖王,为了艾泽拉斯！！！
                    match client_status {
                        PacketState::Play => match packet3.id() {
                            0x00 => {
                                // 玩家在接受服务端后的tp逻辑

                                if let Some(_pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::AcceptTeleportation>(
                                ) {
                                }
                            }
                            0x08 =>{
                                if let Some(pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::ChatMessageCtS>(
                                ) {
                                    // 处理 玩家消息 数据包
                                    log::info!("ChatMessageCtS 数据包: {:?}", pk);
                                }
                            }
                            0x0c => {
                                if let Some(_pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::ChunkBatchStart>(
                                ) {
                                    // 处理 ChunkBatchStart 数据包
                                    // log::info!("ChunkBatchStart 数据包: {:?}", pk);
                                }
                            }
                            0x1B => {
                                if let Some(_pk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::KeepAliveServerPlay>(
                                ) {
                                    // 处理 KeepAlive 数据包
                                }
                            }
                            0x1D => {
                                if let Some(pk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::MovePlayerPos>(
                                ) {
                                    // 处理 MovePlayerPos 数据包
                                    // log::info!("移动数据包(不含视角):{:?}",pk);
                                }
                            }
                            0x1E => {
                                if let Some(pk) = packet3
                                    .as_any()
                                    .downcast_ref::<qexed_net::packet::packet_pool::MovePlayerPosRot>(
                                ) {
                                    // 处理 MovePlayerPosRot 数据包
                                    // log::info!("移动数据包:{:?}",pk);
                                }
                            }

                            _ => {
                                log::info!("未知的数据包与内容2:{:?}", packets.clone());
                            }
                        },
                        _ =>{}
                    }
                }
            }
        });
    }
    Ok(())
}

// 下阶段登录检查(是否需要正版验证)
async fn is_online(player: &String, uuids: uuid::Uuid) -> Result<bool, anyhow::Error> {
    let config = qexed_config::get_global_config()?;
    if !config.game.online_mode {
        return Ok(true);
    };
    // 检查player是否使用uuid v3生成的,是的话绝对是盗版用户
    if let Some(version) = uuids.get_version() {
        if version != uuid::Version::Random {
            return Ok(false);
        }
    }

    // 2. 检查是否是离线模式特征 (前8字节全零)
    let bytes = uuids.as_bytes();
    if bytes[0..8] == [0; 8] {
        return Ok(false);
    }
    let user_data = query_mojang_for_usernames(uuids).await;
    if user_data.is_err() {
        return Ok(false);
    }
    if *player != user_data.unwrap().name {
        return Ok(false);
    }
    return Ok(true);
}
// 构建符合协议的状态响应

// 为 Minecraft 1.21.8 编码空区块数据
fn encode_empty_chunk_data_1_21() -> Vec<u8> {
    let mut data = Vec::new();

    // 1.21.8 使用 24 个区块段落 (从 y=-64 到 y=319)
    for _ in 0..24 {
        // 段落非空气方块数量为 0
        data.extend_from_slice(&0i16.to_be_bytes());

        // 方块状态
        // 使用调色板模式，只有一个空气方块
        let bits_per_block = 1; // 只需要 1 位，因为只有一种方块
        data.push(bits_per_block as u8);

        // 调色板长度 - 使用 VarInt 编码
        data.extend(encode_var_int(1));

        // 空气方块的 ID
        data.extend(encode_var_int(0));

        // 计算需要多少个 long 来存储 4096 个方块
        let blocks_per_long = 64 / bits_per_block;
        let num_longs = (4096 + blocks_per_long - 1) / blocks_per_long;

        // 所有方块都是空气 (调色板索引 0)
        for _ in 0..num_longs {
            data.extend_from_slice(&0i64.to_be_bytes());
        }

        // 生物群系数据
        // 使用调色板模式，只有一个生物群系
        let bits_per_biome = 1; // 只需要 1 位，因为只有一种生物群系
        data.push(bits_per_biome as u8);

        // 生物群系调色板长度 - 使用 VarInt 编码
        data.extend(encode_var_int(1));

        // 平原生物群系的 ID
        data.extend(encode_var_int(1));

        // 计算需要多少个 long 来存储 64 个生物群系 (4x4x4)
        let biomes_per_long = 64 / bits_per_biome;
        let num_biome_longs = (64 + biomes_per_long - 1) / biomes_per_long;

        // 所有生物群系都是平原 (调色板索引 0)
        for _ in 0..num_biome_longs {
            data.extend_from_slice(&0i64.to_be_bytes());
        }
    }
    data
}

fn encode_var_int(value: i32) -> Vec<u8> {
    let mut value = value as u32;
    let mut buf = Vec::new();
    loop {
        if value & !0x7F == 0 {
            buf.push(value as u8);
            break;
        } else {
            buf.push((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }
    }
    buf
}
fn create_heightmaps() -> Vec<qexed_net::net_types::heightmap::Heightmaps> {
    vec![
        qexed_net::net_types::heightmap::Heightmaps {
            type_id: VarInt(0), // MOTION_BLOCKING
            // 高度图应该包含 256 个值（16x16），每个值是一个 VarLong
            // 对于空区块，所有高度都是世界底部（-64）
            data: vec![0; 36], // 这个大小可能需要调整
        },
        qexed_net::net_types::heightmap::Heightmaps {
            type_id: VarInt(1), // WORLD_SURFACE
            data: vec![0; 36],  // 这个大小可能需要调整
        },
    ]
}
