use crate::commands::conversion::should_stop;
use crate::converters::common::OutputFormat;
use crate::converters::dispatcher::convert_image_auto;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde_json::json;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter};
use tauri_plugin_log::log::{debug, error, info, trace};

/// 批量转换多个文件（支持混合格式）
///
/// # 参数
/// - `app`: Tauri 应用句柄
/// - `files`: 文件对列表，每个元素包含 (输入路径, 输出路径)
/// - `format`: 目标输出格式（已在 commands 层验证）
/// - `quality`: JPEG质量（1-100），已在 commands 层验证
///
/// # 返回
/// 成功返回 Ok(was_stopped)，其中 was_stopped 表示是否被停止中断
/// 失败返回错误信息
pub async fn batch_convert(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: String,
    quality: u8,
) -> Result<bool, String> {
    // 参数已在 commands 层验证，直接转换
    let output_format = OutputFormat::from_str(&format, quality)
        .map_err(|e| format!("内部错误：格式解析失败: {}", e))?;

    info!(
        "开始批量转换，共 {} 个文件，目标格式: {:?}",
        files.len(),
        output_format
    );
    batch_convert_images(app, files, output_format)
}

/// 批量转换图片的核心实现（并行处理）
///
/// # 返回
/// Ok(was_stopped): was_stopped 表示是否被停止中断（true 表示被停止，false 表示正常完成）
/// Err(error): 错误信息
fn batch_convert_images(
    app: &AppHandle,
    files: Vec<(String, String)>,
    format: OutputFormat,
) -> Result<bool, String> {
    let total = files.len();
    trace!("开始并行处理 {} 个文件", total);

    // 使用原子操作替代 Mutex，减少锁竞争
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let stopped_flag = Arc::new(AtomicBool::new(false)); // 记录是否因为停止信号而中止
    let processed_count = Arc::new(AtomicUsize::new(0)); // 记录已处理的文件数
    let files_arc = Arc::new(files); // 文件列表是只读的，不需要 Mutex
    let next_index = Arc::new(AtomicUsize::new(0)); // 下一个要处理的文件索引
    let app_arc = Arc::new(app.clone()); // Arc 包装 AppHandle，避免 Clone 开销

    // 根据 CPU 核心数动态调整线程池大小
    // 少量文件时使用文件数，大量文件时保留 25% 给 UI
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let pool_size = if total < 10 {
        total // 少量文件时使用文件数
    } else {
        (num_cpus * 3 / 4).max(2) // 大量文件时保留 25% 给 UI
    };

    trace!(
        "CPU 核心数: {}, 文件数: {}, 线程池大小: {}",
        num_cpus,
        total,
        pool_size
    );

    // 创建自定义线程池，优化并发性能
    let pool = ThreadPoolBuilder::new()
        .num_threads(pool_size)
        .thread_name(|index| format!("converter-{}", index))
        .build()
        .map_err(|e| format!("创建线程池失败: {}", e))?;

    debug!("线程池创建成功，准备开始转换");

    // 发送转换开始事件，通知前端线程池已创建完成，开始处理
    let _ = app.emit("conversion-started", serde_json::json!({}));

    debug!("开始并行处理，停止标志状态: {}", should_stop());

    // 使用 par_bridge 并行处理
    pool.install(|| {
        (0..pool_size).into_par_iter().for_each(|_| {
            loop {
                // 检查停止标志
                if should_stop() {
                    stopped_flag.store(true, Ordering::Release);
                    break;
                }

                // 使用原子操作获取下一个任务索引
                let index = next_index.fetch_add(1, Ordering::Relaxed);
                if index >= total {
                    break; // 所有文件都已处理
                }

                // 再次检查停止标志（在获取索引后）
                if should_stop() {
                    stopped_flag.store(true, Ordering::Release);
                    // 直接退出，不回退索引（前端会将剩余文件重置为 pending）
                    break;
                }

                // 直接访问文件列表（只读，不需要锁）
                let (input, output) = files_arc[index].clone();

                let current = index + 1;
                trace!("处理文件 {}/{}: {}", current, total, input);

                let result = convert_image_auto(&app_arc, &input, &output, format);

                match result {
                    Ok(_) => {
                        // 使用原子操作更新计数器
                        success_count.fetch_add(1, Ordering::Relaxed);
                        processed_count.fetch_add(1, Ordering::Relaxed);
                        debug!("✓ 转换成功 {}/{}: {} -> {}", current, total, input, output);
                        let _ = app_arc.emit(
                            "conversion-update",
                            json!({
                                "path": input,
                                "status": "done",
                                "output_path": output,
                            }),
                        );
                    },
                    Err(e) => {
                        // 使用原子操作更新计数器
                        error_count.fetch_add(1, Ordering::Relaxed);
                        processed_count.fetch_add(1, Ordering::Relaxed);
                        error!("✗ 转换失败 {}/{}: {} - {}", current, total, input, e);
                        let _ = app_arc.emit(
                            "conversion-update",
                            json!({
                                "path": input,
                                "status": "error",
                                "errorMessage": e.user_message(),
                            }),
                        );
                    },
                }
            }
        });
    });

    // 显式释放线程池，确保线程被正确回收
    drop(pool);

    // 检查是否在准备阶段就被停止
    if should_stop() {
        stopped_flag.store(true, Ordering::Release);
    }

    let success = success_count.load(Ordering::Relaxed);
    let error = error_count.load(Ordering::Relaxed);
    let processed = processed_count.load(Ordering::Relaxed);
    let was_stopped = stopped_flag.load(Ordering::Relaxed);

    info!(
        "批量转换完成 - 成功: {}, 失败: {}, 已处理: {}, 总计: {}",
        success, error, processed, total
    );

    // 显式释放 Arc 引用
    drop(app_arc);
    drop(files_arc);
    drop(success_count);
    drop(error_count);
    drop(stopped_flag);
    drop(processed_count);
    drop(next_index);

    // 返回是否被停止
    Ok(was_stopped)
}
