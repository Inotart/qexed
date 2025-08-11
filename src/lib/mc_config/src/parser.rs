use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// 配置解析错误类型
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("无效的键值对格式: {0}")]
    InvalidFormat(String),
}

/// 配置文件解析器
#[derive(Debug, Clone, Default)]
pub struct ConfigParser {
    /// 存储键值对配置
    values: HashMap<String, String>,
    /// 存储原始注释和空行
    pub raw_lines: Vec<String>,
}

impl ConfigParser {
    /// 创建新的空配置解析器
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            raw_lines: Vec::new(),
        }
    }

    /// 从字符串解析配置
    pub fn from_str(s: &str) -> Self {
        let mut parser = Self::new();
        parser.parse_str(s);
        parser
    }

    /// 从文件解析配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        Ok(Self::from_str(&content))
    }

    /// 解析字符串内容
    pub fn parse_str(&mut self, s: &str) {
        for line in s.lines() {
            // 保存原始行（不修改）
            let original_line = line.to_string();
            let trimmed = line.trim();
            
            // 处理注释行和空行
            if trimmed.starts_with('#') || trimmed.is_empty() {
                self.raw_lines.push(original_line);
                continue;
            }
            
            // 处理行内注释
            let (content_line, comment) = if let Some(pos) = trimmed.find('#') {
                // 检查 # 前是否有空格（确保是行内注释）
                if pos > 0 && trimmed.as_bytes()[pos - 1].is_ascii_whitespace() {
                    (
                        trimmed[..pos].trim(),
                        Some(trimmed[pos..].to_string()))
                } else {
                    (trimmed, None)
                }
            } else {
                (trimmed, None)
            };
            
            // 解析键值对
            if let Some((key, value)) = Self::parse_key_value(content_line) {
                self.values.insert(key.to_string(), value.to_string());
                
                // 重建行（保留原始格式）
                let mut new_line = format!("{}={}", key, value);
                if let Some(comment) = comment {
                    new_line.push(' ');
                    new_line.push_str(&comment);
                }
                self.raw_lines.push(new_line);
            } else {
                // 保留无法解析的行
                self.raw_lines.push(original_line);
            }
        }
    }

    /// 解析键值对（修复版本）
    fn parse_key_value(line: &str) -> Option<(&str, &str)> {
        // 找到第一个有效的分隔符位置
        let pos = line.find('=')?;
        
        // 提取键（去除前后空格）
        let key = line[..pos].trim();
        if key.is_empty() {
            return None;
        }
        
        // 提取值（去除前后空格）
        let value = line[pos + 1..].trim();
        
        Some((key, value))
    }

    /// 获取所有键值对
    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.values
    }

    /// 获取指定键的值
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// 设置键值对
    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
        
        // 更新原始行
        let new_line = format!("{}={}", key, value);
        if let Some(index) = self.find_key_line_index(key) {
            // 检查原始行是否有注释
            if let Some(comment) = self.extract_comment(&self.raw_lines[index]) {
                self.raw_lines[index] = format!("{} {}", new_line, comment);
            } else {
                self.raw_lines[index] = new_line;
            }
        } else {
            self.raw_lines.push(new_line);
        }
    }

    /// 提取行内注释
    fn extract_comment(&self, line: &str) -> Option<String> {
        if let Some(pos) = line.find('#') {
            if pos > 0 && line.as_bytes()[pos - 1].is_ascii_whitespace() {
                return Some(line[pos..].to_string());
            }
        }
        None
    }

    /// 查找键在原始行中的位置
    fn find_key_line_index(&self, key: &str) -> Option<usize> {
        self.raw_lines
            .iter()
            .position(|line| {
                if let Some((k, _)) = Self::parse_key_value(line.trim()) {
                    k == key
                } else {
                    false
                }
            })
    }

    /// 合并另一个配置解析器
    pub fn merge(&mut self, other: &Self) {
        for (key, value) in &other.values {
            self.set(key, value);
        }
    }

    /// 转换为配置字符串
    pub fn to_string(&self) -> String {
        self.raw_lines.join("\n")
    }

    /// 保存到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}