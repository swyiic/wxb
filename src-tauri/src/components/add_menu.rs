// src-tauri/components/add_menu.rs

use tauri::{AppHandle, Runtime};
use tauri::menu::{Menu, MenuItemBuilder, SubmenuBuilder};

#[warn(dead_code)]
pub fn create_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, tauri::Error> {
    //
    // 创建你需要的所有菜单项（不设置任何快捷键）
    //
    let add_vuln = MenuItemBuilder::new("添加漏洞")
        .id("add_vulnerability")
        .build(app)?;

    let add_query = MenuItemBuilder::new("添加爱企查配置")
        .id("add_query_info")
        .build(app)?;

    let add_platform = MenuItemBuilder::new("添加平台登录配置")
        .id("add_platform_login")
        .build(app)?;

    let about = MenuItemBuilder::new("关于工具")
        .id("about_department")
        .build(app)?;

    let paste = MenuItemBuilder::new("粘贴")
        .id("paste_event")
        .build(app)?;

    //
    // 把所有菜单都放到 Settings 下
    //
    let settings_submenu = SubmenuBuilder::new(app, "设置")
        .item(&add_vuln)
        .item(&add_query)
        .item(&add_platform)
        .item(&paste)
        .item(&about)
        .build()?;

    //
    // App 主菜单（左上角应用名菜单）
    //
    let app_menu = SubmenuBuilder::new(app, "网信办支撑工具")
        .item(&settings_submenu)
        .build()?;

    //
    // 返回菜单（你的 main.rs 不会调用 set_menu，所以不会破坏快捷键）
    //
    Menu::with_items(app, &[&app_menu])
}