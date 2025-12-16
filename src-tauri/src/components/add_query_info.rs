use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs;
use tauri::command;

fn get_config_path(config_name: &str) -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    path.push("resources");
    path.push(config_name);
    path
}

#[command]
pub fn add_query_info(aqc_cookie: String) -> Result<(), String> {
    // 拼接 config.txt 文件路径
    let config_path = get_config_path("config.txt");

    // 打开或创建 config.txt 文件，并追加写入
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&config_path)
        .map_err(|e| format!("无法打开或创建文件: {}", e.to_string()))?;

    // 将 aqc_cookie 写入文件
    writeln!(file, "{}", aqc_cookie).map_err(|e| format!("写入文件失败: {}", e.to_string()))?;

    println!("已写入 Cookie 至文件: {:?}", config_path);

    Ok(())
}

#[command]
pub fn load_query_info() -> Result<String, String> {
    let config_path = get_config_path("config.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .open(&config_path)
        .map_err(|e| format!("无法打开: {}", e.to_string()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("无法读取文件内容: {}", e.to_string()))?;

    println!("读取到的 Cookie 内容: {:?}", contents);

    Ok(contents)
}
