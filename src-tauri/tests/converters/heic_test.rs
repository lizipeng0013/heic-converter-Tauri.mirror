use heic_converter_lib::converters::heic::is_heic_format;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_is_heic_format_by_extension() {
    // 测试 HEIC 扩展名
    assert!(is_heic_format("/path/to/image.heic"));
    assert!(is_heic_format("/path/to/image.HEIC"));
    
    // 测试 HEIF 扩展名
    assert!(is_heic_format("/path/to/image.heif"));
    assert!(is_heic_format("/path/to/image.HEIF"));
    
    // 测试其他格式
    assert!(!is_heic_format("/path/to/image.jpg"));
    assert!(!is_heic_format("/path/to/image.png"));
    assert!(!is_heic_format("/path/to/image.jpeg"));
}

#[test]
fn test_is_heic_format_empty_path() {
    assert!(!is_heic_format(""));
}

#[test]
fn test_is_heic_format_no_extension() {
    assert!(!is_heic_format("/path/to/image"));
}

#[test]
fn test_is_heic_format_with_dots_in_name() {
    assert!(is_heic_format("/path/to/my.image.heic"));
    assert!(!is_heic_format("/path/to/my.image.jpg"));
}

#[test]
fn test_is_heic_format_nonexistent_file() {
    // 不存在的文件：扩展名检查通过，但文件不存在
    // is_heic_format 会先检查扩展名，再尝试用 libheif 打开
    // 由于扩展名是 .heic，会返回 true（扩展名匹配）
    // 这是预期行为：扩展名匹配即认为是 HEIC 文件
    assert!(is_heic_format("/nonexistent/path/image.heic"));
}

#[test]
fn test_is_heic_format_temp_heic_file() {
    // 创建一个假的 HEIC 文件（仅包含扩展名验证）
    let temp_dir = TempDir::new().unwrap();
    let heic_path = temp_dir.path().join("test.heic");
    
    // 创建空文件
    File::create(&heic_path).unwrap();
    
    // 扩展名匹配，返回 true
    // 注意：由于文件是空的，libheif 无法打开，但扩展名检查已经通过
    assert!(is_heic_format(heic_path.to_str().unwrap()));
}

#[test]
fn test_is_heic_format_jpg_file() {
    let temp_dir = TempDir::new().unwrap();
    let jpg_path = temp_dir.path().join("test.jpg");
    
    // 创建一个简单的 JPEG 文件头
    let mut file = File::create(&jpg_path).unwrap();
    // 写入一个无效但看起来像 JPEG 的数据
    file.write_all(&[0xFF, 0xD8, 0xFF, 0xE0]).unwrap();
    
    // 应该返回 false（扩展名不是 HEIC）
    assert!(!is_heic_format(jpg_path.to_str().unwrap()));
}

#[test]
fn test_is_heic_format_case_insensitive() {
    // 测试大小写不敏感
    assert!(is_heic_format("image.heic"));
    assert!(is_heic_format("image.HEIC"));
    assert!(is_heic_format("image.Heic"));
    assert!(is_heic_format("image.hEiC"));
}

#[test]
fn test_is_heic_format_windows_paths() {
    // 测试 Windows 风格路径（如果适用）
    #[cfg(target_os = "windows")]
    {
        assert!(is_heic_format("C:\\images\\photo.heic"));
        assert!(!is_heic_format("C:\\images\\photo.jpg"));
    }
    
    // 测试路径中的反斜杠和正斜杠
    assert!(is_heic_format("path/to/image.heic"));
    assert!(!is_heic_format("path/to/image.jpg"));
}
