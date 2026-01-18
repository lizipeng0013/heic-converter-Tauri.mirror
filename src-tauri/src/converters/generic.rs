use super::common::{ConversionError, OutputFormat, save_image_buffer};
use image::ImageReader;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::info;

/// 处理普通图片格式转换 (JPEG, PNG, WebP, GIF, BMP等)
pub fn convert_regular_image(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    info!("转换普通图片: {} -> {}", input_path, output_path);

    // let _ = app.emit("conversion-update", json!({
    //         "path": input_path,
    //         "status": "converting",
    //         "progress": 10
    //     }));
    // 使用 with_guessed_format 而不是 open，更可靠
    let img = ImageReader::open(input_path)?
        .decode()?;
    // let _ = app.emit("conversion-update", json!({
    //         "path": input_path,
    //         "status": "converting",
    //         "progress": 30
    //     }));
    // 转换为 RGB 图像缓冲区
    let rgb_img = img.to_rgb8();
    let _ = app.emit("conversion-update", json!({
            "path": input_path,
            "status": "converting",
            "progress": 90
        }));
    // 使用公共保存函数
    save_image_buffer(&rgb_img, output_path, format)
}
