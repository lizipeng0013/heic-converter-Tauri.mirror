mod commands;
mod converters;
mod services;
mod setup;
mod utils;

// 使用 mimalloc 作为全局内存分配器，提升所有平台的性能
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use commands::conversion::{convert_images, force_exit, stop_conversion};
use commands::tray::show_tray;
use commands::window::{
    close_window, drag_window, hide_window, minimize_window, show_window, toggle_always_on_top,
    toggle_maximize_window,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup::init_plugins(tauri::Builder::default())
        .invoke_handler(tauri::generate_handler![
            // 窗口控制类
            minimize_window,
            hide_window,
            show_window,
            toggle_maximize_window,
            close_window,
            toggle_always_on_top,
            drag_window,
            // 托盘控制类
            show_tray,
            // 业务逻辑类
            convert_images,
            stop_conversion,
            force_exit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
