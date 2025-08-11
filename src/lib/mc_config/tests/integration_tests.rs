use mc_config::parser::{ConfigParser, ConfigError};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_from_string() {
    let config = ConfigParser::from_str("key1=value1\n# 注释行\nkey2=value2\n\nkey3=value3");
    
    assert_eq!(config.get("key1"), Some("value1"));
    assert_eq!(config.get("key2"), Some("value2"));
    assert_eq!(config.get("key3"), Some("value3"));
    assert_eq!(config.get("nonexistent"), None);
}

#[test]
fn test_parse_from_file() -> Result<(), ConfigError> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "server-port=25565\n# 服务器端口")?;
    
    let config = ConfigParser::from_file(file.path())?;
    
    assert_eq!(config.get("server-port"), Some("25565"));
    Ok(())
}

#[test]
fn test_set_and_update() {
    let mut config = ConfigParser::from_str("key1=old_value");
    config.set("key1", "new_value");
    config.set("key2", "value2");
    
    assert_eq!(config.get("key1"), Some("new_value"));
    assert_eq!(config.get("key2"), Some("value2"));
}

#[test]
fn test_merge_configs() {
    let mut base = ConfigParser::from_str("key1=value1\nkey2=old_value");
    let update = ConfigParser::from_str("key2=new_value\nkey3=value3");
    
    base.merge(&update);
    
    assert_eq!(base.get("key1"), Some("value1"));
    assert_eq!(base.get("key2"), Some("new_value"));
    assert_eq!(base.get("key3"), Some("value3"));
}

#[test]
fn test_to_string_preserves_structure() {
    let input = "# 头部注释\nkey1=value1\n\n# 中间注释\nkey2=value2\n# 尾部注释";
    let config = ConfigParser::from_str(input);
    let output = config.to_string();
    
    assert_eq!(input, output);
}

#[test]
fn test_save_to_file() -> Result<(), ConfigError> {
    let mut config = ConfigParser::new();
    config.set("motd", "Welcome to my server!");
    config.set("max-players", "20");
    
    let file = NamedTempFile::new()?;
    let path = file.path();
    config.save_to_file(path)?;
    
    let content = std::fs::read_to_string(path)?;
    assert!(content.contains("motd=Welcome to my server!"));
    assert!(content.contains("max-players=20"));
    
    Ok(())
}

#[test]
fn test_eula_check() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "eula=true").unwrap();
    assert!(mc_config::is_eula_accepted(file.path().to_str().unwrap()));
    
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "eula=false").unwrap();
    assert!(!mc_config::is_eula_accepted(file.path().to_str().unwrap()));
    
    assert!(!mc_config::is_eula_accepted("nonexistent.txt"));
}
#[test]
fn test_inline_comments() {
    let input = "a = 123 # 我是注释\nb=456#这不是注释\nc=789";
    let config = ConfigParser::from_str(input);
    
    // 验证值
    assert_eq!(config.get("a"), Some("123")); // 修复前这里失败
    assert_eq!(config.get("b"), Some("456#这不是注释"));
    assert_eq!(config.get("c"), Some("789"));
    
    // 验证原始格式
    let output = config.to_string();
    assert!(output.contains("a=123 # 我是注释"));
    assert!(output.contains("b=456#这不是注释"));
    assert!(output.contains("c=789"));
    
    // 更新带注释的键
    let mut config = config;
    config.set("a", "456");
    let updated = config.to_string();
    assert!(updated.contains("a=456 # 我是注释"));
}

#[test]
fn test_key_value_with_spaces() {
    let config = ConfigParser::from_str(
        " key1 = value1 \nkey2= value2\n key3 =value3 \nkey4 = value with spaces "
    );
    
    assert_eq!(config.get("key1"), Some("value1"));
    assert_eq!(config.get("key2"), Some("value2"));
    assert_eq!(config.get("key3"), Some("value3"));
    assert_eq!(config.get("key4"), Some("value with spaces"));
}
