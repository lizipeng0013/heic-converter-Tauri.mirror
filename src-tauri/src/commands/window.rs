use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{command, Window};

// 全局状态，初始为 false
static IS_TOP: AtomicBool = AtomicBool::new(false);

// --- 最小化窗口 ---
#[command]
pub fn minimize_window(window: Window) {
    window.minimize().ok(); // 改为 ok() 更安全，避免程序崩溃
}

// --- 隐藏窗口 ---
#[command]
pub fn hide_window(window: Window) {
    window.hide().ok();
}

// --- 显示窗口 ---
#[command]
pub fn show_window(window: Window) {
    window.show().ok();
    window.unminimize().ok();
    window.set_focus().ok();
}

// --- 最大化/还原窗口 (已修复) ---
#[tauri::command]
pub fn toggle_maximize_window(window: Window) {
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
#[command]
pub fn close_window(window: Window) {
    window.close().ok();
}

// --- 窗口置顶 ---
#[command]
pub async fn toggle_always_on_top(window: Window) -> Result<(), String> {
    // 获取或初始化状态（初始为 false）
    let new_state = !IS_TOP.load(Ordering::SeqCst);
    window
        .set_always_on_top(new_state)
        .map_err(|e| e.to_string())?;
    IS_TOP.store(new_state, Ordering::SeqCst);
    Ok(())
}

#[command]
pub fn drag_window(window: Window) {
    window.start_dragging().ok();
}
