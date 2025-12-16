use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;
use tauri::{command, AppHandle, Wry};
use uuid::Uuid;
use serde_json::json;
use tauri::Manager;

/// 获取可执行文件路径
fn get_executable_path(app_handle: &AppHandle<Wry>) -> PathBuf {
    #[cfg(debug_assertions)]
        {
            let mut path = std::env::current_dir().expect("无法获取当前目录");
            path.push("resources");
            path.push("generate_report");
            path
        }

    #[cfg(not(debug_assertions))]
        {
            // 生产模式直接放在 resources 下
            let exe_path = app_handle
                .path()
                .resource_dir()
                .expect("无法获取资源目录")
                .join("generate_report"); // 已打包的可执行文件
            exe_path
        }
}

#[command]
pub fn generate_report(
    app_handle: AppHandle<Wry>,
    company_name: String,
    web_url: String,
    find_date: String,
    system_name: String,
    vuln_name: String,
    vuln_url: String,
    vuln_description: String,
    fix_suggestion: String,
    affects_type: String,
    assist_image: String,
    test_process: String,
    appendix: String,
) -> Result<String, String> {
    // 临时目录存放 JSON 文件
    let temp_dir = app_handle.path().temp_dir().expect("无法获取临时目录");
    fs::create_dir_all(&temp_dir).ok();

    let uuid = Uuid::new_v4().to_string();
    let temp_json_path = temp_dir.join(format!("temp_data_{}.json", uuid));

    let json_data = json!({
        "company_name": company_name,
        "web_url": web_url,
        "find_date": find_date,
        "system_name": system_name,
        "vuln_name": vuln_name,
        "vuln_url": vuln_url,
        "vuln_description": vuln_description,
        "fix_suggestion": fix_suggestion,
        "affects_type": affects_type,
        "assist_image": assist_image,
        "test_process": test_process,
        "appendix": appendix,
    });

    File::create(&temp_json_path)
        .map_err(|e| e.to_string())?
        .write_all(json_data.to_string().as_bytes())
        .map_err(|e| e.to_string())?;

    // 检查可执行文件
    let exe_path = get_executable_path(&app_handle);
    if !exe_path.exists() {
        return Err(format!("生成报告可执行文件不存在: {:?}", exe_path));
    }

    println!("可执行文件路径: {:?}", exe_path);
    println!("JSON 文件路径: {:?}", temp_json_path);

    // 调用 Python 可执行文件
    let output = Command::new(exe_path.to_str().unwrap())
        .arg(temp_json_path.to_str().unwrap()) // 传 JSON 路径
        .output()
        .map_err(|e| format!("执行生成报告失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("生成报告 stdout: {}", stdout);
    println!("生成报告 stderr: {}", stderr);

    // 删除临时 JSON 文件
    fs::remove_file(&temp_json_path).ok();

    if output.status.success() {
        let report_path = stdout.trim();
        if report_path.is_empty() {
            Err("Python 可执行文件未返回报告路径，请检查内部 print".into())
        } else {
            println!("报告生成成功，路径: {}", report_path);
            Ok(report_path.to_string())
        }
    } else {
        Err(format!("报告生成失败: {}", stderr))
    }
}