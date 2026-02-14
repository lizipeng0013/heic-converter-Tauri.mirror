// --- 文件项接口 ---
export interface FileItem {
  /**
   * 源文件路径
   */
  path: string;
  name: string;
  size: number;
  /**
   * 转换完成的文件路径
   */
  convertedFilePath?: string;
  /**
   * 转换错误信息
   */
  errorMessage?: string;
}

// --- 支持的输出格式类型 ---
export type OutputFormat = "jpeg" | "jpg" | "png" | "webp" | "bmp" | "tiff" | "ico";

// --- 格式信息接口 ---
export interface FormatInfo {
  value: OutputFormat;
  label: string;
  supportsQuality: boolean;
}

// --- 转换设置接口 ---
export interface ConverterSettings {
  format: OutputFormat;
  quality: number[];
}

// --- 关闭窗口操作类型 ---
export type CloseAction = "hideToTray" | "exit" | "cancel";

// --- Tauri 事件类型定义 ---

/**
 * 转换更新事件
 */
export type ConversionUpdateEvent =
  | {
      path: string;
      status: "done";
      output_path: string;
    }
  | {
      path: string;
      status: "error";
      errorMessage: string;
    };

/**
 * 转换失败事件
 */
export interface ConversionFailedEvent {
  errorMessage: string;
}
