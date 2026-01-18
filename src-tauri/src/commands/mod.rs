use serde_json::json;
use tauri::{AppHandle, Emitter};
use crate::build_target_path;
use tauri_plugin_log::log::{error, info};

// 声明子模块
pub mod window;
pub mod converter;  // 新的统一转换器模块

pub async fn batch_convert(
    app: &AppHandle,
    file_pairs: Vec<(String, String)>, // 前端传递 [(输入1,输出1), (输入2,输出2), ...]
    format: String,
) -> Result<(), String> {
    info!("检查输出格式是否符合要求");

    let output_format = match converter::common::OutputFormat::from_str(&format) {
        Ok(fmt) => fmt,
        Err(e) => {
            error!("不支持该输出格式：{} {}", &format, e);
            // 如果格式错误，所有文件都返回同样的错误
            return Err(e.to_string());
        }
    };
    info!("开始调用批量转换格式 converter::dispatcher::batch_convert_images");
    converter::dispatcher::batch_convert_images(&app, file_pairs, output_format)
}

#[tauri::command]
pub async fn convert_images(app: AppHandle, paths: Vec<String>, target_type: String, output_folder: String) -> Result<(), String> {
    info!("开始转换，共 {} 个文件", paths.len());
    let start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

    let app_clone = app.clone();
    let paths_clone = paths.clone();
    let format_clone = target_type.clone();

    // 【关键】开启后台任务，不阻塞主响应
    // 这行代码运行完，函数就立即返回了，前端拿到结果后不会卡住
    tauri::async_runtime::spawn(async move {
        let mut file_pairs: Vec<(String, String)> = Vec::new();
        for path in paths_clone {
            info!("当前处理: {}", path);
            let target_path = build_target_path(path.as_str(), &format_clone, &output_folder);
            file_pairs.push((path, target_path));
        }
        info!("构造了输入输出");
        let _ = batch_convert(&app_clone, file_pairs, format_clone).await.expect("批量转换出现严重错误");
        let end_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let spend_time = end_time - start_time;
        let _ = app_clone.emit("conversion-batch-finished", json!({
            "spend_time": spend_time
        }));
    });
    Ok(())

}