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
  // 为了模块化，虽然现在不用 path，但结构里保留
  // sourcePath?: string 
  outputPath?: string
}

// --- 转换设置接口 ---
export interface ConverterSettings {
  format: 'jpeg' | 'png'
  quality: number[]
}