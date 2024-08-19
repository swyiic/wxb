use tauri::{CustomMenuItem, Menu, Submenu};

pub fn create_menu() -> Menu {
    // 子菜单项
    let vulnerability = CustomMenuItem::new("show_add_vulnerability", "添加漏洞");
    let query_info = CustomMenuItem::new("show_add_query_info", "添加爱企查信息");
    let platform_login_info = CustomMenuItem::new("show_add_platform_login_info", "添加情报平台信息");
    let about_department = CustomMenuItem::new("show_about_department", "上海事业部业务支撑部");

    // 菜单项
    let config_submenu = Submenu::new(
        "配置信息",
        Menu::new()
            .add_item(query_info)
            .add_item(platform_login_info),
    );

    let vul_menu = Submenu::new("漏洞信息", Menu::new().add_item(vulnerability));

    let about_menu = Submenu::new("关于", Menu::new().add_item(about_department));

    // 创建主菜单
    Menu::new()
        .add_submenu(config_submenu)
        .add_submenu(vul_menu)
        .add_submenu(about_menu)
}
