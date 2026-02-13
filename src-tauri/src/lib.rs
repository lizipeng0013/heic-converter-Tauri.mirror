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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup::init_plugins(tauri::Builder::default())
        .invoke_handler(tauri::generate_handler![
            // 业务逻辑类
            convert_images,
            stop_conversion,
            force_exit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
