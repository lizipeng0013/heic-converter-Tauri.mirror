// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Window;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// --- 最小化窗口 ---
#[tauri::command]
fn minimize_window(window: Window) {
    window.minimize().ok(); // 改为 ok() 更安全，避免程序崩溃
}

// --- 最大化/还原窗口 (已修复) ---
#[tauri::command]
fn toggle_maximize_window(window: Window) {
    window
        .is_maximized()
        .map(|is_maximized| {
            if is_maximized {
                let _ = window.unmaximize();
            } else {
                let _ = window.maximize();
            }
        })
        .ok();
}

// --- 关闭窗口 ---
#[tauri::command]
fn close_window(window: Window) {
    window.close().ok(); // 改为 ok() 更安全
}

use std::sync::atomic::{AtomicBool, Ordering};
// 全局状态，初始为 false
static IS_TOP: AtomicBool = AtomicBool::new(false);

// --- 窗口置顶 ---
#[tauri::command]
async fn toggle_always_on_top(window: Window) -> Result<(), String> {
    // 获取或初始化状态（初始为 false）
    let new_state = !IS_TOP.load(Ordering::SeqCst);
    window
        .set_always_on_top(new_state)
        .map_err(|e| e.to_string())?;
    IS_TOP.store(new_state, Ordering::SeqCst);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            minimize_window,
            toggle_maximize_window,
            close_window,
            toggle_always_on_top
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
