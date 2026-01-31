use chrono::Local;
use tauri_plugin_log::{Target, TargetKind, log::LevelFilter};

/// 初始化所有 Tauri 插件
pub fn init_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    // 确定日志级别
    // 优先级：环境变量 > 构建配置
    let log_level = if let Ok(level_str) = std::env::var("RUST_LOG") {
        // 如果用户设置了 RUST_LOG 环境变量，使用它
        parse_log_level(&level_str)
    } else if cfg!(debug_assertions) {
        // 开发模式：显示所有日志（包括debug）
        LevelFilter::Debug
    } else {
        // 生产模式：只显示info及以上级别的日志
        LevelFilter::Info
    };

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        // 日志插件
        .plugin(tauri_plugin_log::Builder::new()
            .level(log_level)
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .format(move | out, message, record| {
                let now = Local::now();
                out.finish(format_args!(
                        "[{}] [{}] [{}] {}",
                        now.format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        record.target(),
                        message
                    ));
            })
            .build())
}

/// 解析日志级别字符串
fn parse_log_level(level_str: &str) -> LevelFilter {
    match level_str.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => {
            // 如果无法解析，默认使用info
            eprintln!("无法解析日志级别 '{}', 使用默认值 info", level_str);
            LevelFilter::Info
        }
    }
}