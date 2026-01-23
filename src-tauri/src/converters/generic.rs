use super::common::{ConversionError, OutputFormat, save_image_buffer};
use image::ImageReader;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::trace;

/// 处理普通图片格式转换 (JPEG, PNG, WebP, GIF, BMP等)
pub fn convert_regular_image(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    trace!("转换普通图片: {} -> {}", input_path, output_path);

    let img = ImageReader::open(input_path)?
        .decode()?;
    trace!("图片尺寸: {}x{}", img.width(), img.height());
    
    // 转换为 RGB 图像缓冲区
    let rgb_img = img.to_rgb8();
    
    // 使用公共保存函数
    save_image_buffer(&rgb_img, output_path, format)
}
