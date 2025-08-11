use std::{fs::File, path::Path};

use anyhow::Ok;
use std::io::Write;

use crate::is_eula_accepted;
pub fn check_eula()->bool{
    let eula_path: &Path = Path::new("eula.txt");
    
    // 确保文件不存在
    if !eula_path.exists(){
        let is_true: Result<(), anyhow::Error> = summon_eula(eula_path);
        if is_true.is_err(){
            log::error!("eula.txt 生成失败,停止运行");
            return false;
        }
    }
    
    // 检查 EULA 状态
    if !is_eula_accepted(eula_path.to_str().unwrap()){
        log::warn!("您需要同意eula协议方可运行,文件为eula.txt");
        return false;
    }
    true
}

fn summon_eula(eula_path: &Path) -> anyhow::Result<()> {
    // 创建 eula.txt 文件
    let mut file = File::create(eula_path)?;
    
    writeln!(file, "# Minecraft EULA")?;
    writeln!(file, "# By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).")?;
    writeln!(file, "# {}", chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y"))?;
    writeln!(file, "eula=false")?;
    
    Ok(())
}