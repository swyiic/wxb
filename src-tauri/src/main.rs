#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod implements;
mod components;

use components::add_vulnerability::add_vulnerability;
use components::delete_vulnerability::delete_vulnerability;
use components::edit_vulnerability::edit_vulnerability;
use components::add_platform_login_info::{save_auth_from_headers};
use components::query_all_vulnerability::{query_all_vulnerability, search_vulnerability_by_name};
use components::load_all_data::load_all_data;
use implements::process_json::load_latest_pending_report;
use implements::process_docx::generate_report;
use implements::process_upload::{get_report_number,process_submit_form};


use tauri::{Emitter, Manager};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            app.on_menu_event(|app, event| {
                let window = app.get_webview_window("main").unwrap();

                match event.id().as_ref() {
                    "add_vulnerability" => { window.emit("show_add_vulnerability", ()).unwrap(); }
                    "add_platform_login" => { window.emit("show_add_platform_login_info", ()).unwrap(); }
                    "save_auth_from_headers" =>{window.emit("save_auth_from_headers",()).unwrap();}
                    "load_all_data" =>{window.emit("load_all_data",()).unwrap();}
                    "get_report_number" =>{window.emit("get_report_number",()).unwrap();}
                    "process_submit_form" =>{window.emit("process_submit_form",()).unwrap();}
                    _ => {}
                }
                let _ = window.set_focus();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_vulnerability,
            edit_vulnerability,
            delete_vulnerability,
            query_all_vulnerability,
            search_vulnerability_by_name,
            generate_report,
            save_auth_from_headers,
            load_all_data,
            get_report_number,
            process_submit_form
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}