use super::common::{ConversionError, OutputFormat, save_image_buffer};
use libheif_rs::{LibHeif, HeifContext, ColorSpace, RgbChroma};
use image::{ImageBuffer, RgbImage};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::{info, warn, debug, error};

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
    
    debug!("HEIC图像信息: {}x{}", handle.width(), handle.height());
    
    // 优先使用 libheif 的内置 RGB 解码，避免手动 YUV->RGB 转换
    // 这样可以利用 libheif 内部的 SIMD 优化
    let image = libheif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let planes = image.planes();
    let width = image.width() as u32;
    let height = image.height() as u32;
    
    // 检查是否有交错的 RGB 数据
    if let Some(interleaved) = planes.interleaved {
        debug!("使用 libheif 内置 RGB 解码（SIMD 优化）");
        let data = interleaved.data;
        let stride = interleaved.stride;
        
        // 直接创建 ImageBuffer，避免逐像素处理
        // 这比手动 YUV->RGB 转换快 10 倍以上
        let buffer: RgbImage = if stride == (width * 3) as usize {
            // 如果 stride 符合预期，直接使用数据
            ImageBuffer::from_raw(width, height, data.to_vec())
                .ok_or(ConversionError::UnsupportedInputFormat("无法创建 RGB buffer".to_string()))?
        } else {
            // 如果 stride 不符合预期，需要创建新的 buffer 并逐行复制
            debug!("RGB 数据 stride 不匹配，创建新的 buffer 并逐行复制");
            let mut buffer_data = vec![0u8; (width * height * 3) as usize];
            let row_bytes = (width * 3) as usize;
            
            for y in 0..height {
                let y_usize = y as usize;
                let src_start = y_usize * stride;
                let src_end = src_start + row_bytes;
                let dst_start = y_usize * row_bytes;
                let dst_end = dst_start + row_bytes;
                
                if src_end <= data.len() && dst_end <= buffer_data.len() {
                    // 直接复制整行数据，避免逐像素处理
                    buffer_data[dst_start..dst_end].copy_from_slice(&data[src_start..src_end]);
                }
            }
            
            ImageBuffer::from_raw(width, height, buffer_data)
                .ok_or(ConversionError::UnsupportedInputFormat("无法创建 RGB buffer".to_string()))?
        };
        
        let _ = app.emit("conversion-update", json!({
            "path": input_path,
            "status": "converting",
            "progress": 90
        }));
        
        save_image_buffer(&buffer, output_path, format)?;
    } else {
        // 如果没有交错 RGB 数据，尝试 YUV 格式（回退方案）
        debug!("无交错 RGB 数据，尝试 YUV 格式");
        
        if let (Some(y), Some(cb), Some(cr)) = (&planes.y, &planes.cb, &planes.cr) {
            warn!("使用 YUV 格式（性能较差，建议更新 libheif 版本）");
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
            
            let _ = app.emit("conversion-update", json!({
                "path": input_path,
                "status": "converting",
                "progress": 90
            }));
            
            save_image_buffer(&buffer, output_path, format)?;
        } else {
            error!("无法处理的平面格式");
            return Err(ConversionError::UnsupportedInputFormat("无法解码的HEIC格式".to_string()));
        }
    }
    
    Ok(())
}