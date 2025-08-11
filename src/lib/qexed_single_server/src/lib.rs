use anyhow::{Ok, Result};
use qexed_net::{
    mojang_online::query_mojang_for_usernames, net_types::packet::PacketState,
    packet::packet_pool::DisconnectLogin, player::Player, read_packet,
};
use rand::prelude::*;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{self, net::TcpListener};
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
    while let std::result::Result::Ok((socket, socketaddr)) = tcplistener.accept().await {
        tokio::spawn(async move {
            let packet_socket_raw = Arc::new(Mutex::new(qexed_net::PacketListener::new(
                socket, socketaddr,
            )));
            log::info!("客户端 {}:{} 创建连接", socketaddr.ip(), socketaddr.port());
            let mut client_status = PacketState::Handshake;
            loop {
                let mut packet_socket = packet_socket_raw.lock().await;
                let raw_packets = packet_socket.read().await;
                if raw_packets.is_err() {
                    log::info!("客户端 {}:{} 断开连接", socketaddr.ip(), socketaddr.port());
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
                log::info!("数据包:{:?}", packet3);

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
                                let mut pk = qexed_net::packet::packet_pool::StatusResponse::new();
                                let json_response = build_server_status(players);
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
                                let mut pk = qexed_net::packet::packet_pool::PingResponse::new();
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
                                    let mut pk =
                                        qexed_net::packet::packet_pool::LoginSuccess::new();
                                    pk.name = loginpk.player_name.clone();
                                    pk.uuids = loginpk.player_uuid.clone();
                                    let mut player = Player::new();
                                    player.name = loginpk.player_name.clone();
                                    player.uuids = loginpk.player_uuid.clone();
                                    packet_socket.player = Some(player);

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
                        0x07 => {
                            if let Some(pk) = packet3
                                .as_any()
                                .downcast_ref::<qexed_net::packet::packet_pool::SelectKnownPacksCtoS>(
                            ) {
                                // 处理已知包
                                log::info!("已知包: {:?}", pk.known_packs);
                            }
                        }
                        _ => {
                            log::info!("未知的数据包与内容:{:?}", packets.clone());
                        }
                    },
                    _ => {}
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
fn build_server_status(players: i64) -> Result<serde_json::Value, anyhow::Error> {
    let config = qexed_config::get_global_config()?;
    let mut rand_item = &config.game.motd[0];
    if let Some(random_item) = config.game.motd.choose(&mut rand::rng()) {
        rand_item = random_item;
    }
    let v = json!({
        "version": {
            "name": "1.21.8",
            "protocol": 772
        },
        "players": {
            "max": config.game.max_player,
            "online": players,
            "sample": [
            ]
        },
        "description": {
            "text": rand_item,
        },
        "favicon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAACXBIWXMAAA9hAAAPYQGoP6dpAAACtklEQVR42u2ay0rDQBSGJ2EQCipqERU3SkFQQUERRJSCuHDrQvcu3Powbn0DH6IIohQKIi26ELRF8FLxAlbsyksTmTC2yVwyk3ZizmySkjaT/zvnP5mT1PpuDJTgYaOEDwAAAAAAAAAAAAAAAAAAAAAAAABI5MAyX7YsS3lC07pvLCs+jAAd4DpqARXxia8BpsOzk5r6QgB0iTfZOnbU0TO9bthRCIhT0cRRpX4U4v2yUnUezJokrA10iReZX7XWYN0CdNUO0Wj7BUzm+rGJvpSJKn2c/M7ZikKQArC/1RqVnYPvjon3gyELwWp+N+j3QyJ8cHYNTU30uPuvz1V3W8yd/IHAmph3UToLqOi5uAAc8dnNDU/0eeW95SSf10UPQlgAQZEPC0U0kzAv5R3xtPDFhRQaHc+4+7flK1R5SqHba30W0HUHoe2gVAOIeFo4EZ8v1Bt7dTSzuuTCCqoHKqmvAoRAYM2PWdF3PH9eeQwUvzyf8WpB6cZiimve6rrdqmYMcylMCh6d8seFO088GbkiZkaB5ftOd4yYl/601x/KvylPxBN7DPUidC+Yjn4FTtQqYazBswETgCNueHygIR41xL94wi8ua2gk/eEer771ofvTI7SX/wrl7043Tszb4O6ijda3s6746bFu1J8e8rLCEX92WHL3afG8KDdHsB0AWHNw1wEOhJG5FTQ52uV+fqk9+grnpTHP68YCIBDoEZTuMguhZiBGA5CdTHYl2I5nCEHnhldj/1mcyBrDCABBdaEd/YUdx6jpPI8xAHQWQJl+w6gMoK0QNhNkmy3jLCCyihRprCJ5JqialrINjEhEVd8VYNPEs+4MUSynjXwsLmMJ7W+GTIh+OxsmOy7iY7cUjoN4aIaiAhCX6AcWQdX1eJz+TYbjfPFQAwAAAAAAAAAAAACl8QOub9TOwLTmGwAAAABJRU5ErkJggg==",
        "enforcesSecureChat": false
    });
    Ok(v)
}
