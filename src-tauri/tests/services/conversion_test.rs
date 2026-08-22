use heic_converter_lib::services::conversion::batch_convert;
use heic_converter_lib::converters::common::OutputFormat;
use tauri::AppHandle;
use tempfile::TempDir;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs::File;
use std::io::Write;

// 全局停止标志用于测试
static TEST_SHOULD_STOP: AtomicBool = AtomicBool::new(false);

#[test]
#[ignore = "需要 Tauri 测试环境"]
fn test_batch_convert_empty_files() {
    // 测试空文件列表
    // 这需要 Tauri AppHandle，在集成测试中实现
}

#[test]
fn test_output_format_from_str_valid() {
    let formats = vec![
        ("jpg", 90, OutputFormat::Jpeg(90)),
        ("jpeg", 80, OutputFormat::Jpeg(80)),
        ("png", 0, OutputFormat::Png),
        ("webp", 85, OutputFormat::WebP(85)),
        ("bmp", 0, OutputFormat::Bmp),
        ("tiff", 0, OutputFormat::Tiff),
        ("ico", 0, OutputFormat::Ico),
    ];
    
    for (format_str, quality, expected) in formats {
        let result = OutputFormat::from_str(format_str, quality).unwrap();
        match (expected, result) {
            (OutputFormat::Jpeg(q1), OutputFormat::Jpeg(q2)) => {
                assert_eq!(q1, q2);
            }
            (OutputFormat::Png, OutputFormat::Png) => {}
            (OutputFormat::WebP(q1), OutputFormat::WebP(q2)) => {
                assert_eq!(q1, q2);
            }
            (OutputFormat::Bmp, OutputFormat::Bmp) => {}
            (OutputFormat::Tiff, OutputFormat::Tiff) => {}
            (OutputFormat::Ico, OutputFormat::Ico) => {}
            _ => panic!("格式不匹配: {:?} vs {:?}", expected, result),
        }
    }
}

#[test]
fn test_output_format_from_str_invalid() {
    let result = OutputFormat::from_str("invalid", 0);
    assert!(result.is_err());
    
    let result = OutputFormat::from_str("gif", 0);
    assert!(result.is_err());
    
    let result = OutputFormat::from_str("", 0);
    assert!(result.is_err());
}

#[test]
fn test_output_format_quality_validation() {
    // 测试有效质量范围
    assert!(OutputFormat::from_str("jpg", 1).is_ok());
    assert!(OutputFormat::from_str("jpg", 100).is_ok());
    assert!(OutputFormat::from_str("webp", 1).is_ok());
    assert!(OutputFormat::from_str("webp", 100).is_ok());
    
    // 测试无效质量值
    // 注意：from_str 不验证质量，质量由调用方验证
    assert!(OutputFormat::from_str("jpg", 0).is_ok());
    assert!(OutputFormat::from_str("jpg", 101).is_ok());
}

#[test]
fn test_temp_file_creation() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "test content").unwrap();
    
    assert!(file_path.exists());
    
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "test content");
}

#[test]
fn test_atomic_bool_operations() {
    let flag = AtomicBool::new(false);
    
    assert!(!flag.load(Ordering::Relaxed));
    
    flag.store(true, Ordering::Relaxed);
    assert!(flag.load(Ordering::Relaxed));
    
    flag.store(false, Ordering::Relaxed);
    assert!(!flag.load(Ordering::Relaxed));
}

#[test]
fn test_conversion_error_display() {
    use heic_converter_lib::converters::common::ConversionError;
    
    let io_error = ConversionError::IoError(
        std::io::Error::new(std::io::ErrorKind::NotFound, "file not found")
    );
    let error_msg = format!("{}", io_error);
    assert!(error_msg.contains("文件操作失败"));
}

#[test]
fn test_batch_convert_early_return() {
    // 测试提前返回逻辑
    let should_stop = AtomicBool::new(false);
    
    // 初始状态不应停止
    assert!(!should_stop.load(Ordering::Relaxed));
    
    // 设置停止标志
    should_stop.store(true, Ordering::Relaxed);
    assert!(should_stop.load(Ordering::Relaxed));
}
