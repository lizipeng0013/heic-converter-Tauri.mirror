pub mod commands;
pub mod converters;
mod services;
mod setup;
pub mod utils;
pub mod memory_pool;

// 全局线程池
use once_cell::sync::Lazy;
use rayon::ThreadPoolBuilder;

/// 全局转换线程池
///
/// 用于避免每次批量转换都创建新线程池，减少线程创建和销毁的开销。
/// 线程池大小为 CPU 核心数的 75%，保留 25% 给 UI。
pub static CONVERSION_POOL: Lazy<rayon::ThreadPool> = Lazy::new(|| {
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    
    ThreadPoolBuilder::new()
        .num_threads((num_cpus * 3 / 4).max(2))
        .thread_name(|i| format!("converter-{}", i))
        .build()
        .expect("Failed to create conversion pool")
});

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
