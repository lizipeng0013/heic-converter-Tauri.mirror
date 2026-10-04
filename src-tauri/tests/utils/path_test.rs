use heic_converter_lib::utils::path::{
    build_target_path, plan_target_paths, reserve_unique_output_path, validate_input_paths,
    validate_output_folder, MAX_BATCH_FILES,
};
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

#[test]
fn test_plan_target_paths_numbers_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.path();
    std::fs::write(out.join("photo.jpg"), "keep").unwrap();

    let sources = vec!["/in/photo.heic".to_string()];
    let pairs = plan_target_paths(&sources, "jpg", out.to_str().unwrap());

    assert_eq!(
        pairs[0].1,
        out.join("photo (1).jpg").to_string_lossy().into_owned()
    );
    // 已存在的文件内容不被修改
    assert_eq!(
        std::fs::read_to_string(out.join("photo.jpg")).unwrap(),
        "keep"
    );
}

#[test]
fn test_plan_target_paths_dedup_within_batch() {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.path();

    let sources = vec!["/a/x.heic".to_string(), "/b/x.heic".to_string()];
    let pairs = plan_target_paths(&sources, "jpg", out.to_str().unwrap());

    assert_eq!(pairs[0].1, out.join("x.jpg").to_string_lossy().into_owned());
    assert_eq!(
        pairs[1].1,
        out.join("x (1).jpg").to_string_lossy().into_owned()
    );
    assert_ne!(pairs[0].1, pairs[1].1);
}

#[cfg(unix)]
#[test]
fn test_plan_target_paths_dangling_symlink_is_taken() {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.path();
    std::os::unix::fs::symlink("nonexistent-target", out.join("photo.jpg")).unwrap();

    let sources = vec!["/in/photo.heic".to_string()];
    let pairs = plan_target_paths(&sources, "jpg", out.to_str().unwrap());

    assert_eq!(
        pairs[0].1,
        out.join("photo (1).jpg").to_string_lossy().into_owned()
    );
    // 符号链接本身保持不变
    assert!(out.join("photo.jpg").symlink_metadata().is_ok());
}

#[test]
fn test_reserve_unique_output_path_free_path() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("a.jpg");

    let reserved =
        reserve_unique_output_path(target.to_str().unwrap()).expect("空闲路径应预留成功");

    assert_eq!(reserved, target.to_string_lossy().into_owned());
    assert!(target.exists());
}

#[test]
fn test_reserve_unique_output_path_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("photo.jpg");
    std::fs::write(&target, "keep").unwrap();

    let reserved =
        reserve_unique_output_path(target.to_str().unwrap()).expect("已存在时应返回加序号路径");

    let reserved_path = std::path::Path::new(&reserved);
    assert_eq!(reserved_path.file_name().unwrap(), "photo (1).jpg");
    assert_eq!(reserved_path.parent().unwrap(), temp_dir.path());
    // 原文件内容不变
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "keep");
}

#[cfg(unix)]
#[test]
fn test_reserve_unique_output_path_dangling_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("photo.jpg");
    std::os::unix::fs::symlink("nonexistent-target", &target).unwrap();

    let reserved =
        reserve_unique_output_path(target.to_str().unwrap()).expect("悬空符号链接应被跳过");

    let reserved_path = std::path::Path::new(&reserved);
    assert_eq!(reserved_path.file_name().unwrap(), "photo (1).jpg");
    // 符号链接仍指向原目标，未被覆盖
    assert_eq!(
        std::fs::read_link(&target).unwrap(),
        std::path::Path::new("nonexistent-target")
    );
}

#[test]
fn test_validate_output_folder_probe_conflict_left_alone() {
    let temp_dir = TempDir::new().unwrap();
    let probe = temp_dir.path().join(".heic_converter_write_test");
    std::fs::write(&probe, "userdata").unwrap();

    let result = validate_output_folder(temp_dir.path().to_str().unwrap());
    assert!(result.is_ok());
    // 探针冲突时不得覆盖或删除已有内容
    assert_eq!(std::fs::read_to_string(&probe).unwrap(), "userdata");
}

#[cfg(unix)]
#[test]
fn test_validate_output_folder_probe_symlink_not_followed() {
    let temp_dir = TempDir::new().unwrap();
    let victim = temp_dir.path().join("victim.txt");
    std::fs::write(&victim, "precious").unwrap();
    std::os::unix::fs::symlink(&victim, temp_dir.path().join(".heic_converter_write_test"))
        .unwrap();

    let result = validate_output_folder(temp_dir.path().to_str().unwrap());
    assert!(result.is_ok());
    // 被链接的文件不得被改写
    assert_eq!(std::fs::read_to_string(&victim).unwrap(), "precious");
}

#[test]
fn test_validate_input_paths_accepts_regular_heic() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("photo.heic");
    std::fs::write(&file, "x").unwrap();

    let paths = vec![file.to_string_lossy().into_owned()];
    assert!(validate_input_paths(&paths).is_ok());
}

#[test]
fn test_validate_input_paths_rejects_missing_file() {
    let paths = vec!["/nonexistent/photo.heic".to_string()];
    let err = validate_input_paths(&paths).unwrap_err();
    assert!(err.contains("无效"));
    // 错误信息不泄露输入路径
    assert!(!err.contains("nonexistent"));
}

#[test]
fn test_validate_input_paths_rejects_directory() {
    let temp_dir = TempDir::new().unwrap();
    let paths = vec![temp_dir.path().to_string_lossy().into_owned()];
    assert!(validate_input_paths(&paths).is_err());
}

#[test]
fn test_validate_input_paths_rejects_wrong_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("photo.jpg");
    std::fs::write(&file, "x").unwrap();

    let paths = vec![file.to_string_lossy().into_owned()];
    let err = validate_input_paths(&paths).unwrap_err();
    assert!(err.contains("不支持"));
}

#[test]
fn test_validate_input_paths_rejects_too_many() {
    // 数量上限先于存在性检查
    let paths: Vec<String> = (0..MAX_BATCH_FILES + 1)
        .map(|i| format!("/x/{}.heic", i))
        .collect();
    let err = validate_input_paths(&paths).unwrap_err();
    assert!(err.contains("最多"));
}

#[cfg(unix)]
#[test]
fn test_validate_input_paths_rejects_fifo() {
    let temp_dir = TempDir::new().unwrap();
    let fifo = temp_dir.path().join("pipe.heic");
    let status = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo 应可用");
    assert!(status.success());

    let paths = vec![fifo.to_string_lossy().into_owned()];
    let err = validate_input_paths(&paths).unwrap_err();
    assert!(err.contains("无效"));
}
