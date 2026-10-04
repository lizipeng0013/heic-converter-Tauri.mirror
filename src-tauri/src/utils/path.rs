use std::collections::HashSet;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tauri_plugin_log::log::{debug, error};

use crate::converters::common::ConversionError;
use crate::utils::locale::Locale;

/// 输出路径编号尝试上限（同名冲突自动加序号的最大次数）
const MAX_UNIQUE_ATTEMPTS: usize = 10_000;

/// 单次批量转换的输入文件数量上限
pub const MAX_BATCH_FILES: usize = 1000;

/// 允许的输入扩展名（与前端筛选一致，Rust 侧强制）
const INPUT_EXTENSIONS: [&str; 2] = ["heic", "heif"];

/// 校验一批输入路径：数量上限、必须是普通文件、扩展名白名单。
///
/// 前端过滤只是 UI 级别，这里才是真实边界；错误信息不包含路径。
/// `locale` 由调用方在入口捕获（错误文案在命令边界渲染）。
pub fn validate_input_paths(paths: &[String], locale: Locale) -> Result<(), String> {
    use crate::utils::locale::{message, message_fmt};
    if paths.len() > MAX_BATCH_FILES {
        return Err(message_fmt(
            "validate.max_batch",
            locale,
            &[("max", &MAX_BATCH_FILES.to_string())],
        ));
    }

    for path in paths {
        let p = Path::new(path);
        // metadata 跟随符号链接：指向普通文件的链接可转换，
        // 目录、FIFO、设备文件等一律拒绝（避免转换线程被特殊文件阻塞）
        match std::fs::metadata(p) {
            Ok(meta) if meta.is_file() => {},
            _ => return Err(message("validate.invalid_input", locale)),
        }

        let ext = p
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if !INPUT_EXTENSIONS.contains(&ext.as_str()) {
            return Err(message("validate.unsupported_format", locale));
        }
    }
    Ok(())
}

/// 为路径追加序号：`photo.jpg` + 1 -> `photo (1).jpg`
fn numbered_path(base: &Path, n: usize) -> PathBuf {
    if n == 0 {
        return base.to_path_buf();
    }
    let stem = base
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "image".to_string());
    let name = match base.extension() {
        Some(ext) => format!("{} ({}).{}", stem, n, ext.to_string_lossy()),
        None => format!("{} ({})", stem, n),
    };
    base.with_file_name(name)
}

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

/// 为一批源文件规划互不冲突的目标路径。
///
/// 目标已存在（包括符号链接，哪怕悬空）或在本批次内已被认领时，
/// 自动追加序号；绝不指向任何已有文件系统条目。
pub fn plan_target_paths(
    sources: &[String],
    target_type: &str,
    output_folder: &str,
) -> Vec<(String, String)> {
    let mut claimed: HashSet<String> = HashSet::new();
    let mut result = Vec::with_capacity(sources.len());

    for source in sources {
        let base = build_target_path(source, target_type, output_folder);
        let base_path = PathBuf::from(&base);
        let mut chosen = base;
        for n in 0..MAX_UNIQUE_ATTEMPTS {
            let candidate = numbered_path(&base_path, n);
            let candidate_str = candidate.to_string_lossy().into_owned();
            let taken = claimed.contains(&candidate_str) || candidate.symlink_metadata().is_ok();
            if !taken {
                chosen = candidate_str;
                break;
            }
            chosen = candidate_str;
        }
        claimed.insert(chosen.clone());
        result.push((source.clone(), chosen));
    }
    result
}

/// 以"创建即失败"方式为输出文件预留一个空闲路径。
///
/// 已存在的任何文件系统条目（含符号链接）都被视为占用，自动尝试
/// 加序号的候选路径；成功后会真实创建该空文件（由调用方写入内容）。
pub fn reserve_unique_output_path(target: &str) -> Result<String, ConversionError> {
    let base = PathBuf::from(target);
    for n in 0..MAX_UNIQUE_ATTEMPTS {
        let candidate = numbered_path(&base, n);
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(_) => return Ok(candidate.to_string_lossy().into_owned()),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(ConversionError::IoError(e)),
        }
    }
    Err(ConversionError::OutputConflict)
}

/// 日志用脱敏标签：仅保留路径最后一级名称，避免完整目录泄露到日志
fn log_label(p: &Path) -> String {
    p.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "<root>".to_string())
}

/// 验证输出目录是否有写入权限
///
/// # 参数
/// - `folder`: 输出文件夹路径
///
/// # 返回
/// - `Ok(())`: 目录有效且有写入权限
/// - `Err(String)`: 用户友好的错误信息
pub fn validate_output_folder(folder: &str, locale: Locale) -> Result<(), String> {
    use crate::utils::locale::message;
    let p = Path::new(folder);

    // 检查路径是否为空
    if folder.is_empty() {
        return Err(message("validate.empty_output", locale));
    }

    // 检查目录是否存在，不存在则创建
    if !p.exists() {
        debug!("输出目录不存在，尝试创建: {}", log_label(p));
        if let Err(e) = std::fs::create_dir_all(p) {
            error!("创建输出目录失败: {} - 错误详情: {}", log_label(p), e);
            return Err(message("validate.create_dir_failed", locale));
        }
    }

    // 检查是否是目录
    if !p.is_dir() {
        return Err(message("validate.not_a_directory", locale));
    }

    // 检查是否有写入权限（以"创建即失败"方式探测，不覆盖、不跟随任何已有条目）
    let test_file = p.join(".heic_converter_write_test");
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&test_file)
    {
        Ok(_) => {
            // 清理探测文件：仅删除我们刚创建的条目（remove_file 不跟随符号链接）
            if let Err(e) = std::fs::remove_file(&test_file) {
                debug!("警告：无法清理探测文件: {}", e);
            }
            debug!("输出目录写入权限验证通过: {}", log_label(p));
            Ok(())
        },
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            // 已有同名条目（可能是历史残留或他人预置）：不碰它，也不视为失败
            debug!("探测文件已存在，跳过写入测试: {}", log_label(p));
            Ok(())
        },
        Err(e) => {
            error!(
                "输出目录写入权限验证失败: {} - 错误详情: {}",
                log_label(p),
                e
            );
            Err(crate::utils::locale::message(
                "validate.no_write_permission",
                locale,
            ))
        },
    }
}
