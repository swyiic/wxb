use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use tauri::State;
use dirs::home_dir;
use reqwest::{Client, multipart::{Form, Part}};
use tokio::fs::File;
use chrono::Local;
use tokio::io::AsyncReadExt;


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

// ==================== 读取 auth.json ====================
async fn load_auth() -> Result<Value, String> {
    let path = get_auth_path()?;
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取 auth.json 失败: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("解析 auth.json 失败: {}", e))
}

// ==================== 上传文件并返回 id ====================
#[tauri::command]
pub async fn get_report_number(file_path: String) -> Result<String, String> {
    // 1. 读取 auth 信息
    let auth: Value = load_auth().await?;
    let user_id = auth["userId"].as_str().ok_or("auth.json 中缺少 userId")?.to_owned();
    let token   = auth["token"].as_str().ok_or("auth.json 中缺少 token")?.to_owned();
    let cookie  = auth.get("cookie").and_then(|v| v.as_str()).unwrap_or("");
    let origin  = auth.get("origin").and_then(|v| v.as_str()).unwrap_or("http://10.20.240.60");
    let referer = auth.get("referer").and_then(|v| v.as_str()).unwrap_or("http://10.20.240.60/intelligence/submit-intelligence");

    // 2. 读取要上传的文件
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err("文件不存在".to_owned());
    }
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown").to_owned();

    let mut file = File::open(&path).await
        .map_err(|e| format!("打开文件失败: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await
        .map_err(|e| format!("读取文件内容失败: {}", e))?;

    // 3. 构造 multipart/form-data
    let part = Part::bytes(buffer)
        .file_name(file_name.clone())
        .mime_str("application/octet-stream")
        .map_err(|e| format!("设置 mime 失败: {}", e))?;

    let form = Form::new()
        .part("file", part)
        .text("isAppend", "1");

    // 4. 发起请求
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建 reqwest client 失败: {}", e))?;

    let url = "http://10.20.240.60/icpt/file/upload";

    let response = client.post(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36")
        .header("userId", &user_id)
        .header("token", &token)
        .header("Origin", origin)
        .header("Referer", referer)
        .header("Cookie", cookie)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    // 5. 检查状态码
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("上传失败 HTTP {}: {}", status, text));
    }
    // 6. 解析返回的 JSON，取出 data.id
    let resp_json: Value = response.json().await
        .map_err(|e| format!("解析响应 JSON 失败: {}", e))?;

    if let Some(code) = resp_json["bizCode"].as_str() {
        if code != "200" {
            let msg = resp_json["message"].as_str().unwrap_or("未知错误");
            if msg.contains("登录") || msg.contains("token") || msg.contains("认证") {
                return Err("Token 无效或已过期，请重新登录！".to_string());
            }
            return Err(format!("业务失败: {}", msg));
        }
    }

    println!("上传响应: {}", serde_json::to_string_pretty(&resp_json).unwrap());

    let file_id = resp_json["data"]["id"]
        .as_str()
        .ok_or("响应中未找到 data.id")?
        .to_owned();

    Ok(file_id)
}

#[tauri::command]
pub async fn process_submit_form(form_data: serde_json::Value, pending_file_path: String) -> Result<String, String> {
    // 1. 读取 auth.json（复用你之前的逻辑）
    let auth = load_auth().await?;
    let user_id = auth["userId"].as_str().ok_or("auth.json 缺失 userId")?.to_string();
    let token = auth["token"].as_str().ok_or("auth.json 缺失 token")?.to_string();
    let cookie = auth.get("cookie").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let origin = auth.get("origin").and_then(|v| v.as_str()).unwrap_or("http://10.20.240.60");
    let referer = auth.get("referer").and_then(|v| v.as_str()).unwrap_or("http://10.20.240.60/intelligence/submit-intelligence");

    let found_time = {
        let raw = form_data["foundTime"].as_str().unwrap_or("");
        if raw.is_empty() {
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        } else if raw.len() == 10 && raw.chars().nth(4) == Some('-') && raw.chars().nth(7) == Some('-') {
            // 是 "2025-11-24" 这种格式 → 补时间
            format!("{} 00:00:00", raw)
        } else {
            // 已经是完整时间，直接用
            raw.to_string()
        }
    };

    // 2. 清理和转换字段（和你的抓包完全一致）
    let payload = json!({
        "companyName": form_data["companyName"].as_str().unwrap_or(""),
        "companyWebName": form_data["companyWebName"].as_str().unwrap_or(""),
        "province": form_data["province"].as_str().unwrap_or("31"),
        "city": form_data["city"].as_str().unwrap_or("3101"),
        "district": form_data["district"].as_str().unwrap_or(""),
        "address": form_data["address"].as_str().unwrap_or(""),
        "alarmTargetIp": form_data["alarmTargetIp"].as_str().unwrap_or(""),
        "alarmPort": form_data["alarmPort"].as_str().unwrap_or(""),
        "alarmTargetUrl": form_data["alarmTargetUrl"].as_str().unwrap_or(""),
        "alarmSort": form_data["alarmSort"].as_i64().unwrap_or(2),
        "riskType": form_data["riskType"].as_i64().unwrap_or(0),
        "informationName": form_data["informationName"].as_str().unwrap_or(""),
        "foundTime": found_time,
        "alarmDesc": form_data["alarmDesc"].as_str().unwrap_or(""),
        "affectsType": form_data["affectsType"].as_str().unwrap_or(""),
        "solution": form_data["solution"].as_str().unwrap_or(""),
        "fileIds": form_data["fileIds"].as_array().unwrap_or(&vec![]),
        "taskId": form_data["taskId"].as_str().unwrap_or(""),
        "taskListId": form_data["taskListId"].as_str().unwrap_or("")
    });

    println!("提交 payload: {}", serde_json::to_string_pretty(&payload).unwrap());

    // 3. 发送请求
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let response = client
        .post("http://10.20.240.60/icpt/informationsubmit/submit")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .header("userId", user_id)
        .header("token", token)
        .header("Origin", origin)
        .header("Referer", referer)
        .header("Cookie", cookie)
        .header("Content-Type", "application/json;charset=UTF-8")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    // 4. 关键：401 拦截 + 业务码判断
    if response.status() == 401 {
        return Err("Token 无效或已过期，请重新登录！".to_string());
    }

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("提交失败 HTTP {}: {}", status, text));
    }

    let resp_json: Value = response.json().await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    println!("提交响应: {}", serde_json::to_string_pretty(&resp_json).unwrap());

    // 判断业务是否成功
    if resp_json["bizCode"].as_str() != Some("200") {
        let msg = resp_json["message"].as_str().unwrap_or("未知错误");
        if msg.contains("登录") || msg.contains("token") {
            return Err("Token 无效或已过期，请重新登录！".to_string());
        }
        return Err(format!("提交失败: {}", msg));
    }

    if let Err(e) = tokio::fs::remove_file(&pending_file_path).await {
        println!("删除 pending 文件失败（非致命）: {}", e);
        // 不返回 Err，上传成功了就不影响用户体验
    }

    Ok(json!({
        "success": true,
        "message": "情报提交成功",
        "pendingFile": pending_file_path
    }).to_string())

}