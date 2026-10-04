//! 应用语言（locale）状态与用户可见消息目录。
//!
//! 目录用三元组表结构：每条目强制同时携带三种语言，缺失翻译在类型层面不可表达；
//! 前端负责把裸系统标签（zh-CN / zh-Hant-TW / zh_CN …）归一化为内部键。

use std::sync::Mutex;

/// 内部 locale 标识（与前端目录键一致）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locale {
    En,
    ZhHans,
    ZhHant,
}

impl Locale {
    pub fn as_str(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::ZhHans => "zh-Hans",
            Locale::ZhHant => "zh-Hant",
        }
    }

    /// 解析内部键；裸系统标签不在 Rust 侧解析（归一化由前端完成）
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Locale::En),
            "zh-Hans" => Some(Locale::ZhHans),
            "zh-Hant" => Some(Locale::ZhHant),
            _ => None,
        }
    }
}

/// 消息目录条目：三种语言并列，缺一不可
pub struct Entry {
    pub key: &'static str,
    pub en: &'static str,
    pub zh_hans: &'static str,
    pub zh_hant: &'static str,
}

/// 用户可见消息目录（英文为源串，中文为对照翻译）
pub static CATALOG: &[Entry] = &[
    Entry {
        key: "notification.conversion_done_summary",
        en: "Conversion completed",
        zh_hans: "转换完成",
        zh_hant: "轉換完成",
    },
    Entry {
        key: "notification.conversion_done_body",
        en: "Images converted successfully",
        zh_hans: "图片转换已完成",
        zh_hant: "圖片轉換已完成",
    },
    Entry {
        key: "error.heif_decode_failed",
        en: "HEIC file could not be decoded; make sure it is not corrupted",
        zh_hans: "HEIC文件解码失败，请确保文件没有损坏",
        zh_hant: "HEIC檔案解碼失敗，請確保檔案沒有損壞",
    },
    Entry {
        key: "error.image_process_failed",
        en: "Image processing failed; check the file format",
        zh_hans: "图片处理失败，请检查文件格式是否正确",
        zh_hant: "圖片處理失敗，請檢查檔案格式是否正確",
    },
    Entry {
        key: "error.io_failed",
        en: "File I/O failed; check permissions and disk space",
        zh_hans: "文件读写失败，请检查文件权限和磁盘空间",
        zh_hant: "檔案讀寫失敗，請檢查檔案權限與磁碟空間",
    },
    Entry {
        key: "error.jpeg_encode_failed",
        en: "JPEG encoding failed; try lowering the quality",
        zh_hans: "JPEG编码失败，请尝试降低图片质量",
        zh_hant: "JPEG 編碼失敗，請嘗試降低圖片品質",
    },
    Entry {
        key: "error.unsupported_output_format",
        en: "Unsupported output format: {format}; choose jpg, png, webp, bmp, tiff or ico",
        zh_hans: "不支持的输出格式: {format}，请选择 jpg、png、webp、bmp、tiff 或 ico",
        zh_hant: "不支援的輸出格式: {format}，請選擇 jpg、png、webp、bmp、tiff 或 ico",
    },
    Entry {
        key: "error.unknown_format",
        en: "Unrecognized image format",
        zh_hans: "无法识别图片格式",
        zh_hant: "無法識別圖片格式",
    },
    Entry {
        key: "error.output_conflict",
        en: "Output conflict: too many files with the same name; change the output folder or rename the source",
        zh_hans: "输出文件冲突：同名文件过多，请更换输出目录或源文件名",
        zh_hant: "輸出檔案衝突：同名檔案過多，請更換輸出目錄或重新命名來源檔案",
    },
    Entry {
        key: "error.dimensions_too_large",
        en: "Image dimensions exceed the limit",
        zh_hans: "图片尺寸超出限制，无法处理",
        zh_hant: "圖片尺寸超出限制，無法處理",
    },
    Entry {
        key: "error.file_too_large",
        en: "File exceeds the size limit",
        zh_hans: "文件过大，超出处理上限",
        zh_hant: "檔案過大，超出處理上限",
    },
    Entry {
        key: "validate.max_batch",
        en: "At most {max} files per conversion",
        zh_hans: "一次最多转换 {max} 个文件",
        zh_hant: "一次最多轉換 {max} 個檔案",
    },
    Entry {
        key: "validate.invalid_input",
        en: "Selected files include invalid ones; only regular HEIC/HEIF files are supported",
        zh_hans: "所选文件中包含无效文件：仅支持普通的 HEIC/HEIF 文件",
        zh_hant: "所選檔案中包含無效檔案：僅支援一般的 HEIC/HEIF 檔案",
    },
    Entry {
        key: "validate.unsupported_format",
        en: "Selected files include unsupported formats; only HEIC/HEIF is supported",
        zh_hans: "所选文件中包含不支持的格式，仅支持 HEIC/HEIF 文件",
        zh_hant: "所選檔案中包含不支援的格式，僅支援 HEIC/HEIF 檔案",
    },
    Entry {
        key: "validate.empty_output",
        en: "Output folder cannot be empty",
        zh_hans: "输出目录不能为空",
        zh_hant: "輸出資料夾不能為空",
    },
    Entry {
        key: "validate.create_dir_failed",
        en: "Cannot create the output folder; check that the path is valid",
        zh_hans: "无法创建输出目录，请检查路径是否有效",
        zh_hant: "無法建立輸出目錄，請檢查路徑是否有效",
    },
    Entry {
        key: "validate.not_a_directory",
        en: "The selected output location is not a folder",
        zh_hans: "所选输出位置不是有效的目录",
        zh_hant: "所選輸出位置不是有效的目錄",
    },
    Entry {
        key: "validate.no_write_permission",
        en: "The output folder is not writable; choose another folder or change its permissions",
        zh_hans: "输出目录无写入权限，请选择其他目录或修改文件夹权限",
        zh_hant: "輸出目錄無寫入權限，請選擇其他目錄或修改資料夾權限",
    },
    Entry {
        key: "validate.unconfirmed",
        en: "Output folder not confirmed; re-select it in settings",
        zh_hans: "输出目录未经确认，请在设置中重新选择输出目录",
        zh_hant: "輸出目錄未經確認，請在設定中重新選擇輸出目錄",
    },
    Entry {
        key: "convert.no_files",
        en: "No files selected",
        zh_hans: "没有选择任何文件",
        zh_hant: "未選擇任何檔案",
    },
    Entry {
        key: "convert.unsupported_output",
        en: "Unsupported output format: {format}. Supported formats: {supported}",
        zh_hans: "不支持的输出格式: {format}。支持的格式: {supported}",
        zh_hant: "不支援的輸出格式: {format}。支援的格式: {supported}",
    },
    Entry {
        key: "convert.quality_range",
        en: "Quality must be between 1-100, current value: {quality}",
        zh_hans: "质量参数必须在 1-100 之间，当前值: {quality}",
        zh_hant: "品質參數必須在 1-100 之間，目前值: {quality}",
    },
    Entry {
        key: "convert.batch_running",
        en: "A conversion is already running; wait for it to finish or stop it first",
        zh_hans: "已有转换任务正在进行中，请等待完成或先停止当前任务",
        zh_hant: "已有轉換任務正在進行中，請等待完成或先停止目前任務",
    },
    Entry {
        key: "error.heic_processing_failed",
        en: "Failed to process the HEIC image data",
        zh_hans: "HEIC 图像数据处理失败",
        zh_hant: "HEIC 影像資料處理失敗",
    },
    Entry {
        key: "convert.failed_check_logs",
        en: "Conversion failed; check the logs",
        zh_hans: "转换任务执行失败，请检查日志",
        zh_hant: "轉換任務執行失敗，請檢查日誌",
    },
];

static CURRENT: Mutex<Locale> = Mutex::new(Locale::En);

/// 设置进程级 locale（由前端 set_locale 下发）
pub fn set_current(locale: Locale) {
    *CURRENT.lock().unwrap() = locale;
}

/// 当前进程级 locale（默认 en，前端挂载后立即覆盖）
pub fn current() -> Locale {
    *CURRENT.lock().unwrap()
}

/// 测试辅助：全局锁内设置 locale 执行断言，结束后恢复原值。
/// 串行化对共享 CURRENT 的观察，避免并行测试互相干扰。
/// 返回 `(断言结果, 恢复后的值)`——恢复值在锁内捕获，调用方在锁外比较也无竞争。
pub fn with_current<R>(locale: Locale, f: impl FnOnce() -> R) -> (R, Locale) {
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let previous = current();
    set_current(locale);
    let result = f();
    set_current(previous);
    (result, previous)
}

/// 按 locale 渲染消息；未知键返回键本身（便于定位缺译）
pub fn message(key: &str, locale: Locale) -> String {
    let Some(entry) = CATALOG.iter().find(|e| e.key == key) else {
        return key.to_string();
    };
    let rendered = match locale {
        Locale::En => entry.en,
        Locale::ZhHans => entry.zh_hans,
        Locale::ZhHant => entry.zh_hant,
    };
    rendered.to_string()
}

/// 渲染并做 `{name}` 占位符替换（参数缺失时保留占位符，便于定位）
pub fn message_fmt(key: &str, locale: Locale, params: &[(&str, &str)]) -> String {
    let mut out = message(key, locale);
    for (name, value) in params {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out
}
