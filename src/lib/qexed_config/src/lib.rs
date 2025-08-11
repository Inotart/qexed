use std::{path::Path, fs, sync::RwLock};
use once_cell::sync::OnceCell;
use anyhow::{Context, Result};

mod server_config;
pub use server_config::Config;

// 在编译时将资源文件嵌入到可执行文件中
const DEFAULT_CONFIG: &str = include_str!("../../../../assets/data/configs/main-config.toml");
const CONFIG_PATH: &str = "config.toml";

// 使用 RwLock 替代 OnceCell 以支持配置更新
static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();

/// 加载配置，如果不存在则使用内嵌的默认配置创建
pub fn load_or_create_config(config_path: &str) -> Result<Config> {
    let path = Path::new(config_path);
    
    // 如果配置文件不存在，则使用内嵌的默认配置创建
    if !path.exists() {
        log::info!("Config file not found, creating new configuration");
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
            }
        }
        
        // 直接使用编译时嵌入的默认配置内容
        fs::write(path, DEFAULT_CONFIG)
            .with_context(|| format!("Failed to write new config at {}", path.display()))?;
    }
    
    // 读取配置文件内容
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file at {}", path.display()))?;
    
    // 解析为 Config 结构体
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file at {}", path.display()))?;
    
    Ok(config)
}

/// 保存配置到文件
pub fn save_config(config: &Config, path: &str) -> Result<()> {
    let toml_string = toml::to_string_pretty(config)
        .context("Failed to serialize config to TOML")?;
    
    fs::write(path, toml_string)
        .with_context(|| format!("Failed to write config to {}", path))?;
    
    log::info!("Config saved successfully to {}", path);
    Ok(())
}

/// 初始化并获取全局配置
pub fn init_global_config() -> Result<()> {
    let config = load_or_create_config(CONFIG_PATH)?;
    
    CONFIG.get_or_init(|| RwLock::new(config));
    
    Ok(())
}

/// 获取全局配置的只读引用
pub fn get_global_config() -> Result<Config> {
    let config_guard = CONFIG.get()
        .context("Global config not initialized")?
        .read()
        .map_err(|_| anyhow::anyhow!("Failed to acquire read lock for config"))?;
    
    Ok(config_guard.clone())
}

/// 更新全局配置
pub fn update_global_config<F>(updater: F) -> Result<()>
where
    F: FnOnce(&mut Config)
{
    let mut config_guard = CONFIG.get()
        .context("Global config not initialized")?
        .write()
        .map_err(|_| anyhow::anyhow!("Failed to acquire write lock for config"))?;
    
    // 应用更新函数
    updater(&mut config_guard);
    
    // 保存更新后的配置到文件
    save_config(&*config_guard, CONFIG_PATH)?;
    
    Ok(())
}

/// 重新加载配置文件并更新全局配置
pub fn reload_global_config() -> Result<()> {
    let new_config = load_or_create_config(CONFIG_PATH)?;
    
    let mut config_guard = CONFIG.get()
        .context("Global config not initialized")?
        .write()
        .map_err(|_| anyhow::anyhow!("Failed to acquire write lock for config"))?;
    
    *config_guard = new_config;
    
    log::info!("Configuration reloaded successfully");
    Ok(())
}