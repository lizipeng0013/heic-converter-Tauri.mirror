use serde_json::json;
use tauri::{AppHandle, Emitter};
use super::common::{ConversionError, OutputFormat};
use super::heic_conv::{is_heic_format, convert_heic_image};
use super::image_conv::convert_regular_image;
use tauri_plugin_log::log::{info, debug};

/// 智能图片转换 - 自动检测格式并选择正确的转换器
pub fn convert_image_auto(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    info!("智能转换: {} -> {}", input_path, output_path);
    
    // 检测是否为HEIC格式
    if is_heic_format(input_path) {
        debug!("检测到HEIC格式，使用HEIC转换器");
        convert_heic_image(app, input_path, output_path, format)
    } else {
        debug!("检测到普通图片格式，使用普通转换器");
        convert_regular_image(app, input_path, output_path, format)
    }
}

/// 批量转换多个文件（支持混合格式）
pub fn batch_convert_images(
    app: &AppHandle,
    files: Vec<(String, String)>, // (输入路径, 输出路径)
    format: OutputFormat,
) -> Result<(), String> {
    info!("遍历数组，进行转换...");

    files.into_iter().for_each(|(input, output)| {
        let _ = app.emit("conversion-update", json!({
            "path": input,
            "status": "converting",
            "progress": 0
        }));
        let result = convert_image_auto(&app, &input, &output, format);
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