export const LOCALES = ["en", "zh-Hans", "zh-Hant"] as const;
export type Locale = (typeof LOCALES)[number];

type Entry = Record<Locale, string>;

/**
 * 消息目录：每条目强制携带三种语言（英文为源串，中文为对照翻译），
 * 缺失翻译在类型层面不可表达。
 */
export const catalog = {
  "settings.title": {
    en: "Conversion Settings",
    "zh-Hans": "转换设置",
    "zh-Hant": "轉換設定",
  },
  "settings.language": {
    en: "Language",
    "zh-Hans": "语言",
    "zh-Hant": "語言",
  },
  "settings.followSystem": {
    en: "Follow system",
    "zh-Hans": "跟随系统",
    "zh-Hant": "跟隨系統",
  },
  "filelist.tabs.pending": {
    en: "Tasks ({count})",
    "zh-Hans": "任务列表 ({count})",
    "zh-Hant": "任務列表 ({count})",
  },
  "filelist.tabs.completed": {
    en: "Completed ({count})",
    "zh-Hans": "已完成 ({count})",
    "zh-Hant": "已完成 ({count})",
  },
  "filelist.tabs.error": {
    en: "Failed ({count})",
    "zh-Hans": "转换失败 ({count})",
    "zh-Hant": "轉換失敗 ({count})",
  },
  "filelist.clearList": {
    en: "Clear all",
    "zh-Hans": "清空列表",
    "zh-Hant": "清空列表",
  },
  "filelist.clearErrors": {
    en: "Clear failed",
    "zh-Hans": "清空失败",
    "zh-Hant": "清空失敗",
  },
  "filelist.clearCompleted": {
    en: "Clear completed",
    "zh-Hans": "清空已完成",
    "zh-Hant": "清空已完成",
  },
  "filelist.dragTitle": {
    en: "Drop HEIC files here",
    "zh-Hans": "拖拽 HEIC 文件到此处",
    "zh-Hant": "拖拽 HEIC 檔案到此處",
  },
  "filelist.dragHint": {
    en: "Supports .heic, .heif formats",
    "zh-Hans": "支持 .heic, .heif 格式",
    "zh-Hant": "支援 .heic, .heif 格式",
  },
  "filelist.emptyCompletedTitle": {
    en: "No completed files yet",
    "zh-Hans": "暂无已完成的文件",
    "zh-Hant": "暫無已完成的檔案",
  },
  "filelist.emptyCompletedHint": {
    en: "Converted files will appear here",
    "zh-Hans": "转换完成的文件将显示在这里",
    "zh-Hant": "轉換完成的檔案將顯示在這裡",
  },
  "filelist.emptyErrorTitle": {
    en: "No failed files",
    "zh-Hans": "暂无转换失败的文件",
    "zh-Hant": "暫無轉換失敗的檔案",
  },
  "filelist.emptyErrorHint": {
    en: "Failed conversions will appear here",
    "zh-Hans": "转换失败的文件将显示在这里",
    "zh-Hant": "轉換失敗的檔案將顯示在這裡",
  },
  "filelist.selectFiles": {
    en: "Select files",
    "zh-Hans": "选择文件",
    "zh-Hant": "選擇檔案",
  },
  "filelist.dialogFilterName": {
    en: "HEIC/HEIF files",
    "zh-Hans": "HEIC/HEIF 文件",
    "zh-Hant": "HEIC/HEIF 檔案",
  },
  "filelist.selectFailed": {
    en: "Failed to select files: {error}",
    "zh-Hans": "选择文件失败：{error}",
    "zh-Hant": "選擇檔案失敗：{error}",
  },
  "card.badgeProcessing": {
    en: "Processing",
    "zh-Hans": "处理中",
    "zh-Hant": "處理中",
  },
  "card.badgeWaiting": {
    en: "Waiting",
    "zh-Hans": "等待",
    "zh-Hant": "等待",
  },
  "card.badgeFailed": {
    en: "Failed",
    "zh-Hans": "失败",
    "zh-Hant": "失敗",
  },
  "card.openDir": {
    en: "Open the folder containing the converted file",
    "zh-Hans": "打开转换成功的文件所在目录",
    "zh-Hant": "開啟轉換成功的檔案所在目錄",
  },
  "card.removeCompleted": {
    en: "Remove from completed list",
    "zh-Hans": "从已完成列表中移除",
    "zh-Hant": "從已完成列表中移除",
  },
  "card.removeError": {
    en: "Remove from failed list",
    "zh-Hans": "从失败列表中移除",
    "zh-Hant": "從失敗列表中移除",
  },
  "card.removeTask": {
    en: "Remove from task list",
    "zh-Hans": "从任务列表中移除",
    "zh-Hant": "從任務列表中移除",
  },
  "card.loading": {
    en: "Loading...",
    "zh-Hans": "加载中...",
    "zh-Hant": "載入中...",
  },
  "settings.targetFormat": {
    en: "Target format",
    "zh-Hans": "目标格式",
    "zh-Hant": "目標格式",
  },
  "settings.currentFormat": {
    en: "Current: {label}",
    "zh-Hans": "当前选择：{label}",
    "zh-Hant": "目前選擇：{label}",
  },
  "settings.quality": {
    en: "Image quality",
    "zh-Hans": "图片质量",
    "zh-Hant": "圖片品質",
  },
  "settings.qualityTooltipHigh": {
    en: "Higher quality increases file size. Applies to JPEG and WebP only.",
    "zh-Hans": "更高的质量将导致文件体积变大。此选项仅对 JPEG 和 WebP 格式有效。",
    "zh-Hant": "更高的品質將導致檔案體積變大。此選項僅對 JPEG 和 WebP 格式有效。",
  },
  "settings.qualityTooltipUnsupported": {
    en: "The current format does not support quality adjustment.",
    "zh-Hans": "当前格式不支持质量调整。",
    "zh-Hant": "目前格式不支援品質調整。",
  },
  "settings.more": {
    en: "More",
    "zh-Hans": "更多",
    "zh-Hant": "更多",
  },
  "settings.outputFolder": {
    en: "Output folder",
    "zh-Hans": "输出目录",
    "zh-Hant": "輸出資料夾",
  },
  "settings.selectOutputFolder": {
    en: "Select output folder",
    "zh-Hans": "选择输出目录",
    "zh-Hant": "選擇輸出資料夾",
  },
  "settings.outputFolderHint": {
    en: "If not selected, files are saved to the system Downloads folder.",
    "zh-Hans": "未选择时，将保存到系统的“下载”文件夹。",
    "zh-Hant": "未選擇時，將儲存到系統的「下載」資料夾。",
  },
  "settings.stopConverting": {
    en: "Stop conversion",
    "zh-Hans": "停止转换",
    "zh-Hant": "停止轉換",
  },
  "settings.stopping": {
    en: "Stopping...",
    "zh-Hans": "正在停止...",
    "zh-Hant": "正在停止...",
  },
  "settings.preparing": {
    en: "Preparing...",
    "zh-Hans": "正在准备...",
    "zh-Hant": "正在準備...",
  },
  "settings.startBatch": {
    en: "Start batch conversion",
    "zh-Hans": "开始批量转换",
    "zh-Hant": "開始批次轉換",
  },
  "settings.continueBatch": {
    en: "Resume conversion ({count} pending)",
    "zh-Hans": "继续转换 ({count} 待处理)",
    "zh-Hant": "繼續轉換 ({count} 待處理)",
  },
  "titlebar.maximize": {
    en: "Maximize/Restore",
    "zh-Hans": "最大化/还原",
    "zh-Hant": "最大化/還原",
  },
  "titlebar.minimize": {
    en: "Minimize",
    "zh-Hans": "最小化",
    "zh-Hant": "最小化",
  },
  "titlebar.pin": {
    en: "Pin window",
    "zh-Hans": "置顶窗口",
    "zh-Hant": "置頂視窗",
  },
  "titlebar.close": {
    en: "Close",
    "zh-Hans": "关闭",
    "zh-Hant": "關閉",
  },
  "titlebar.toggleTheme": {
    en: "Toggle theme",
    "zh-Hans": "切换主题",
    "zh-Hant": "切換主題",
  },
  "titlebar.minimizeToTray": {
    en: "Minimize to tray",
    "zh-Hans": "最小化到托盘",
    "zh-Hant": "最小化至系統匣",
  },
  "app.name": {
    en: "HEIC Image Converter",
    "zh-Hans": "HEIC 图片格式转换器",
    "zh-Hant": "HEIC 圖片轉換器",
  },
  "tray.showWindow": {
    en: "Show window",
    "zh-Hans": "显示窗口",
    "zh-Hant": "顯示視窗",
  },
  "tray.exit": {
    en: "Exit",
    "zh-Hans": "退出",
    "zh-Hant": "結束",
  },
  "confirm.exitTitle": {
    en: "Confirm exit",
    "zh-Hans": "确认退出",
    "zh-Hant": "確認結束",
  },
  "confirm.exitBody": {
    en: "A conversion is in progress. Stop it and exit?",
    "zh-Hans": "转换正在进行中，确认要停止转换并退出吗？",
    "zh-Hant": "轉換正在進行中，確認要停止轉換並結束嗎？",
  },
  "dialog.chooseActionTitle": {
    en: "Choose an action",
    "zh-Hans": "请选择您的操作",
    "zh-Hant": "請選擇您的操作",
  },
  "dialog.convertingBody": {
    en: "A conversion is in progress. What would you like to do?",
    "zh-Hans": "转换正在进行中，您希望如何处理？",
    "zh-Hant": "轉換正在進行中，您希望如何處理？",
  },
  "dialog.hideToTray": {
    en: "Minimize to system tray",
    "zh-Hans": "最小化到系统托盘",
    "zh-Hant": "最小化至系統匣",
  },
  "dialog.quit": {
    en: "Quit",
    "zh-Hans": "退出",
    "zh-Hant": "結束",
  },
  "dialog.closeTitle": {
    en: "Confirm close",
    "zh-Hans": "确认关闭",
    "zh-Hant": "確認關閉",
  },
  "dialog.pendingCloseBody": {
    en: "There are still files waiting to be processed. Close the window?",
    "zh-Hans": "任务队列中仍有文件未处理，确认要关闭窗口吗？",
    "zh-Hant": "任務佇列中仍有檔案未處理，確認要關閉視窗嗎？",
  },
  "dialog.cancel": {
    en: "Cancel",
    "zh-Hans": "取消",
    "zh-Hant": "取消",
  },
  "dialog.close": {
    en: "Close",
    "zh-Hans": "关闭",
    "zh-Hant": "關閉",
  },
  "dialog.confirm": {
    en: "OK",
    "zh-Hans": "确定",
    "zh-Hant": "確定",
  },
  "error.dialogTitle": {
    en: "Error",
    "zh-Hans": "发生错误",
    "zh-Hant": "發生錯誤",
  },
  "status.totalTime": {
    en: "Total time:",
    "zh-Hans": "总耗时:",
    "zh-Hant": "總耗時:",
  },
  "store.convertFailed": {
    en: "Failed to start the conversion: {error}",
    "zh-Hans": "转换任务执行失败！{error}",
    "zh-Hant": "轉換任務執行失敗！{error}",
  },
  "store.stopFailed": {
    en: "Failed to stop the conversion: {error}",
    "zh-Hans": "停止转换任务失败！{error}",
    "zh-Hant": "停止轉換任務失敗！{error}",
  },
} satisfies Record<string, Entry>;

export type MessageKey = keyof typeof catalog;

export function buildMessages(): Record<Locale, Record<string, string>> {
  const messages = Object.fromEntries(
    LOCALES.map((locale) => [locale, {} as Record<string, string>])
  ) as Record<Locale, Record<string, string>>;
  for (const [key, entry] of Object.entries(catalog)) {
    for (const locale of LOCALES) {
      messages[locale][key] = entry[locale];
    }
  }
  return messages;
}
