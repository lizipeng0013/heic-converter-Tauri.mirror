// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use commands::window;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        // 注册所有 Command
        // 使用 路径::函数名 的方式注册
        .invoke_handler(tauri::generate_handler![
            greet,
            // 窗口控制类
            window::minimize_window,
            window::toggle_maximize_window,
            window::close_window,
            window::toggle_always_on_top
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
