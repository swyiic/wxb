// use std::fs::OpenOptions;
// use std::io::Write;
// use std::path::PathBuf;
// use std::fs::create_dir_all;
// use tauri::{command, State, Env, PackageInfo};
// use tauri::api::path::resource_dir;
// use crate::AppState;

// // // 修正 get_config_path 函数的返回类型
// // fn get_config_path(config_name: &str, state: &State<'_, AppState>) -> Result<PathBuf, String> {
// //     let resource_dir = resource_dir(&state.package_info, &state.env).ok_or("Failed to get resource directory")?;
// //     Ok(resource_dir.join("resources").join(config_name))
// // }

// #[command]
// pub fn add_query_info(
//     aqc_cookie: String,
//     state: State<'_, AppState>,
// ) -> Result<(), String> {
//     // 获取当前项目的根目录
//     let mut resource_dir = std::env::current_dir().map_err(|e| format!("无法获取当前目录: {}", e.to_string()))?;

//     // 拼接到 `src-tauri/resources` 目录
//     resource_dir.push("resources");

//     // 检查或创建 `resources` 目录
//     if !resource_dir.exists() {
//         create_dir_all(&resource_dir).map_err(|e| format!("无法创建资源目录: {}", e.to_string()))?;
//     }

//     // 拼接 config.txt 文件路径
//     let config_path = resource_dir.join("config.txt");

//     // 打开或创建 config.txt 文件，并追加写入
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .append(true)

//         .open(&config_path)
//         .map_err(|e| format!("无法打开或创建文件: {}", e.to_string()))?;

//     // 将 aqc_cookie 写入文件
//     writeln!(file, "{}", aqc_cookie)
//         .map_err(|e| format!("写入文件失败: {}", e.to_string()))?;

//     println!("已写入 Cookie 至文件: {:?}", config_path);

//     Ok(())
// }

// #[command]
// pub fn load_query_info(
//     state: State<'_, AppState>,
// ) -> Result<String, String> {
//     let config_path = get_config_path("config.txt", &state)?;

//     let mut file = OpenOptions::new()
//         .read(true)
//         .open(&config_path)
//         .map_err(|e| format!("无法打开: {}", e.to_string()))?;

//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .map_err(|e| format!("无法读取文件内容: {}", e.to_string()))?;

//     println!("读取到的 Cookie 内容: {:?}", contents);

//     Ok(contents)
// }
