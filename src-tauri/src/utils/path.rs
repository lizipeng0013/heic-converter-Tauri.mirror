use std::path::Path;

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
    let stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");
    let file_name = format!("{}.{}", stem, target_type);
    let target_path = Path::new(output_folder).join(file_name);
    target_path.to_string_lossy().into_owned()
}