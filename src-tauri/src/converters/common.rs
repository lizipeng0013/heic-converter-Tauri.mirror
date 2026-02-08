use image::{ImageFormat, RgbImage};
use tauri_plugin_log::log::{debug, trace};
use thiserror::Error;
use turbojpeg::{Compressor, Image, PixelFormat, Subsamp};

/// 统一的错误类型，覆盖所有转换场景
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("HEIF解码失败: {0}")]
    HeifError(#[from] libheif_rs::HeifError),
    #[error("图片处理失败: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("文件操作失败: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JPEG编码失败: {0}")]
    JpegEncodeError(String),
    #[error("不支持的输出格式: {0}")]
    UnsupportedFormat(String),
    #[error("无法识别的图片格式")]
    #[allow(dead_code)]
    UnknownFormat,
    #[error("不支持的输入格式: {0}")]
    UnsupportedInputFormat(String),
}

impl ConversionError {
    /// 获取用户友好的错误信息
    pub fn user_message(&self) -> String {
        match self {
            ConversionError::HeifError(_) => "HEIC文件解码失败，请确保文件没有损坏".to_string(),
            ConversionError::ImageError(_) => "图片处理失败，请检查文件格式是否正确".to_string(),
            ConversionError::IoError(_) => "文件读写失败，请检查文件权限和磁盘空间".to_string(),
            ConversionError::JpegEncodeError(_) => "JPEG编码失败，请尝试降低图片质量".to_string(),
            ConversionError::UnsupportedFormat(format) => format!("不支持的输出格式: {}，请选择 jpg、png、webp、bmp、tiff 或 ico", format),
            ConversionError::UnknownFormat => "无法识别图片格式".to_string(),
            ConversionError::UnsupportedInputFormat(format) => format!("不支持的输入格式: {}，仅支持 HEIC/HEIF 格式", format),
        }
    }
}

/// 统一支持的输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Jpeg(u8), // 包含质量参数 (1-100)
    Png,
    WebP(u8), // 包含质量参数 (1-100)
    Bmp,
    Tiff,
    Ico,
}

impl OutputFormat {
    /// 从字符串解析输出格式
    /// # 参数
    /// - `format`: 格式字符串（"jpg", "jpeg", "png", "webp", "bmp", "tiff", "ico"）
    /// - `quality`: 质量参数（1-100），仅对 JPEG 和 WebP 格式有效
    pub fn from_str(format: &str, quality: u8) -> Result<Self, ConversionError> {
        match format.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(OutputFormat::Jpeg(quality)),
            "png" => Ok(OutputFormat::Png),
            "webp" => Ok(OutputFormat::WebP(quality)),
            "bmp" => Ok(OutputFormat::Bmp),
            "tiff" | "tif" => Ok(OutputFormat::Tiff),
            "ico" => Ok(OutputFormat::Ico),
            _ => Err(ConversionError::UnsupportedFormat(format.to_string())),
        }
    }

    /// 转换为 image crate 的 ImageFormat
    pub fn as_image_format(&self) -> ImageFormat {
        match self {
            OutputFormat::Jpeg(_) => ImageFormat::Jpeg,
            OutputFormat::Png => ImageFormat::Png,
            OutputFormat::WebP(_) => ImageFormat::WebP,
            OutputFormat::Bmp => ImageFormat::Bmp,
            OutputFormat::Tiff => ImageFormat::Tiff,
            OutputFormat::Ico => ImageFormat::Ico,
        }
    }

    /// 获取格式的扩展名
    #[allow(dead_code)]
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Jpeg(_) => "jpg",
            OutputFormat::Png => "png",
            OutputFormat::WebP(_) => "webp",
            OutputFormat::Bmp => "bmp",
            OutputFormat::Tiff => "tif",
            OutputFormat::Ico => "ico",
        }
    }

    /// 获取格式的名称
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            OutputFormat::Jpeg(_) => "JPEG",
            OutputFormat::Png => "PNG",
            OutputFormat::WebP(_) => "WebP",
            OutputFormat::Bmp => "BMP",
            OutputFormat::Tiff => "TIFF",
            OutputFormat::Ico => "ICO",
        }
    }

    /// 检查格式是否支持质量参数
    #[allow(dead_code)]
    pub fn supports_quality(&self) -> bool {
        matches!(self, OutputFormat::Jpeg(_) | OutputFormat::WebP(_))
    }
}

/// 保存图片的公共函数（优化版本）
pub fn save_image_buffer(
    buffer: &RgbImage,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    debug!("保存图片到: {}, 格式: {:?}", output_path, format);

    // ICO 格式需要特殊处理：尺寸必须在 1-256 之间
    let buffer_ref: &RgbImage = if let OutputFormat::Ico = format {
        let (width, height) = buffer.dimensions();
        debug!("原始图片尺寸: {}x{}", width, height);

        // 如果图片尺寸超过256，需要缩放
        if width > 256 || height > 256 {
            debug!("ICO格式需要缩放，原始尺寸: {}x{}", width, height);

            // 计算缩放比例，保持宽高比
            let scale = if width >= height {
                256.0 / width as f32
            } else {
                256.0 / height as f32
            };

            let new_width = (width as f32 * scale).round() as u32;
            let new_height = (height as f32 * scale).round() as u32;

            debug!("缩放后尺寸: {}x{}", new_width, new_height);

            // 使用 image crate 的缩放功能
            return {
                let resized = image::imageops::resize(
                    buffer,
                    new_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                );

                // 保存缩放后的图片
                resized.save_with_format(output_path, ImageFormat::Ico)?;
                debug!("图片保存成功: {}", output_path);
                Ok(())
            };
        }

        // ICO 格式但尺寸符合要求，使用原 buffer
        buffer
    } else {
        // 非 ICO 格式，使用原 buffer
        buffer
    };

    // 如果是JPEG且有质量参数 - 使用 turbojpeg 进行高性能编码
    if let OutputFormat::Jpeg(quality) = format {
        trace!("使用 turbojpeg 进行 JPEG 编码，质量: {}", quality);

        let (width, height) = buffer_ref.dimensions();
        let data = buffer_ref.as_raw();

        // 创建 turbojpeg 压缩器
        let mut compressor = Compressor::new()
            .map_err(|e| ConversionError::JpegEncodeError(format!("创建压缩器失败: {}", e)))?;

        // 设置压缩质量（需要转换为 i32）
        compressor
            .set_quality(quality as i32)
            .map_err(|e| ConversionError::JpegEncodeError(format!("设置压缩质量失败: {}", e)))?;

        // 设置子采样模式（高质量，无色度子采样）
        compressor
            .set_subsamp(Subsamp::None)
            .map_err(|e| ConversionError::JpegEncodeError(format!("设置子采样模式失败: {}", e)))?;

        // 创建 turbojpeg 图像结构
        let image = Image {
            pixels: data.as_slice(),
            width: width as usize,
            pitch: 3 * width as usize, // RGB 格式，每行 3*width 字节
            height: height as usize,
            format: PixelFormat::RGB,
        };

        // 压缩为 JPEG 数据
        let jpeg_data = compressor
            .compress_to_vec(image.as_deref())
            .map_err(|e| ConversionError::JpegEncodeError(format!("压缩失败: {}", e)))?;

        // 写入文件
        std::fs::write(output_path, jpeg_data)
            .map_err(|e| ConversionError::JpegEncodeError(format!("写入文件失败: {}", e)))?;
    } else {
        // PNG、WebP、BMP、TIFF、ICO 等其他格式
        buffer_ref.save_with_format(output_path, format.as_image_format())?;
    }

    debug!("图片保存成功: {}", output_path);
    Ok(())
}
