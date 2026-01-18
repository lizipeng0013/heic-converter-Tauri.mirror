use thiserror::Error;
use image::{ImageFormat, ImageBuffer, RgbImage};
use tauri_plugin_log::log::{debug, info, error};

/// 统一的错误类型，覆盖所有转换场景
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("HEIF解码失败: {0}")]
    HeifError(#[from] libheif_rs::HeifError),
    #[error("图片处理失败: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("文件操作失败: {0}")]
    IoError(#[from] std::io::Error),
    #[error("不支持的输出格式: {0}")]
    UnsupportedFormat(String),
    #[error("无法识别的图片格式")]
    UnknownFormat,
    #[error("不支持的输入格式: {0}")]
    UnsupportedInputFormat(String),
}

/// 统一支持的输出格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Jpeg(u8), // 包含质量参数 (1-100)
    Png,
}

impl OutputFormat {
    /// 从字符串解析输出格式
    pub fn from_str(format: &str) -> Result<Self, ConversionError> {
        match format.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(OutputFormat::Jpeg(85)), // 默认质量85
            "png" => Ok(OutputFormat::Png),
            _ => Err(ConversionError::UnsupportedFormat(format.to_string())),
        }
    }
    
    /// 转换为 image crate 的 ImageFormat
    pub fn to_image_format(&self) -> ImageFormat {
        match self {
            OutputFormat::Jpeg(_) => ImageFormat::Jpeg,
            OutputFormat::Png => ImageFormat::Png,
        }
    }
    
    /// 获取文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Jpeg(_) => "jpg",
            OutputFormat::Png => "png",
        }
    }
}

/// 保存图片的公共函数
pub fn save_image_buffer(
    buffer: &RgbImage,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    debug!("保存图片到: {}, 格式: {:?}", output_path, format);
    
    // 如果是JPEG且有质量参数
    if let OutputFormat::Jpeg(quality) = format {
        // 使用 image crate 的 JPEG 编码器以支持质量参数
        let mut file = std::fs::File::create(output_path)?;
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality);
        encoder.encode_image(buffer)?;
    } else {
        // PNG和其他格式
        buffer.save_with_format(output_path, format.to_image_format())?;
    }
    
    info!("图片保存成功: {}", output_path);
    Ok(())
}