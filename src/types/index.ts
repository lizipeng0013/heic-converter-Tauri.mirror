// --- 文件状态 ---
export type FileStatus = 'idle' | 'converting' | 'done' | 'error'

// --- 文件项接口 ---
export interface FileItem {
  id: string
  name: string
  size: number
  status: FileStatus
  progress: number
  fileObject?: File
  // 预留源文件路径，未使用
  sourceFilePath: string
  /**
   * 转换完成的文件路径
   */
  convertedFilePath: string
}

// --- 转换设置接口 ---
export interface ConverterSettings {
  format: 'jpeg' | 'png'
  quality: number[]
}