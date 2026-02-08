use std::path::Path;
use tauri_plugin_log::log::debug;

/// 构建目标文件路径
///
/// # 参数
/// - `source_path`: 源文件路径
/// - `target_type`: 目标文件类型（如 "jpg", "png"）
/// - `output_folder`: 输出文件夹路径
///
/// # 返回
/// 完整的目标文件路径字符串
pub fn build_target_path(source_path: &str, target_type: &str, output_folder: &str) -> String {
    let path = Path::new(source_path);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let file_name = format!("{}.{}", stem, target_type);
    let target_path = Path::new(output_folder).join(file_name);
    target_path.to_string_lossy().into_owned()
}

/// 验证输出目录是否有写入权限
///
/// # 参数
/// - `folder`: 输出文件夹路径
///
/// # 返回
/// - `Ok(())`: 目录有效且有写入权限
/// - `Err(String)`: 错误信息
pub fn validate_output_folder(folder: &str) -> Result<(), String> {
    let p = Path::new(folder);

    // 检查路径是否为空
    if folder.is_empty() {
        return Err("输出目录不能为空".to_string());
    }

    // 检查目录是否存在，不存在则创建
    if !p.exists() {
        debug!("输出目录不存在，尝试创建: {}", folder);
        std::fs::create_dir_all(p).map_err(|e| format!("无法创建输出目录 '{}': {}", folder, e))?;
    }

    // 检查是否是目录
    if !p.is_dir() {
        return Err(format!("'{}' 不是有效的目录", folder));
    }

    // 检查是否有写入权限（通过创建测试文件验证）
    let test_file = p.join(".heic_converter_write_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            // 清理测试文件
            if let Err(e) = std::fs::remove_file(&test_file) {
                debug!("警告：无法清理测试文件 '{}': {}", test_file.display(), e);
            }
            debug!("输出目录写入权限验证通过: {}", folder);
            Ok(())
        },
        Err(e) => Err(format!("输出目录 '{}' 无写入权限: {}", folder, e)),
    }
}
