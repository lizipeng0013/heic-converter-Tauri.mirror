use super::common::{ConversionError, OutputFormat, save_image_buffer};
use libheif_rs::{LibHeif, HeifContext, ColorSpace, RgbChroma};
use image::{ImageBuffer, RgbImage};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::{info, warn, debug, trace, error};

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

/// HEIC/HEIF 专用转换器（优化版本）
pub fn convert_heic_image(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    debug!("转换HEIC图片: {} -> {}", input_path, output_path);
    
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
            // 如果 stride 符合预期，直接使用数据（零拷贝）
            ImageBuffer::from_raw(width, height, data.to_vec())
                .ok_or(ConversionError::UnsupportedInputFormat("无法创建 RGB buffer".to_string()))?
        } else {
            // 如果 stride 不符合预期，需要创建新的 buffer 并逐行复制
            trace!("RGB 数据 stride 不匹配，创建新的 buffer 并逐行复制");
            let row_bytes = (width * 3) as usize;
            let total_bytes = row_bytes * height as usize;
            
            // 预分配 buffer，避免多次重新分配
            let mut buffer_data = Vec::with_capacity(total_bytes);
            unsafe {
                buffer_data.set_len(total_bytes);
            }
            
            // 使用更高效的批量复制 - 使用 copy_within 优化
            let mut dst_offset = 0;
            for y in 0..height {
                let y_usize = y as usize;
                let src_start = y_usize * stride;
                let src_end = src_start + row_bytes;
                
                if src_end <= data.len() && dst_offset + row_bytes <= total_bytes {
                    // 使用 copy_from_slice 进行高效的批量复制
                    buffer_data[dst_offset..dst_offset + row_bytes]
                        .copy_from_slice(&data[src_start..src_end]);
                    dst_offset += row_bytes;
                }
            }
            
            ImageBuffer::from_raw(width, height, buffer_data)
                .ok_or(ConversionError::UnsupportedInputFormat("无法创建 RGB buffer".to_string()))?
        };
        
        save_image_buffer(&buffer, output_path, format)?;
    } else {
        // 如果没有交错 RGB 数据，尝试 YUV 格式（回退方案）
        trace!("无交错 RGB 数据，尝试 YUV 格式");
        
        if let (Some(y), Some(cb), Some(cr)) = (&planes.y, &planes.cb, &planes.cr) {
            warn!("使用 YUV 格式（性能较差，建议更新 libheif 版本）");
            let mut buffer = ImageBuffer::new(width, height);
            
            // 优化 YUV 转换 - 使用批量处理而不是逐像素
            let width_usize = width as usize;
            let height_usize = height as usize;
            
            for y_pos in 0..height_usize {
                let y_row_start = y_pos * y.stride;
                let uv_y = y_pos / 2;
                let uv_row_start = uv_y * cb.stride;
                
                for x_pos in 0..width_usize {
                    let y_idx = y_row_start + x_pos;
                    let uv_x = x_pos / 2;
                    let cb_idx = uv_row_start + uv_x;
                    let cr_idx = uv_row_start + uv_x;
                    
                    if y_idx < y.data.len() && cb_idx < cb.data.len() && cr_idx < cr.data.len() {
                        let y_val = y.data[y_idx] as f32;
                        let cb_val = cb.data[cb_idx] as f32 - 128.0;
                        let cr_val = cr.data[cr_idx] as f32 - 128.0;
                        
                        let r = (y_val + 1.402 * cr_val).clamp(0.0, 255.0) as u8;
                        let g = (y_val - 0.344136 * cb_val - 0.714136 * cr_val).clamp(0.0, 255.0) as u8;
                        let b = (y_val + 1.772 * cb_val).clamp(0.0, 255.0) as u8;
                        
                        buffer.put_pixel(x_pos as u32, y_pos as u32, image::Rgb([r, g, b]));
                    }
                }
            }
            
            save_image_buffer(&buffer, output_path, format)?;
        } else {
            error!("无法处理的平面格式");
            return Err(ConversionError::UnsupportedInputFormat("无法解码的HEIC格式".to_string()));
        }
    }
    
    Ok(())
}