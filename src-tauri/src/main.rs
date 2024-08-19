// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #[macro_use]
mod components;
mod implements;
use components::about_department::show_about_department;
use components::add_menu::create_menu;
use components::add_platform_login_info::add_platform_login_info;
// use components::add_query_info::add_query_info;
use components::add_vulnerability::add_vulnerability;
use components::delete_vulnerability::delete_vulnerability;
use components::edit_vulnerability::edit_vulnerability;
use components::query_all_vulnerability::query_all_vulnerability;
use components::query_all_vulnerability::search_vulnerability_by_name;
use implements::python_processor::generate_report;
use tauri::Manager;
use tauri::{Env, PackageInfo, WindowMenuEvent};

// 定义全局状态
pub struct AppState {
    package_info: PackageInfo,
    env: Env,
}

fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "show_add_vulnerability" => {
            if let Err(e) = event.window().emit("show_add_vulnerability", ()) {
                eprintln!("发送指令漏洞失败: {}", e);
            }
        }
        "show_add_query_info" => {
            if let Err(e) = event.window().emit("show_add_query_info", ()) {
                eprintln!("发送指令爱企查失败: {}", e);
            }
        }
        "show_add_platform_login_info" => {
            if let Err(e) = event.window().emit("show_add_platform_login_info", ()) {
                eprintln!("发送指令平台失败: {}", e);
            }
        }
        "show_about_department" => {
            if let Err(e) = event.window().emit("show_about_department", ()) {
                eprintln!("发送指令关于失败: {}", e);
            }
        }
        _ => {}
    }
}

fn main() {
    let menu = create_menu();

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(handle_menu_event)
        .setup(|app| {
            let package_info = app.package_info().clone();
            let env = app.env().clone();
            app.manage(AppState { package_info, env });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_vulnerability,
            edit_vulnerability,
            delete_vulnerability,
            query_all_vulnerability,
            search_vulnerability_by_name,
            generate_report,
            add_platform_login_info,
            show_about_department
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
