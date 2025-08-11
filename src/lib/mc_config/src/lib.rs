pub mod parser;
pub mod eula;
pub mod version;
/// 检查 EULA 是否已接受
/// 
/// # 参数
/// - `eula_path`: eula.txt 文件路径
/// 
/// # 返回值
/// - `true`: EULA 已接受
/// - `false`: EULA 未接受或文件不存在
pub fn is_eula_accepted(eula_path: &str) -> bool {
    match parser::ConfigParser::from_file(eula_path) {
        Ok(parser) => parser.get("eula")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        Err(_) => false,
    }
}
