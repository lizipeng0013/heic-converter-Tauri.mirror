use super::common::{ConversionError, OutputFormat, save_image_buffer};
use libheif_rs::{LibHeif, HeifContext, ColorSpace, RgbChroma, Chroma};
use image::{ImageBuffer, RgbImage};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::{debug, info, warn};

/// 检测是否为 HEIC/HEIF 文件
pub fn is_heic_format(input_path: &str) -> bool {
    // 首先检查扩展名（快速检查）
    let path = std::path::Path::new(input_path);
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        if ext_str == "heic" || ext_str == "heif" {
            return true;
        }
    }
    
    // 然后尝试用 libheif 打开（更准确）
    match HeifContext::read_from_file(input_path) {
        Ok(ctx) => ctx.number_of_top_level_images() > 0,
        Err(_) => false,
    }
}

/// HEIC/HEIF 专用转换器
pub fn convert_heic_image(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    info!("转换HEIC图片: {} -> {}", input_path, output_path);
    
    let libheif = LibHeif::new();
    let ctx = HeifContext::read_from_file(input_path)?;
    let handle = ctx.primary_image_handle()?;
    
    info!("HEIC图像信息: {}x{}", handle.width(), handle.height());
    // let _ = app.emit("conversion-update", json!({
    //         "path": input_path,
    //         "status": "converting",
    //         "progress": 10
    //     }));
    // 尝试多种解码策略
    let image = if let Ok(img) = libheif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None) {
        info!("使用交错RGB格式解码");
        img
    } else if let Ok(img) = libheif.decode(&handle, ColorSpace::YCbCr(Chroma::C420), None) {
        info!("使用YUV 420格式解码");
        img
    } else if let Ok(img) = libheif.decode(&handle, ColorSpace::YCbCr(Chroma::C444), None) {
        info!("使用YUV 444格式解码");
        img
    } else {
        warn!("所有解码尝试失败，尝试默认解码");
        libheif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?
    };
    // let _ = app.emit("conversion-update", json!({
    //         "path": input_path,
    //         "status": "converting",
    //         "progress": 30
    //     }));
    let planes = image.planes();
    let width = image.width() as u32;
    let height = image.height() as u32;
    
    let img_buffer: RgbImage = if let Some(interleaved) = planes.interleaved {
        // 交错RGB数据
        let stride = interleaved.stride;
        let data = interleaved.data;
        let mut buffer = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y as usize) * stride + (x as usize) * 3;
                if idx + 2 < data.len() {
                    buffer.put_pixel(x, y, image::Rgb([
                        data[idx],
                        data[idx + 1],
                        data[idx + 2]
                    ]));
                }
            }
        }
        buffer
    } else if let (Some(y), Some(cb), Some(cr)) = (&planes.y, &planes.cb, &planes.cr) {
        // YUV数据（最常见）
        let mut buffer = ImageBuffer::new(width, height);
        
        for y_pos in 0..height {
            for x_pos in 0..width {
                let y_idx = (y_pos as usize) * y.stride + (x_pos as usize);
                let uv_x = x_pos as usize / 2;
                let uv_y = y_pos as usize / 2;
                let cb_idx = uv_y * cb.stride + uv_x;
                let cr_idx = uv_y * cr.stride + uv_x;
                
                if y_idx < y.data.len() && cb_idx < cb.data.len() && cr_idx < cr.data.len() {
                    let y_val = y.data[y_idx] as f32;
                    let cb_val = cb.data[cb_idx] as f32 - 128.0;
                    let cr_val = cr.data[cr_idx] as f32 - 128.0;
                    
                    let r = (y_val + 1.402 * cr_val).clamp(0.0, 255.0) as u8;
                    let g = (y_val - 0.344136 * cb_val - 0.714136 * cr_val).clamp(0.0, 255.0) as u8;
                    let b = (y_val + 1.772 * cb_val).clamp(0.0, 255.0) as u8;
                    
                    buffer.put_pixel(x_pos, y_pos, image::Rgb([r, g, b]));
                }
            }
        }
        buffer
    } else {
        warn!("无法处理的平面格式");
        return Err(ConversionError::UnsupportedInputFormat("无法解码的HEIC格式".to_string()));
    };
    let _ = app.emit("conversion-update", json!({
            "path": input_path,
            "status": "converting",
            "progress": 90
        }));
    
    save_image_buffer(&img_buffer, output_path, format)

}