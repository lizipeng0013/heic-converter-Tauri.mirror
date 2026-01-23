use tauri::AppHandle;
use super::common::{ConversionError, OutputFormat};
use super::heic::{is_heic_format, convert_heic_image};
use super::generic::convert_regular_image;
use tauri_plugin_log::log::{debug, trace};

/// 智能图片转换 - 自动检测格式并选择正确的转换器
pub fn convert_image_auto(
    app: &AppHandle,
    input_path: &str,
    output_path: &str,
    format: OutputFormat,
) -> Result<(), ConversionError> {
    debug!("智能转换: {} -> {}", input_path, output_path);
    
    // 检测是否为HEIC格式
    if is_heic_format(input_path) {
        trace!("检测到HEIC格式，使用HEIC转换器");
        convert_heic_image(app, input_path, output_path, format)
    } else {
        trace!("检测到普通图片格式，使用普通转换器");
        convert_regular_image(app, input_path, output_path, format)
    }
}