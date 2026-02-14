use crate::services::conversion::batch_convert;
use notify_rust::Notification;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_log::log::{debug, error, info};
use tracing::instrument;

// 全局停止标志
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

/// 批量转换图片命令
///
/// # 参数
/// - `app`: Tauri 应用句柄
/// - `paths`: 待转换文件路径列表
/// - `target_type`: 目标格式（"jpg", "png"）
/// - `output_folder`: 输出文件夹
/// - `quality`: JPEG质量（1-100），仅对JPEG格式有效
#[tauri::command]
#[instrument(skip(app), fields(
    file_count = paths.len(),
    format = %target_type,
    quality = quality
))]
pub async fn convert_images(
    app: AppHandle,
    paths: Vec<String>,
    target_type: String,
    output_folder: String,
    quality: u8,
) -> Result<(), String> {
    // 验证输入参数
    if paths.is_empty() {
        return Err("没有选择任何文件".to_string());
    }

    // 验证输出目录权限
    crate::utils::validate_output_folder(&output_folder)?;

    // 验证格式
    let valid_formats = ["jpeg", "jpg", "png", "webp", "bmp", "tiff", "ico"];
    if !valid_formats.contains(&target_type.to_lowercase().as_str()) {
        return Err(format!(
            "不支持的输出格式: {}. 支持的格式: {}",
            target_type,
            valid_formats.join(", ")
        ));
    }

    // 验证质量参数
    if quality == 0 || quality > 100 {
        return Err(format!("质量参数必须在 1-100 之间，当前值: {}", quality));
    }

    let app_clone = app.clone();
    let paths_clone = paths.clone();
    let format_clone = target_type.clone();

    // 重置停止标志
    SHOULD_STOP.store(false, Ordering::SeqCst);

    // 开启后台任务，不阻塞主响应
    tauri::async_runtime::spawn(async move {
        info!("开始后台转换任务");

        let mut file_pairs: Vec<(String, String)> = Vec::new();
        for path in paths_clone {
            debug!("构建输出路径: {}", path);
            let target_path = crate::utils::build_target_path(&path, &format_clone, &output_folder);
            file_pairs.push((path, target_path));
        }

        // 处理批量转换结果
        let was_stopped = match batch_convert(&app_clone, file_pairs, format_clone, quality).await {
            Ok(result) => result,
            Err(e) => {
                error!("批量转换失败: {}", e);
                // 发送错误事件到前端
                let _ = app_clone.emit(
                    "conversion-failed",
                    serde_json::json!({
                        "errorMessage": "转换任务执行失败，请检查日志"
                    }),
                );
                return; // 退出任务，不继续处理
            },
        };

        // 根据是否被停止，发送不同的完成事件
        if was_stopped {
            info!("转换任务被停止，发送停止完成事件");
            let _ = app_clone.emit("conversion-stopped", serde_json::json!({}));
        } else {
            info!("转换任务正常完成，发送批次完成事件");
            let _ = app_clone.emit("conversion-batch-finished", serde_json::json!({}));

            // 正常完成时，检查窗口状态并发送系统通知（仅当窗口最小化或隐藏时）
            if let Some(window) = app_clone.get_webview_window("main") {
                if let (Ok(is_minimized), Ok(is_visible)) =
                    (window.is_minimized(), window.is_visible())
                {
                    if is_minimized || !is_visible {
                        debug!("窗口最小化或隐藏，发送系统通知");

                        #[cfg(target_os = "windows")]
                        {
                            let _ = Notification::new()
                                .app_id("tech.hotime.heic-converter")
                                .summary("转换完成")
                                .body("图片转换已完成")
                                .show();
                        }

                        #[cfg(not(target_os = "windows"))]
                        {
                            let _ = Notification::new()
                                .summary("转换完成")
                                .body("图片转换已完成")
                                .show();
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

/// 停止转换命令
#[tauri::command]
pub fn stop_conversion() -> Result<(), String> {
    info!("收到停止转换请求");
    SHOULD_STOP.store(true, Ordering::SeqCst);
    Ok(())
}

/// 检查是否应该停止
pub fn should_stop() -> bool {
    SHOULD_STOP.load(Ordering::SeqCst)
}

/// 强制退出应用
#[tauri::command]
pub fn force_exit(app: AppHandle) {
    app.exit(0);
}
