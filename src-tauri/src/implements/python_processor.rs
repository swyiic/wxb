use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tauri::command;
use uuid::Uuid;
// use std::path::PathBuf;

#[command]
pub fn generate_report(
    company_name: String,
    web_url: String,
    find_date: String,
    system_name: String,
    vuln_name: String,
    vuln_url: String,
    vuln_description: String,
    fix_suggestion: String,
    assist_image: String,
    test_process: String,
    appendix: String,
) -> Result<String, String> {
    let uuid = Uuid::new_v4().to_string();
    let temp_file_path = format!("temp_data_{}.json", uuid);
    println!("JSON 创建成功: {}", temp_file_path);

    let json_data = serde_json::json!({
        "company_name": company_name,
        "web_url": web_url,
        "find_date": find_date,
        "system_name": system_name,
        "vuln_name": vuln_name,
        "vuln_url": vuln_url,
        "vuln_description": vuln_description,
        "fix_suggestion": fix_suggestion,
        "assist_image": assist_image,
        "test_process": test_process,
        "appendix": appendix,
    });

    // 创建临时 JSON 文件
    let mut file = File::create(&temp_file_path).map_err(|e| e.to_string())?;
    file.write_all(json_data.to_string().as_bytes())
        .map_err(|e| e.to_string())?;
    // 获取可执行文件的路径
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let py_path = exe_dir.join("resources").join("generate_report.py");
    // 运行 Python 脚本
    let output = Command::new("resources\\myenv\\Scripts\\python.exe")
        .arg(py_path.to_str().unwrap())
        .arg(&temp_file_path)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e.to_string()))?;

    // 删除临时文件
    std::fs::remove_file(&temp_file_path).ok();

    // 检查 Python 脚本的执行状态
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Python 脚本执行成功: {}", stdout);
        Ok("报告生成成功".into()) // 返回给前端成功消息
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Python 脚本执行失败: {}", stderr);
        Err(format!("报告生成失败: {}", stderr)) // 返回给前端失败消息
    }
}
