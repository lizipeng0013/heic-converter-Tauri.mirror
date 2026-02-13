
use tauri::{command, Window};

// --- 隐藏窗口 ---
// 用于最小化到托盘功能
#[command]
pub fn hide_window(window: Window) {
    window.hide().ok();
}

// --- 显示窗口 ---
// 用于从托盘恢复窗口
#[command]
pub fn show_window(window: Window) {
    window.show().ok();
    window.unminimize().ok();
    window.set_focus().ok();
}

// 以下窗口操作已迁移到前端 API，不再需要后端命令：
// - minimize_window      -> window.minimize()
// - toggle_maximize_window -> window.toggleMaximize()
// - close_window         -> window.close()
// - drag_window          -> window.startDragging()
// - toggle_always_on_top -> window.setAlwaysOnTop()
