// use rustpython_vm as vm;  // 导入 rustpython_vm 库并重命名为 vm
// 
// fn main() -> vm::PyResult<()> {  // 主函数返回 Python 操作结果
//     // 创建不带标准库的 Python 解释器实例
//     vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
//         // 创建包含内置函数的作用域（如 print 函数）
//         let scope = vm.new_scope_with_builtins();
//         
//         // 要执行的 Python 源代码
//         let source = r#"print("Hello World!")"#;
//         
//         // 将源代码编译为可执行的字节码
//         let code_obj = vm
//             .compile(
//                 source,                      // 源代码
//                 vm::compiler::Mode::Exec,    // 编译模式：可执行模块
//                 "<embedded>".to_owned()      // 模拟文件名（用于错误信息）
//             )
//             .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;  // 编译错误处理
//         
//         // 在创建的作用域中执行编译后的字节码
//         vm.run_code_obj(code_obj, scope)?;
//         
//         Ok(())  // 返回成功
//     })
// }
// use mc_config::parser::ConfigParser;
// 
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 从文件加载配置
//     let mut config = ConfigParser::from_file("server.properties")?;
//     
//     // 读取配置值
//     if let Some(port) = config.get("server-port") {
//         println!("服务器端口: {}", port);
//     }
//     
//     // 更新配置
//     config.set("motd", "My Minecraft Server");
//     config.set("max-players", "20");
//     
//     // 合并其他配置
//     let extra_config = ConfigParser::from_str("difficulty=hard\npvp=true");
//     config.merge(&extra_config);
//     
//     // 保存更新后的配置
//     config.save_to_file("server_updated.properties")?;
//     
//     // 检查 EULA
//     if !mc_config::is_eula_accepted("eula.txt") {
//         eprintln!("请先接受 EULA 协议");
//     }
//     
//     Ok(())
// }



use qexed_config::Config;
use tklog::{Format, ASYNC_LOG,MODE};


// 调用异步函数
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let is_true = modern_init().await;
    if !is_true{
        log::info!("程序将在3秒后关闭");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        return Ok(());
    }
    let is_init_true = qexed_config::init_global_config();
    if is_init_true.is_err(){
        log::error!("初始化读取配置文件失败")
    }
    let config  = qexed_config::get_global_config()?;
    // let mut config: Config = load_or_create_config()?;
    match config.node.mode.as_str() {
        "single_server" => {
            log::info!("单服务端模式激活");
            return qexed_single_server::main().await;
        }
        "child_node" => {
            log::info!("子节点模式激活");
        }
        "parent_node" => {
            log::info!("母节点模式激活");
        }
        "control"=>{
            log::info!("控制模式激活");
        }
        _ => {
            log::warn!("未知节点模式.目前版本仅支持如下模式参数:");
            log::warn!("single_server,child_node,parent_node,control");
            return Ok(())
        },
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(())
}

async fn modern_init()->bool{
    log_init().await;
    log::info!("初始化中,请稍后");
    let is_eula = mc_config::eula::check_eula();
    if !is_eula{
        return false;
    }
    
    true
    
}

async fn log_init() {
    //初始化
    ASYNC_LOG.set_console(true)
        // .set_level(LEVEL::Trace)
        .set_cutmode_by_time("./log/server.log", MODE::DAY, 30, true).await
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)
        .uselog(); //启用官方log库
}
