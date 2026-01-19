// --- 文件状态 ---
export type FileStatus = "pending" | "converting" | "done" | "error";

// --- 文件项接口 ---
export interface FileItem {
  /**
   * 源文件路径
   */
  path: string;
  name: string;
  size: number;
  status: FileStatus;
  progress: number;
  /**
   * 转换完成的文件路径
   */
  convertedFilePath?: string;
  error?: string;
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
