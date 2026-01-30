use tauri::{AppHandle, Emitter};
use crate::services::conversion::batch_convert;
use tauri_plugin_log::log::{info, debug};
use tracing::instrument;
use std::sync::atomic::{AtomicBool, Ordering};

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

        let was_stopped = batch_convert(&app_clone, file_pairs, format_clone, quality)
            .await
            .expect("批量转换出现严重错误");

        // 根据是否被停止，发送不同的完成事件
        if was_stopped {
            info!("转换任务被停止，发送停止完成事件");
            let _ = app_clone.emit("conversion-stopped", serde_json::json!({}));
        } else {
            info!("转换任务正常完成，发送批次完成事件");
            let _ = app_clone.emit("conversion-batch-finished", serde_json::json!({}));
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