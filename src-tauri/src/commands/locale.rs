use crate::utils::locale::{self, Locale};

/// 由前端下发当前应用语言（内部键：en / zh-Hans / zh-Hant）。
/// 错误文案与系统通知按此 locale 渲染；非法键直接返回错误供前端定位。
#[tauri::command]
pub fn set_locale(locale: String) -> Result<(), String> {
    match Locale::parse(&locale) {
        Some(l) => {
            locale::set_current(l);
            Ok(())
        },
        None => Err(format!("unsupported locale: {locale}")),
    }
}
