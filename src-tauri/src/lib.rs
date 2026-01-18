// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

use std::path::Path;
use commands::{window, converter, batch_convert};
use tauri_plugin_log::{Target, TargetKind, log::info};
use tauri_plugin_log::log::error;
use crate::commands::convert_images;
use crate::commands::window::window::{close_window, minimize_window, toggle_always_on_top, toggle_maximize_window};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn build_target_path(source_path: &str, target_type: &str, output_folder: &str) -> String {
    // const nameParts = source_path.
    // const nameParts =
    // const name = nameParts[nameParts.length - 1];
    let path = Path::new(source_path);
    let stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");
    let file_name = format!("{}.{}", stem, target_type);
    let target_path = Path::new(output_folder).join(file_name);
    target_path.to_string_lossy().into_owned()

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        
        // 日志插件
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir { file_name: None }),
            Target::new(TargetKind::Webview),
        ]).build())

        // 注册所有 Command
        // 使用 路径::函数名 的方式注册
        .invoke_handler(tauri::generate_handler![
            greet,
            // 窗口控制类
            minimize_window,
            toggle_maximize_window,
            close_window,
            toggle_always_on_top,
            // 业务逻辑类
            convert_images,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
