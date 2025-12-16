use serde_json::json;
use std::{fs, path::PathBuf, io::Read};
use dirs::home_dir;

/// 获取 auth.json 保存路径
fn get_auth_path() -> Result<PathBuf, String> {
    if tauri::is_dev() {
        // 开发模式 → src-tauri/resources
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("获取当前目录失败: {}", e))?;
        let mut path = current_dir.join("resources");
        fs::create_dir_all(&path)
            .map_err(|e| format!("创建开发模式 resources 目录失败: {}", e))?;
        path.push("auth.json");
        println!("[DEV] auth.json path = {:?}", path);
        Ok(path)
    } else {
        // 生产模式 → macOS 用户可写目录
        #[cfg(target_os = "macos")]
            {
                let home = home_dir().ok_or("无法获取 home 目录")?;
                let app_name = env!("CARGO_PKG_NAME"); // 自动获取 App 名称
                let app_dir = home.join("Library")
                    .join("Application Support")
                    .join(app_name);
                let mut path = app_dir.join("resources");
                fs::create_dir_all(&path)
                    .map_err(|e| format!("创建生产模式 resources 目录失败: {}", e))?;
                path.push("auth.json");
                println!("[PROD] auth.json path = {:?}", path);
                Ok(path)
            }

        #[cfg(not(target_os = "macos"))]
            {
                let exe_dir = std::env::current_exe()
                    .map_err(|e| format!("获取 exe 路径失败: {}", e))?
                    .parent()
                    .ok_or("exe parent 不存在")?
                    .to_path_buf();
                let mut path = exe_dir.join("resources");
                fs::create_dir_all(&path)
                    .map_err(|e| format!("创建 resources 目录失败: {}", e))?;
                path.push("auth.json");
                println!("[OTHER] auth.json path = {:?}", path);
                Ok(path)
            }
    }
}

/// 保存 headers 到 auth.json
#[tauri::command]
pub fn save_auth_from_headers(headers_text: String) -> Result<String, String> {
    println!("==== save_auth_from_headers called ====");
    println!("headers_text length = {}", headers_text.len());

    // 解析 headers
    let mut token = String::new();
    let mut user_id = String::new();
    let mut cookie = String::new();
    let mut origin = String::new();
    let mut referer = String::new();

    for (i, line) in headers_text.lines().enumerate() {
        let line = line.trim();
        println!("line {} => {:?}", i + 1, line);
        if let Some(v) = line.strip_prefix("token:") { token = v.trim().to_string(); }
        else if let Some(v) = line.strip_prefix("userId:") { user_id = v.trim().to_string(); }
        else if let Some(v) = line.strip_prefix("Cookie:") { cookie = v.trim().to_string(); }
        else if let Some(v) = line.strip_prefix("Origin:") { origin = v.trim().to_string(); }
        else if let Some(v) = line.strip_prefix("Referer:") { referer = v.trim().to_string(); }
    }
    println!("parsed -> token.len={}, userId.len={}", token.len(), user_id.len());

    if token.is_empty() || user_id.is_empty() {
        let err_msg = format!(
            "未检测到 token 或 userId (token.len={}, userId.len={})",
            token.len(), user_id.len()
        );
        println!("ERROR: {}", err_msg);
        return Err(err_msg);
    }

    let auth_json = json!({
        "userId": user_id,
        "token": token,
        "cookie": cookie,
        "origin": origin,
        "referer": referer
    });
    // 获取路径
    let path = get_auth_path()?;
    // 写入文件
    let json_text = serde_json::to_string_pretty(&auth_json)
        .map_err(|e| format!("JSON 序列化失败: {}", e))?;

    if let Err(e) = fs::write(&path, json_text.as_bytes()) {
        let em = format!("写入文件失败: {}", e);
        println!("ERROR: {}", em);
        return Err(em);
    } else {
        println!("写入文件成功");
    }
    // 验证读取
    match fs::File::open(&path) {
        Ok(mut f) => {
            let mut buf = String::new();
            if let Err(e) = f.read_to_string(&mut buf) {
                let em = format!("写入后读取文件失败: {}", e);
                println!("ERROR: {}", em);
                return Err(em);
            } else {
                println!("read back file length = {}", buf.len());
            }
        }
        Err(e) => {
            let em = format!("写入后打开文件失败: {}", e);
            println!("ERROR: {}", em);
            return Err(em);
        }
    }
    println!("==== save_auth_from_headers finished OK ====");
    Ok(format!("认证信息已成功保存到 {:?}", path))
}