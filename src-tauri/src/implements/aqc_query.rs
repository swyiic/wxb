// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, REFERER, COOKIE};
// use reqwest::Error;
// use serde_json::{json, Value};
// use std::fs::File;
// use std::io::{self, Write};
// use tauri::command;
// use urlencoding::encode;

// #[command]
// pub async fn query_enterprise(enterprise_name: String, cookie: String) -> Result<String, String> {
//     let client = reqwest::Client::new();
//     let query = format!("?q={}&t=0", encode(&enterprise_name));
//     let url = format!("https://aiqicha.baidu.com/s{}", query);

//     let mut headers = HeaderMap::new();
//     headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"));
//     headers.insert(REFERER, HeaderValue::from_str(&format!("https://aiqicha.baidu.com/s{}", query)).unwrap());
//     headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
//     headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));

//     let response = client
//         .get(&url)
//         .headers(headers)
//         .send()
//         .await
//         .map_err(|e| format!("请求失败: {}", e))?
//         .text()
//         .await
//         .map_err(|e| format!("获取响应失败: {}", e))?;

//     // 调用解析函数解析 JSON 响应
//     if let Err(e) = parse_json_response(&response) {
//         return Err(format!("解析 JSON 响应失败: {}", e));
//     }

//     Ok(response)
// }

// fn decode_unicode(input: &str) -> String {
//     // 在 Rust 中，这里假设 input 是一个 Unicode 转义序列
//     // 需要替换为实际的解码逻辑
//     input.to_string() // 占位符，具体逻辑视需求而定
// }

// fn parse_json_response(json_response: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let json_object: Value = serde_json::from_str(json_response)?;

//     let mut result = String::new();

//     if let Some(data) = json_object.get("data") {
//         if let Some(basic_data) = data.get("basicData") {
//             let prev_ent_name = if let Some(prev_ent_names) = basic_data.get("prevEntName") {
//                 if prev_ent_names.is_array() && !prev_ent_names.as_array().unwrap().is_empty() {
//                     decode_unicode(prev_ent_names[0].as_str().unwrap())
//                 } else {
//                     decode_unicode(prev_ent_names.as_str().unwrap())
//                 }
//             } else {
//                 "未提供".to_string()
//             };

//             let reg_addr = if let Some(addr) = basic_data.get("regAddr") {
//                 decode_unicode(addr.as_str().unwrap())
//             } else {
//                 "未提供".to_string()
//             };

//             println!("企业名称: {}", prev_ent_name);
//             println!("注册地址: {}", reg_addr);

//             result.push_str(&format!("企业名称: {}\n", prev_ent_name));
//             result.push_str(&format!("注册地址: {}\n", reg_addr));
//         } else {
//             println!("[!] 没有找到 'basicData' 字段");
//         }
//     } else {
//         println!("[!] 未找到有效数据");
//     }

//     // 将结果写入临时文件
//     let mut file = File::create("tmp")?;
//     file.write_all(result.as_bytes())?;

//     Ok(())
// }
