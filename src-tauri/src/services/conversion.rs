use serde_json::json;
use tauri::{AppHandle, Emitter};
use crate::converters::common::OutputFormat;
use crate::converters::dispatcher::convert_image_auto;
use crate::commands::conversion::should_stop;
use tauri_plugin_log::log::{error, info, debug};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

/// 批量转换多个文件（支持混合格式）
///
/// # 参数
/// - `app`: Tauri 应用句柄
/// - `files`: 文件对列表，每个元素包含 (输入路径, 输出路径)
/// - `format`: 目标输出格式
/// - `quality`: JPEG质量（1-100），仅对JPEG格式有效
///
/// # 返回
/// 成功返回 Ok(())，失败返回错误信息
pub async fn batch_convert(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: String,
    quality: u8,
) -> Result<(), String> {
    debug!("验证输出格式: {}, 质量: {}", format, quality);

    let output_format = match OutputFormat::from_str(&format, quality) {
        Ok(fmt) => fmt,
        Err(e) => {
            error!("不支持的输出格式: {} - {}", format, e);
            return Err(e.to_string());
        }
    };

    info!("开始批量转换，共 {} 个文件，目标格式: {:?}", files.len(), output_format);
    batch_convert_images(app, files, output_format)
}

/// 批量转换图片的核心实现（并行处理）
fn batch_convert_images(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: OutputFormat,
) -> Result<(), String> {
    let total = files.len();
    debug!("开始并行处理 {} 个文件", total);

    // 使用 Arc<Mutex> 来共享计数器，因为并行处理需要线程安全
    let success_count = Arc::new(Mutex::new(0));
    let error_count = Arc::new(Mutex::new(0));

    // 使用并行迭代器处理文件
    files.into_par_iter().enumerate().for_each(|(index, (input, output))| {
        // 检查是否应该停止
        if should_stop() {
            info!("收到停止信号，中止转换任务");
            return;
        }

        let current = index + 1;
        debug!("处理文件 {}/{}: {}", current, total, input);

        let _ = app.emit("conversion-update", json!({
            "path": input,
            "status": "converting",
            "progress": 0,
            "current": current,
            "total": total
        }));
        
        let result = convert_image_auto(app, &input, &output, format);
        
        match result {
            Ok(_) => {
                *success_count.lock().unwrap() += 1;
                info!("✓ 转换成功 {}/{}: {} -> {}", current, total, input, output);
                let _ = app.emit("conversion-update", json!({
                    "path": input,
                    "status": "done",
                    "progress": 100,
                    "output_path": output,
                    "current": current,
                    "total": total
                }));
            },
            Err(e) => {
                *error_count.lock().unwrap() += 1;
                error!("✗ 转换失败 {}/{}: {} - {}", current, total, input, e);
                let _ = app.emit("conversion-update", json!({
                    "path": input,
                    "status": "error",
                    "error": e.to_string(),
                    "current": current,
                    "total": total
                }));
            }
        }
    });

    let success = *success_count.lock().unwrap();
    let error = *error_count.lock().unwrap();
    info!("批量转换完成 - 成功: {}, 失败: {}, 总计: {}", success, error, total);
    Ok(())
}