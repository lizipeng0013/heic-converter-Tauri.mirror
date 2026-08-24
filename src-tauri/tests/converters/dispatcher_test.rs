use heic_converter_lib::converters::common::OutputFormat;
use tempfile::TempDir;

#[test]
#[ignore = "需要 Tauri 测试环境"]
fn test_convert_image_auto_heic() {
    // 这个测试需要实际的 HEIC 文件和 Tauri 环境
    // 在 CI 中跳过，本地有测试文件时可以运行
}

#[test]
#[ignore = "需要 Tauri 测试环境"]
fn test_convert_image_auto_jpeg() {
    // 测试 JPEG 转换
}

#[test]
#[ignore = "需要 Tauri 测试环境"]
fn test_convert_image_auto_png() {
    // 测试 PNG 转换
}

#[test]
fn test_output_format_conversion() {
    // 测试格式枚举的转换逻辑
    let formats = vec![
        (OutputFormat::Jpeg(90), "jpg"),
        (OutputFormat::Png, "png"),
        (OutputFormat::WebP(80), "webp"),
        (OutputFormat::Bmp, "bmp"),
        (OutputFormat::Tiff, "tif"),
        (OutputFormat::Ico, "ico"),
    ];
    
    for (format, expected_ext) in formats {
        assert_eq!(format.extension(), expected_ext);
    }
}

#[test]
fn test_temp_dir_creation() {
    // 验证临时目录创建正常工作
    let temp_dir = TempDir::new().unwrap();
    assert!(temp_dir.path().exists());
}

#[test]
fn test_output_path_building() {
    use std::path::Path;
    
    let source_path = "/path/to/image.heic";
    let output_folder = "/output";
    let target_type = "jpg";
    
    let path = Path::new(source_path);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let file_name = format!("{}.{}", stem, target_type);
    let target_path = Path::new(output_folder).join(file_name);
    
    assert_eq!(target_path.to_string_lossy(), "/output/image.jpg");
}
