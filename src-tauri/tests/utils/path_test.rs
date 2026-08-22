use heic_converter_lib::utils::path::{build_target_path, validate_output_folder};
use tempfile::TempDir;

#[test]
fn test_build_target_path() {
    let source_path = "/home/user/images/photo.heic";
    let output_folder = "/home/user/output";
    let target_type = "jpg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/home/user/output/photo.jpg");
}

#[test]
fn test_build_target_path_with_jpeg() {
    let source_path = "/photos/vacation.HEIC";
    let output_folder = "/converted";
    let target_type = "jpeg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/converted/vacation.jpeg");
}

#[test]
fn test_build_target_path_with_png() {
    let source_path = "/photos/image.heif";
    let output_folder = "/output";
    let target_type = "png";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/output/image.png");
}

#[test]
fn test_build_target_path_with_dots_in_name() {
    let source_path = "/path/to/my.photo.heic";
    let output_folder = "/output";
    let target_type = "jpg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/output/my.photo.jpg");
}

#[test]
fn test_build_target_path_no_extension() {
    let source_path = "/path/to/image";
    let output_folder = "/output";
    let target_type = "jpg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/output/image.jpg");
}

#[test]
fn test_build_target_path_empty_output_folder() {
    let source_path = "/path/to/image.heic";
    let output_folder = "";
    let target_type = "jpg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    // 空输出文件夹会创建相对路径
    assert!(result.contains("image.jpg"));
}

#[test]
fn test_validate_output_folder_exists() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_string_lossy();
    
    let result = validate_output_folder(&path);
    assert!(result.is_ok());
}

#[test]
fn test_validate_output_folder_not_exists() {
    let temp_dir = TempDir::new().unwrap();
    let new_path = temp_dir.path().join("subdir");
    
    let result = validate_output_folder(new_path.to_string_lossy().as_ref());
    assert!(result.is_ok());
    assert!(new_path.exists());
}

#[test]
fn test_validate_output_folder_empty() {
    let result = validate_output_folder("");
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("不能为空"));
}

#[test]
fn test_validate_output_folder_file_not_dir() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    std::fs::write(&file_path, "test").unwrap();
    
    let result = validate_output_folder(file_path.to_string_lossy().as_ref());
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("不是有效的目录"));
}

#[test]
fn test_validate_output_folder_with_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let path_with_space = temp_dir.path().join("my folder");
    
    let result = validate_output_folder(path_with_space.to_string_lossy().as_ref());
    assert!(result.is_ok());
    assert!(path_with_space.exists());
}

#[test]
fn test_build_target_path_windows_style() {
    #[cfg(target_os = "windows")]
    {
        let source_path = "C:\\Users\\test\\image.heic";
        let output_folder = "D:\\output";
        let target_type = "png";
        
        let result = build_target_path(source_path, target_type, output_folder);
        assert!(result.contains("image.png"));
    }
}

#[test]
fn test_build_target_path_multiple_extensions() {
    // 测试带有多个点的文件名
    let source_path = "/path/to/my.photo.heic";
    let output_folder = "/output";
    let target_type = "jpg";
    
    let result = build_target_path(source_path, target_type, output_folder);
    assert_eq!(result, "/output/my.photo.jpg");
}

#[test]
fn test_validate_output_folder_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let unicode_path = temp_dir.path().join("测试文件夹");
    
    let result = validate_output_folder(unicode_path.to_string_lossy().as_ref());
    assert!(result.is_ok());
}
