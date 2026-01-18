use serde_json::json;
use tauri::{AppHandle, Emitter};
use crate::converters::common::OutputFormat;
use crate::converters::dispatcher::convert_image_auto;
use tauri_plugin_log::log::{error, info};

/// 批量转换多个文件（支持混合格式）
/// 
/// # 参数
/// - `app`: Tauri 应用句柄
/// - `files`: 文件对列表，每个元素包含 (输入路径, 输出路径)
/// - `format`: 目标输出格式
/// 
/// # 返回
/// 成功返回 Ok(())，失败返回错误信息
pub async fn batch_convert(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: String,
) -> Result<(), String> {
    info!("检查输出格式是否符合要求");

    let output_format = match OutputFormat::from_str(&format) {
        Ok(fmt) => fmt,
        Err(e) => {
            error!("不支持该输出格式：{} {}", &format, e);
            return Err(e.to_string());
        }
    };

    info!("开始调用批量转换格式 converters::dispatcher::convert_image_auto");
    batch_convert_images(app, files, output_format)
}

/// 批量转换图片的核心实现
fn batch_convert_images(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: OutputFormat,
) -> Result<(), String> {
    info!("遍历数组，进行转换...");

    files.into_iter().for_each(|(input, output)| {
        let _ = app.emit("conversion-update", json!({
            "path": input,
            "status": "converting",
            "progress": 0
        }));
        
        let result = convert_image_auto(app, &input, &output, format);
        
        match result {
            Ok(_) => {
                let _ = app.emit("conversion-update", json!({
                    "path": input,
                    "status": "done",
                    "progress": 100,
                    "output_path": output
                }));
            },
            Err(e) => {
                let _ = app.emit("conversion-update", json!({
                    "path": input,
                    "status": "error",
                    "error": e.to_string()
                }));
            }
        }
    });

    Ok(())
}