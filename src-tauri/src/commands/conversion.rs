use crate::services::conversion::batch_convert;
#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::Hint;
use notify_rust::Notification;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_log::log::{debug, error, info};
use tracing::instrument;

// 全局停止标志
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

// 单飞闸：同一时间只允许一个转换批次
static BATCH_RUNNING: AtomicBool = AtomicBool::new(false);

/// 批次执行守卫：无论正常结束、出错还是提前返回，都释放单飞闸
struct BatchGuard;

impl Drop for BatchGuard {
    fn drop(&mut self) {
        BATCH_RUNNING.store(false, Ordering::SeqCst);
    }
}

// 已确认的输出目录（由设置页对话框/下载目录查询经 confirm_output_folder 写入）
static CONFIRMED_OUTPUT_FOLDER: Mutex<Option<String>> = Mutex::new(None);

/// 确认输出目录：前端在用户选定目录或加载默认下载目录时调用。
#[tauri::command]
pub fn confirm_output_folder(path: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err(crate::utils::locale::message(
            "validate.empty_output",
            crate::utils::locale::current(),
        ));
    }
    *CONFIRMED_OUTPUT_FOLDER.lock().unwrap() = Some(path);
    Ok(())
}

/// 校验转换请求使用的输出目录是否已被确认。
///
/// 未确认的任意路径不能驱动目录创建与写入；错误信息不包含路径。
pub fn ensure_output_confirmed(
    output_folder: &str,
    locale: crate::utils::locale::Locale,
) -> Result<(), String> {
    let confirmed = CONFIRMED_OUTPUT_FOLDER.lock().unwrap();
    match &*confirmed {
        Some(c) if c == output_folder => Ok(()),
        _ => Err(crate::utils::locale::message(
            "validate.unconfirmed",
            locale,
        )),
    }
}

/// 转换请求的同步参数校验（纯逻辑 + 已确认目录静态，独立成函数便于测试）。
///
/// 约定：校验错误在命令边界捕获 locale 后渲染（一次性、可并行测试）；
/// 事件/通知类文案在发送时读取 current()（保证"前端总是赢"）。
pub fn validate_convert_request(
    paths: &[String],
    target_type: &str,
    output_folder: &str,
    quality: u8,
    locale: crate::utils::locale::Locale,
) -> Result<(), String> {
    // 验证输入参数
    if paths.is_empty() {
        return Err(crate::utils::locale::message("convert.no_files", locale));
    }

    // 输入校验在 Rust 侧强制执行（前端过滤可被绕过）
    crate::utils::path::validate_input_paths(paths, locale)?;

    // 输出目录必须是事先确认过的（对话框选定或系统下载目录）
    ensure_output_confirmed(output_folder, locale)?;

    // 验证输出目录权限
    crate::utils::validate_output_folder(output_folder, locale)?;

    // 验证格式
    let valid_formats = ["jpeg", "jpg", "png", "webp", "bmp", "tiff", "ico"];
    if !valid_formats.contains(&target_type.to_lowercase().as_str()) {
        return Err(crate::utils::locale::message_fmt(
            "convert.unsupported_output",
            locale,
            &[
                ("format", target_type),
                ("supported", &valid_formats.join(", ")),
            ],
        ));
    }

    // 验证质量参数
    if quality == 0 || quality > 100 {
        return Err(crate::utils::locale::message_fmt(
            "convert.quality_range",
            locale,
            &[("quality", &quality.to_string())],
        ));
    }
    Ok(())
}

/// 批量转换图片命令
///
/// # 参数
/// - `app`: Tauri 应用句柄
/// - `paths`: 待转换文件路径列表
/// - `target_type`: 目标格式（"jpg", "png"）
/// - `output_folder`: 输出文件夹
/// - `quality`: JPEG质量（1-100），仅对JPEG格式有效
#[tauri::command]
#[instrument(skip(app), fields(
    file_count = paths.len(),
    format = %target_type,
    quality = quality
))]
pub async fn convert_images(
    app: AppHandle,
    paths: Vec<String>,
    target_type: String,
    output_folder: String,
    quality: u8,
) -> Result<(), String> {
    // 校验文案按进入时的 locale 渲染（错误在返回给用户的那一刻即命令边界）
    let locale = crate::utils::locale::current();
    validate_convert_request(&paths, &target_type, &output_folder, quality, locale)?;

    let app_clone = app.clone();
    let format_clone = target_type.clone();

    // 规划互不冲突的输出路径（已存在/同批次同名自动加序号）
    let file_pairs = crate::utils::path::plan_target_paths(&paths, &format_clone, &output_folder);

    // 单飞闸：已有批次在跑时明确拒绝，避免并发叠加 CPU/内存
    if BATCH_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Err(crate::utils::locale::message(
            "convert.batch_running",
            locale,
        ));
    }

    // 重置停止标志（必须在拿到单飞闸之后，避免清掉进行中批次的停止请求）
    SHOULD_STOP.store(false, Ordering::SeqCst);

    // 批次整体跑在阻塞线程上，不占用 async 运行时 worker；
    // BatchGuard 在批次退出时释放单飞闸
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = BatchGuard;
        info!("开始后台转换任务");

        // 处理批量转换结果
        let was_stopped = match batch_convert(&app_clone, file_pairs, format_clone, quality) {
            Ok(result) => result,
            Err(e) => {
                error!("批量转换失败: {}", e);
                // 发送错误事件到前端
                let _ = app_clone.emit(
                    "conversion-failed",
                    serde_json::json!({
                        "errorMessage": crate::utils::locale::message(
                            "convert.failed_check_logs",
                            crate::utils::locale::current()
                        )
                    }),
                );
                return; // 退出任务，不继续处理
            },
        };

        // 根据是否被停止，发送不同的完成事件
        if was_stopped {
            info!("转换任务被停止，发送停止完成事件");
            let _ = app_clone.emit("conversion-stopped", serde_json::json!({}));
        } else {
            info!("转换任务正常完成，发送批次完成事件");
            let _ = app_clone.emit("conversion-batch-finished", serde_json::json!({}));

            // 正常完成时，检查窗口状态并发送系统通知（仅当窗口最小化或隐藏时）
            if let Some(window) = app_clone.get_webview_window("main") {
                if let (Ok(is_minimized), Ok(is_visible)) =
                    (window.is_minimized(), window.is_visible())
                {
                    if is_minimized || !is_visible {
                        debug!("窗口最小化或隐藏，发送系统通知");

                        #[cfg(target_os = "windows")]
                        {
                            let _ = Notification::new()
                                .app_id("top.hotime.heic-converter")
                                .summary(crate::utils::locale::message(
                                    "notification.conversion_done_summary",
                                    crate::utils::locale::current(),
                                ))
                                .body(crate::utils::locale::message(
                                    "notification.conversion_done_body",
                                    crate::utils::locale::current(),
                                ))
                                .show();
                        }

                        #[cfg(all(unix, not(target_os = "macos")))]
                        {
                            // Linux 平台支持交互式通知：点击通知本身即可打开窗口。
                            // 等待通知点击放在独立线程：不占批次闸门，也不占运行时线程。
                            std::thread::spawn(move || {
                                let window_clone = window.clone();
                                if let Ok(handle) = Notification::new()
                                    .summary(&crate::utils::locale::message(
                                        "notification.conversion_done_summary",
                                        crate::utils::locale::current(),
                                    ))
                                    .body(&crate::utils::locale::message(
                                        "notification.conversion_done_body",
                                        crate::utils::locale::current(),
                                    ))
                                    .action("default", "") // default action 捕获通知本身的点击
                                    .hint(Hint::Resident(true))
                                    .show()
                                {
                                    handle.wait_for_action(move |action| {
                                        if action == "default" {
                                            debug!("用户点击通知，打开窗口");
                                            let _ = window_clone.show();
                                            let _ = window_clone.unminimize();
                                            let _ = window_clone.set_focus();
                                        }
                                    });
                                }
                            });
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

/// 停止转换命令
#[tauri::command]
pub fn stop_conversion() -> Result<(), String> {
    info!("收到停止转换请求");
    SHOULD_STOP.store(true, Ordering::SeqCst);
    Ok(())
}

/// 检查是否应该停止
pub fn should_stop() -> bool {
    SHOULD_STOP.load(Ordering::SeqCst)
}
