// use quick_xml::{Reader, Writer};
// use quick_xml::events::{Event, BytesText};
// use std::fs::File;
// use std::io::{Cursor, Read, Write};
// use std::path::{Path, PathBuf};
// use tauri::api::path::resource_dir;
// use tauri::command;
// use zip::{write::FileOptions, ZipArchive, ZipWriter};
// use tauri::State;
// use crate::AppState;
// use base64::{Engine as _, engine::general_purpose};
// use std::collections::HashSet;
// use docx_rs::*;
//
// fn get_resource_path(file_name: &str, state: &State<'_, AppState>) -> Result<PathBuf, String> {
//     let resource_dir = resource_dir(&state.package_info, &state.env)
//         .ok_or("Failed to get resource directory")?;
//     Ok(resource_dir.join("resources").join(file_name))
// }
//
// fn get_unique_file_name(base_name: &str, extension: &str) -> String {
//     let mut file_name = format!("{}-{}", base_name, "网络安全检测报告");
//     let mut counter = 1;
//     while Path::new(&format!("{}.{}", file_name, extension)).exists() {
//         file_name = format!("{}-{}-{}", counter, base_name, "网络安全检测报告");
//         counter += 1;
//     }
//     format!("{}.{}", file_name, extension)
// }
//
// fn extract_base64_data(image_tag: &str) -> Result<&str, String> {
//     if let Some(base64_start) = image_tag.find("base64,") {
//         let base64_data = &image_tag[base64_start + 7..];
//         if let Some(end_index) = base64_data.find(|c| c == '"' || c == ' ' || c == '>') {
//             Ok(&base64_data[..end_index])
//         } else {
//             Ok(base64_data)
//         }
//     } else {
//         Err("无效的 Base64 数据格式".to_string())
//     }
// }
//
// fn decode_base64(base64_data: &str) -> Result<Vec<u8>, String> {
//     let decoded_data = base64::engine::general_purpose::STANDARD
//         .decode(base64_data)
//         .map_err(|e| e.to_string())?;
//     Ok(decoded_data)
// }
//
// fn insert_image_to_word(
//     zip_writer: &mut ZipWriter<Cursor<Vec<u8>>>,
//     image_data: &[u8],
//     image_name: &str,
//     image_id_counter: usize,
// ) -> Result<String, String> {
//     let image_path = format!("word/media/{}", image_name);
//     // 先开始文件写入会话，如果成功，再写入数据
//     zip_writer.start_file(image_path, FileOptions::default())
//         .map_err(|e| e.to_string())?;
//     // 写入图像数据
//     zip_writer.write_all(image_data)
//         .map_err(|e| e.to_string())?;
//     // 创建并返回一个关联ID
//     let rel_id = format!("rId{}", image_id_counter);
//     Ok(rel_id)
// }
// fn replace_text_with_image_placeholder(
//     text: &str,
//     replacements: &[(String, String)],
//     image_id_counter: &mut usize,
//     cursor: &mut Cursor<Vec<u8>>,
//     inserted_images: &mut HashSet<String>,
// ) -> Result<String, String> {
//     let mut doc = Docx::new();
//
//     for (placeholder, value) in replacements {
//         if value.contains("<img src=\"data:image/png;base64,") {
//             if inserted_images.contains(placeholder) {
//                 println!("Skipping already processed placeholder: {}", placeholder);
//                 continue;
//             }
//
//             let parts: Vec<&str> = value.split("<img src=\"data:image/png;base64,").collect();
//
//             for part in parts {
//                 if let Some(img_start) = part.find('\"') {
//                     let base64_data = &part[..img_start];
//                     let image_data = base64::decode(base64_data).map_err(|e| e.to_string())?;
//
//                     // 创建 Pic 对象
//                     let pic = Pic::new(&image_data); // 只传递 &image_data
//
//                     // 使用 docx_rs 的 add_image 方法插入图片
//                     doc = doc.add_paragraph(
//                         Paragraph::new().add_run(Run::new().add_image(pic))
//                     );
//
//                     if img_start + 1 < part.len() {
//                         let remaining_text = &part[img_start + 1..];
//                         if !remaining_text.is_empty() {
//                             doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(remaining_text)));
//                         }
//                     }
//                 } else {
//                     doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(part)));
//                 }
//             }
//
//             inserted_images.insert(placeholder.clone());
//
//         } else {
//             let replaced_value = value.replace("<div><br></div>", "\n");
//             doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(replaced_value)));
//         }
//     }
//
//     doc.build().pack(cursor).map_err(|e| e.to_string())?;
//
//     Ok(text.to_owned())
// }
//
//
//
// fn replace_placeholder_in_docx(
//     state: &State<'_, AppState>,
//     output_path: &str,
//     replacements: &[(String, String)],
// ) -> Result<(), String> {
//     let file_path = get_resource_path("demo.docx", state)?;
//     let file = File::open(&file_path).map_err(|e| e.to_string())?;
//     let mut zip = ZipArchive::new(file).map_err(|e| e.to_string())?;
//
//     let buffer = Cursor::new(Vec::new());
//     let mut zip_writer = buffer;
//     let mut image_id_counter = 1;
//     let mut inserted_images = HashSet::new();
//
//     for i in 0..zip.len() {
//         let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
//         let file_name = file.name().to_string();
//         let mut file_content = Vec::new();
//         file.read_to_end(&mut file_content).map_err(|e| e.to_string())?;
//
//         if file_name == "word/document.xml" {
//             let xml_string = String::from_utf8_lossy(&file_content).to_string();
//             let mut reader = Reader::from_str(&xml_string);
//             let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 0);
//             reader.trim_text(true);
//
//             let mut buf = Vec::new();
//             let mut current_text = String::new();
//             let mut text_tag_open = false;
//
//             loop {
//                 match reader.read_event(&mut buf) {
//                     Ok(Event::Start(ref e)) if e.name() == b"w:t" => {
//                         text_tag_open = true;
//                         writer.write_event(Event::Start(e.clone())).unwrap();
//                     }
//                     Ok(Event::Text(e)) if text_tag_open => {
//                         let decoded_text = e.unescape_and_decode(&reader).map_err(|e| e.to_string())?;
//                         current_text.push_str(&decoded_text);
//                     }
//                     Ok(Event::End(ref e)) if e.name() == b"w:t" => {
//                         text_tag_open = false;
//                         // 使用replace_text_with_image_placeholder方法替换文本和图片
//                         let replaced_text = replace_text_with_image_placeholder(
//                             &current_text,
//                             replacements,
//                             &mut image_id_counter,
//                             &mut zip_writer,
//                             &mut inserted_images,
//                         )?;
//                         // 这里使用 Event::Text::escaped 来确保 XML 片段不会被转义
//                         writer.write_event(Event::Text(BytesText::from_escaped_str(&replaced_text))).unwrap();
//                         writer.write_event(Event::End(e.clone())).unwrap();
//                         current_text.clear();
//                     }
//                     Ok(Event::Eof) => break,
//                     Ok(e) => writer.write_event(e).unwrap(),
//                     Err(e) => return Err(e.to_string()),
//                 }
//                 buf.clear();
//             }
//             zip_writer.write_all(&writer.into_inner().into_inner())
//                 .map_err(|e| e.to_string())?;
//         } else {
//             zip_writer.write_all(&file_content)
//                 .map_err(|e| e.to_string())?;
//         }
//     }
//
//     let mut output_file = File::create(output_path).map_err(|e| e.to_string())?;
//     output_file.write_all(&zip_writer.into_inner())
//         .map_err(|e| e.to_string())?;
//
//     Ok(())
// }
//
//
//
//
//
//
// #[command]
// pub fn generate_report(
//     state: State<'_, AppState>,
//     company_name: String,
//     web_url: String,
//     find_date: String,
//     system_name: String,
//     vuln_name: String,
//     vuln_url: String,
//     vuln_description: String,
//     fix_suggestion: String,
//     assist_image: String,
//     test_process: String,
//     appendix: String,
// ) -> Result<(), String> {
//     let base_name = company_name.clone();
//     let output_path = get_unique_file_name(&base_name, "docx");
//
//     let replacements = vec![
//         ("CN".to_string(), company_name),
//         ("CU".to_string(), web_url),
//         ("CD".to_string(), find_date),
//         ("WT".to_string(), system_name),
//         ("VN".to_string(), vuln_name),
//         ("VU".to_string(), vuln_url),
//         ("VD".to_string(), vuln_description),
//         ("FS".to_string(), fix_suggestion),
//         ("AP".to_string(), assist_image),
//         ("TR".to_string(), test_process),
//         ("AD".to_string(), appendix),
//     ];
//
//     replace_placeholder_in_docx(&state, &output_path, &replacements)?;
//     println!("文档生成成功：{}", output_path);
//     Ok(())
// }
