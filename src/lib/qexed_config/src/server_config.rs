
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub log_level: String,
    pub network: NetworkConfig,
    pub node: NodeConfig,
    pub game: GameConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfig {
    pub mode: String,
    #[serde(rename = "child_node")]
    pub child_node: Option<ChildNodeConfig>,
    #[serde(rename = "parent_node")]
    pub parent_node: Option<ParentNodeConfig>,
    #[serde(rename = "control")]
    pub control: Option<ControlConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildNodeConfig {
    pub name: String,
    pub secret: String,
    pub force_cross_subnet_connection: bool,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentNodeConfig {
    pub ip: String,
    pub port: u16,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlConfig {
    pub is_web: bool,
    pub is_cli: bool,
    pub web: Option<WebConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConfig {
    pub ip: String,
    pub port: u16,
    pub parent_node_ip: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub motd: Vec<String>,
    pub favicon:String,
    pub max_player: u32,
    pub tps: u16,
    pub whitelist: bool,
    pub network_compression_threshold: i32,
    pub verify_decompressed_packets: bool,
    pub chunk_render_distance: u8,
    pub plugin_sync: bool,
    pub plugin_config_sync: bool,
    #[serde(rename="online-mode")]
    pub online_mode:bool,
    pub world: WorldConfig,
    pub dimensions: Dimensions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    pub world: String,
    pub db_path: String,
    pub verify_chunk_data: bool,
    pub map_size: u64,
    pub cache_ttl: u64,
    pub cache_capacity: u64,
    pub is_show_world_seed: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    #[serde(rename = "allow-end")]
    pub allow_end: bool,
    #[serde(rename = "allow-hell")]
    pub allow_hell: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub connection_pool_enabled: bool,
    pub max_connections: u32,
    pub min_idle_connections: u32,
    pub connection_timeout: u64,
    pub mongodb: MongoDBConfig,
    pub redis: RedisConfig,
    pub mysql: MySQLConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoDBConfig {
    pub connection_string: String,
    pub default_database: String,
    pub timeout: u64,
    pub ssl_enabled: bool,
    pub read_preference: String,
    pub write_concern: String,
    pub max_pool_size: u32,
    pub min_pool_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub database: u8,
    pub timeout: u64,
    pub max_connections: u32,
    pub max_idle_connections: u32,
    pub tls_enabled: bool,
    pub key_expiration_scan_interval: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MySQLConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub charset: String,
    pub connect_timeout: u64,
    pub max_connections: u32,
    pub max_idle_connections: u32,
    pub ssl_enabled: bool,
    pub max_lifetime: u64,
    pub slow_query_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheConfig {
    pub default_ttl: u64,
    pub max_entries: u64,
    pub key_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ORMConfig {
    pub auto_migrate: bool,
    pub log_queries: bool,
    pub batch_size: u32,
    pub query_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_level: "INFO".to_string(),
            network: NetworkConfig {
                ip: "0.0.0.0".to_string(),
                port: 25565,
            },
            node: NodeConfig {
                mode: "single_server".to_string(),
                child_node: Some(ChildNodeConfig {
                    name: "Server1".to_string(),
                    secret: "".to_string(),
                    force_cross_subnet_connection: false,
                    ip: "127.0.0.1:32757".to_string(),
                }),
                parent_node: Some(ParentNodeConfig {
                    ip: "0.0.0.0".to_string(),
                    port: 32754,
                    secret: "".to_string(),
                }),
                control: Some(ControlConfig {
                    is_web: false,
                    is_cli: false,
                    web: Some(WebConfig {
                        ip: "0.0.0.0".to_string(),
                        port: 8080,
                        parent_node_ip: "127.0.0.1:32754".to_string(),
                        secret: "".to_string(),
                    }),
                }),
            },
            game: GameConfig {
                motd: vec![
                    "Welcome to the best server ever!".to_string(),
                    "Rust".to_string(),
                    "Good luck, have fun!".to_string(),
                ],
                favicon:"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAACXBIWXMAAA9hAAAPYQGoP6dpAAACtklEQVR42u2ay0rDQBSGJ2EQCipqERU3SkFQQUERRJSCuHDrQvcu3Powbn0DH6IIohQKIi26ELRF8FLxAlbsyksTmTC2yVwyk3ZizmySkjaT/zvnP5mT1PpuDJTgYaOEDwAAAAAAAAAAAAAAAAAAAAAAAABI5MAyX7YsS3lC07pvLCs+jAAd4DpqARXxia8BpsOzk5r6QgB0iTfZOnbU0TO9bthRCIhT0cRRpX4U4v2yUnUezJokrA10iReZX7XWYN0CdNUO0Wj7BUzm+rGJvpSJKn2c/M7ZikKQArC/1RqVnYPvjon3gyELwWp+N+j3QyJ8cHYNTU30uPuvz1V3W8yd/IHAmph3UToLqOi5uAAc8dnNDU/0eeW95SSf10UPQlgAQZEPC0U0kzAv5R3xtPDFhRQaHc+4+7flK1R5SqHba30W0HUHoe2gVAOIeFo4EZ8v1Bt7dTSzuuTCCqoHKqmvAoRAYM2PWdF3PH9eeQwUvzyf8WpB6cZiimve6rrdqmYMcylMCh6d8seFO088GbkiZkaB5ftOd4yYl/601x/KvylPxBN7DPUidC+Yjn4FTtQqYazBswETgCNueHygIR41xL94wi8ua2gk/eEer771ofvTI7SX/wrl7043Tszb4O6ijda3s6746bFu1J8e8rLCEX92WHL3afG8KDdHsB0AWHNw1wEOhJG5FTQ52uV+fqk9+grnpTHP68YCIBDoEZTuMguhZiBGA5CdTHYl2I5nCEHnhldj/1mcyBrDCABBdaEd/YUdx6jpPI8xAHQWQJl+w6gMoK0QNhNkmy3jLCCyihRprCJ5JqialrINjEhEVd8VYNPEs+4MUSynjXwsLmMJ7W+GTIh+OxsmOy7iY7cUjoN4aIaiAhCX6AcWQdX1eJz+TYbjfPFQAwAAAAAAAAAAAACl8QOub9TOwLTmGwAAAABJRU5ErkJggg==".to_string(),
                max_player: 100,
                tps: 20,
                whitelist: false,
                network_compression_threshold: 64,
                verify_decompressed_packets: true,
                chunk_render_distance: 12,
                plugin_sync: true,
                plugin_config_sync: false,
                online_mode:true,
                world: WorldConfig {
                    world: "我的世界".to_string(),
                    db_path: "world".to_string(),
                    verify_chunk_data: true,
                    map_size: 1_000,
                    cache_ttl: 60,
                    cache_capacity: 20_000,
                    is_show_world_seed: false,
                },
                dimensions: Dimensions {
                    allow_end: false,
                    allow_hell: false,
                },
            },
            database: DatabaseConfig {
                connection_pool_enabled: true,
                max_connections: 100,
                min_idle_connections: 5,
                connection_timeout: 3000,
                mongodb: MongoDBConfig {
                    connection_string: "mongodb://admin:password@localhost:27017".to_string(),
                    default_database: "app_db".to_string(),
                    timeout: 10,
                    ssl_enabled: false,
                    read_preference: "primary".to_string(),
                    write_concern: "majority".to_string(),
                    max_pool_size: 50,
                    min_pool_size: 5,
                },
                redis: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    password: "".to_string(),
                    database: 0,
                    timeout: 2000,
                    max_connections: 20,
                    max_idle_connections: 5,
                    tls_enabled: false,
                    key_expiration_scan_interval: 60,
                },
                mysql: MySQLConfig {
                    enabled: true,
                    host: "localhost".to_string(),
                    port: 3306,
                    database: "app_db".to_string(),
                    username: "root".to_string(),
                    password: "mysql_password".to_string(),
                    charset: "utf8mb4".to_string(),
                    connect_timeout: 5,
                    max_connections: 30,
                    max_idle_connections: 10,
                    ssl_enabled: true,
                    max_lifetime: 30,
                    slow_query_threshold: 1.0,
                }
            },
        }
    }
}

