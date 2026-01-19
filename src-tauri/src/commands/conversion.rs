use tauri::{AppHandle, Emitter};
use crate::services::conversion::batch_convert;
use tauri_plugin_log::log::{info, debug};
use tracing::instrument;

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
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let app_clone = app.clone();
    let paths_clone = paths.clone();
    let format_clone = target_type.clone();

    // 开启后台任务，不阻塞主响应
    tauri::async_runtime::spawn(async move {
        info!("开始后台转换任务");

        let mut file_pairs: Vec<(String, String)> = Vec::new();
        for path in paths_clone {
            debug!("构建输出路径: {}", path);
            let target_path = crate::utils::build_target_path(&path, &format_clone, &output_folder);
            file_pairs.push((path, target_path));
        }

        let _ = batch_convert(&app_clone, file_pairs, format_clone, quality)
            .await
            .expect("批量转换出现严重错误");

        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let spend_time = end_time - start_time;

        info!("批量转换完成，耗时 {}ms", spend_time);

        let _ = app_clone.emit("conversion-batch-finished", serde_json::json!({
            "spend_time": spend_time
        }));
    });

    Ok(())
}