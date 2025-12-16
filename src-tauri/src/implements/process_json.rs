// src-tauri/src/implements/process_json.rs
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Wry};
use serde_json::{json, Value};
use tauri::Manager;

/// 获取 pending_reports 目录（开发/生产逻辑完全分离，避免错误复制）
fn get_pending_reports_dir(app_handle: Option<&AppHandle<Wry>>) -> PathBuf {
    #[cfg(debug_assertions)]
        {
            // 开发环境：始终读取项目 resources/pending_reports
            let mut path = std::env::current_dir().unwrap();
            path.push("resources");
            path.push("pending_reports");
            fs::create_dir_all(&path).ok();
            path
        }

    #[cfg(not(debug_assertions))]
        {
            // 生产环境：直接读取 App 内部资源目录
            let app_handle = app_handle.expect("生产环境必须传入 AppHandle");
            let mut path = app_handle
                .path()
                .resource_dir()
                .expect("无法获取资源目录")
                .join("pending_reports");

            // 可选：确保目录存在（不创建文件，只创建目录避免报错）
            fs::create_dir_all(&path).ok();

            path
        }
}

/// 读取所有 pending 报告（极简版！什么都不修，什么都不补）
#[tauri::command]
pub fn load_all_pending_reports(app_handle: &AppHandle<Wry>) -> Result<Value, String> {
    let dir = {
        #[cfg(debug_assertions)]
            {
                get_pending_reports_dir(None)
            }

        #[cfg(not(debug_assertions))]
            {
                get_pending_reports_dir(Some(app_handle))
            }
    };

    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("读取目录失败: {e}"))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("pending_") && n.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect();

    // 按修改时间排序（最新在前）
    entries.sort_by_key(|p| {
        p.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    entries.reverse();

    let mut reports = Vec::new();

    for path in entries {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("读取文件失败 {}: {}", path.display(), e);
                continue;
            }
        };

        let mut report: Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("JSON 解析失败 {}: {}", path.display(), e);
                continue;
            }
        };

        // 附加两个前端必须字段（绝对路径 + 文件名）
        if let Ok(abs_path) = path.canonicalize() {
            if let Some(abs_str) = abs_path.to_str() {
                report["__filePath"] = Value::String(abs_str.to_string());
            }
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                report["__fileName"] = Value::String(file_name.to_string());
            }
        }

        reports.push(report);
    }

    if reports.is_empty() {
        return Ok(json!({
            "list": [],
            "total": 0,
            "message": "暂无待提交报告"
        }));
    }

    Ok(json!({
        "list": reports,
        "total": reports.len(),
        "message": format!("成功加载 {} 条待提交报告", reports.len())
    }))
}

/// 读取最新的一条（方便快捷打开）
#[tauri::command]
pub fn load_latest_pending_report(app_handle: &AppHandle<Wry>) -> Result<Value, String> {
    let all = load_all_pending_reports(app_handle)?;
    all["list"]
        .as_array()
        .and_then(|arr| arr.first())
        .cloned()
        .ok_or_else(|| "没有找到任何待提交报告".to_string())
}