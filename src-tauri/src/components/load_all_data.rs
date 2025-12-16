use serde_json::{json, Value};
use rusqlite::{Connection, params};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, command, Wry};
use tauri::Manager;

fn get_db_path(app_handle: &AppHandle<Wry>, db_name: &str) -> PathBuf {
    #[cfg(debug_assertions)]
        {
            let mut path = std::env::current_dir().expect("无法获取当前目录");
            path.push("resources");
            path.push(db_name);
            println!("[DEBUG] Debug 模式数据库路径: {:?}", path);
            path
        }

    #[cfg(not(debug_assertions))]
        {
            let app_data = app_handle
                .path()
                .app_data_dir()
                .expect("无法获取应用数据目录");
            fs::create_dir_all(&app_data).ok();

            let db_path = app_data.join(db_name);
            if !db_path.exists() {
                let resource_db = app_handle
                    .path()
                    .resource_dir()
                    .expect("无法获取资源目录")
                    .join(db_name);

                if !resource_db.exists() {
                    panic!("[ERROR] 打包资源中找不到数据库文件: {:?}", resource_db);
                }

                fs::copy(&resource_db, &db_path).expect(&format!(
                    "[ERROR] 复制数据库到应用数据目录失败: {:?}",
                    resource_db
                ));
                println!("[INFO] 数据库复制到可写目录: {:?} -> {:?}", resource_db, db_path);
            }
            println!("[INFO] Release 模式数据库路径: {:?}", db_path);
            db_path
        }
}

#[command]
pub fn load_all_data(app_handle: AppHandle) -> Result<Value, String> {
    // 1. 数据库路径
    let db_path = get_db_path(&app_handle, "vulnerabilities.db");
    println!("[INFO] 打开数据库: {:?}", db_path);

    let conn = match Connection::open(&db_path) {
        Ok(c) => {
            println!("[INFO] 数据库打开成功");
            c
        },
        Err(e) => {
            println!("[ERROR] 打开数据库失败: {}", e);
            return Err(e.to_string());
        }
    };

    // ================= event_type =================
    let mut event_types = vec![];
    {
        let sql = "SELECT id, dataType, dataName, dataValue, dataDesc, sortIdx, alarmDesc, affects, parentId FROM event_type ORDER BY sortIdx";
        println!("[INFO] 执行 SQL: {}", sql);
        let mut stmt = conn.prepare(sql).map_err(|e| {
            println!("[ERROR] 准备 SQL 失败: {}", e);
            e.to_string()
        })?;

        let mut rows = stmt.query([]).map_err(|e| {
            println!("[ERROR] 查询 SQL 失败: {}", e);
            e.to_string()
        })?;

        while let Some(row) = rows.next().map_err(|e| {
            println!("[ERROR] 读取行失败: {}", e);
            e.to_string()
        })? {
            let id: String = row.get(0).unwrap_or_default();
            let data_type: String = row.get(1).unwrap_or_default();
            let data_name: String = row.get(2).unwrap_or_default();
            let data_value: i64 = row.get(3).unwrap_or_default();
            let data_desc: String = row.get(4).unwrap_or_default();
            let sort_idx: i64 = row.get(5).unwrap_or_default();
            let alarm_desc: String = row.get(6).unwrap_or_default();
            let affects: String = row.get(7).unwrap_or_default();
            let parentId: String = row.get(8).unwrap_or_default();

            event_types.push(json!({
                "id": id,
                "data_value": data_value, // 对应上传API中 alarmSort int类型
                "data_type": data_type, // 用户展示前端情报类别
                "sort_idx": sort_idx, // 垃圾参数
                "dataName": data_desc,// 用于前端展示
                "informationName": data_name, //对应上传API中 informationName
                "alarmDesc": alarm_desc,// 对应上传API中 alarmDesc
                "affectsType": affects, // 对应上传API中 affectsType
                "parentId": parentId, // 用于过滤前端下拉
            }));
        }
        println!("[INFO] 共读取 {} 条 event_type", event_types.len());
    }

    // ================= province =================
    let mut provinces = vec![];  // 省级
    {
        let sql = "SELECT id, name, location FROM province ORDER BY id";
        println!("[INFO] 执行 SQL: {}", sql);
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let id: String = row.get(0).unwrap_or_default();
            let name: String = row.get(1).unwrap_or_default();
            let location: String = row.get(2).unwrap_or_default();

            provinces.push(json!({
            "id": id,
            "name": name,
            "location": location,
        }));
        }
    }

    let mut areas = vec![];  // 市/区/镇
    {
        let sql = "SELECT id, pid, name, level, location FROM area ORDER BY id";
        println!("[INFO] 执行 SQL: {}", sql);
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let id: String = row.get(0).unwrap_or_default();
            let pid: String = row.get(1).unwrap_or_default();
            let name: String = row.get(2).unwrap_or_default();
            let level: String = row.get(3).unwrap_or_default();
            let location: String = row.get(4).unwrap_or_default();

            areas.push(json!({
            "id": id,
            "pid": pid,
            "name": name,
            "level": level,
            "location": location,
        }));
        }
    }

    // ================= pending report =================
    let pending_report = match crate::implements::process_json::load_all_pending_reports(&app_handle) {
        Ok(v) => {
            println!("[INFO] 成功读取 pending_report");
            v
        },
        Err(e) => {
            println!("[WARN] 读取 pending_report 失败: {}", e);
            json!({})
        }
    };

    // ================= 合并返回 =================
    let mut result = json!({
        "provinces": provinces,
        "areas": areas,
        "event_types": event_types,
    });

    if let Value::Object(map) = pending_report {
        for (k, v) in map {
            result[k] = v;
        }
    }
    Ok(result)
}