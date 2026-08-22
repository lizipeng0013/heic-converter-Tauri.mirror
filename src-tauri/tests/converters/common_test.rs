use heic_converter_lib::converters::common::{OutputFormat, ConversionError, save_image_buffer};
use image::{ImageBuffer, RgbImage};
use tempfile::TempDir;

#[test]
fn test_output_format_from_str() {
    // 测试 JPEG 格式解析
    let format = OutputFormat::from_str("jpg", 90).unwrap();
    assert!(matches!(format, OutputFormat::Jpeg(90)));
    
    let format = OutputFormat::from_str("jpeg", 80).unwrap();
    assert!(matches!(format, OutputFormat::Jpeg(80)));
    
    // 测试 PNG 格式解析
    let format = OutputFormat::from_str("png", 0).unwrap();
    assert!(matches!(format, OutputFormat::Png));
    
    // 测试 WebP 格式解析
    let format = OutputFormat::from_str("webp", 85).unwrap();
    assert!(matches!(format, OutputFormat::WebP(85)));
    
    // 测试其他格式
    assert!(matches!(OutputFormat::from_str("bmp", 0).unwrap(), OutputFormat::Bmp));
    assert!(matches!(OutputFormat::from_str("tiff", 0).unwrap(), OutputFormat::Tiff));
    assert!(matches!(OutputFormat::from_str("ico", 0).unwrap(), OutputFormat::Ico));
    
    // 测试无效格式
    let result = OutputFormat::from_str("invalid", 0);
    assert!(result.is_err());
}

#[test]
fn test_output_format_extension() {
    assert_eq!(OutputFormat::Jpeg(90).extension(), "jpg");
    assert_eq!(OutputFormat::Png.extension(), "png");
    assert_eq!(OutputFormat::WebP(80).extension(), "webp");
    assert_eq!(OutputFormat::Bmp.extension(), "bmp");
    assert_eq!(OutputFormat::Tiff.extension(), "tif");
    assert_eq!(OutputFormat::Ico.extension(), "ico");
}

#[test]
fn test_output_format_name() {
    assert_eq!(OutputFormat::Jpeg(90).name(), "JPEG");
    assert_eq!(OutputFormat::Png.name(), "PNG");
    assert_eq!(OutputFormat::WebP(80).name(), "WebP");
    assert_eq!(OutputFormat::Bmp.name(), "BMP");
    assert_eq!(OutputFormat::Tiff.name(), "TIFF");
    assert_eq!(OutputFormat::Ico.name(), "ICO");
}

#[test]
fn test_output_format_supports_quality() {
    assert!(OutputFormat::Jpeg(90).supports_quality());
    assert!(OutputFormat::WebP(80).supports_quality());
    assert!(!OutputFormat::Png.supports_quality());
    assert!(!OutputFormat::Bmp.supports_quality());
    assert!(!OutputFormat::Tiff.supports_quality());
    assert!(!OutputFormat::Ico.supports_quality());
}

#[test]
fn test_save_image_buffer_jpeg() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.jpg");
    
    // 创建一个 10x10 的 RGB 图像
    let buffer: RgbImage = ImageBuffer::new(10, 10);
    
    // 保存为 JPEG
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Jpeg(90));
    assert!(result.is_ok());
    
    // 验证文件存在
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_png() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.png");
    
    let buffer: RgbImage = ImageBuffer::new(10, 10);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Png);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_webp() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.webp");
    
    let buffer: RgbImage = ImageBuffer::new(10, 10);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::WebP(80));
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_bmp() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.bmp");
    
    let buffer: RgbImage = ImageBuffer::new(10, 10);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Bmp);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_tiff() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.tiff");
    
    let buffer: RgbImage = ImageBuffer::new(10, 10);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Tiff);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_ico_small() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.ico");
    
    // 创建小尺寸图像（ICO 允许范围内）
    let buffer: RgbImage = ImageBuffer::new(64, 64);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Ico);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_save_image_buffer_ico_large() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_large.ico");
    
    // 创建大尺寸图像（超过 ICO 允许的 256 像素）
    let buffer: RgbImage = ImageBuffer::new(512, 512);
    
    let result = save_image_buffer(&buffer, output_path.to_str().unwrap(), OutputFormat::Ico);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_conversion_error_user_message() {
    let heif_error = libheif_rs::HeifError {
        code: libheif_rs::HeifErrorCode::InvalidInput,
        sub_code: libheif_rs::HeifErrorSubCode::Unspecified,
        message: "test error".to_string(),
    };
    let conversion_error = ConversionError::HeifError(heif_error);
    assert!(conversion_error.user_message().contains("HEIC"));
    
    let image_error = ConversionError::ImageError(image::ImageError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    )));
    assert!(image_error.user_message().contains("图片处理失败"));
    
    let io_error = ConversionError::IoError(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "permission denied",
    ));
    assert!(io_error.user_message().contains("文件读写失败"));
    
    let unsupported_format = ConversionError::UnsupportedFormat("xyz".to_string());
    assert!(unsupported_format.user_message().contains("不支持的输出格式"));
}

#[test]
fn test_jpeg_quality_range() {
    let temp_dir = TempDir::new().unwrap();
    
    // 测试低质量
    let output_path_low = temp_dir.path().join("low_quality.jpg");
    let buffer: RgbImage = ImageBuffer::new(100, 100);
    let result = save_image_buffer(&buffer, output_path_low.to_str().unwrap(), OutputFormat::Jpeg(10));
    assert!(result.is_ok());
    
    // 测试高质量
    let output_path_high = temp_dir.path().join("high_quality.jpg");
    let result = save_image_buffer(&buffer, output_path_high.to_str().unwrap(), OutputFormat::Jpeg(100));
    assert!(result.is_ok());
    
    // 验证文件大小差异（高质量应该更大）
    let low_size = std::fs::metadata(output_path_low).unwrap().len();
    let high_size = std::fs::metadata(output_path_high).unwrap().len();
    assert!(high_size > low_size);
}
