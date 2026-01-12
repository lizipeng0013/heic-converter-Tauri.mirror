import type { FileItem, ConverterSettings } from '@/types'
import heic2any from 'heic2any'
import { writeFile } from '@tauri-apps/plugin-fs'
import { downloadDir } from '@tauri-apps/api/path'

export class ConversionService {
  
  /**
   * 通用工具：将 Blob 转为 ArrayBuffer (高性能版)
   */
  private static async blobToArrayBuffer(blob: Blob): Promise<Uint8Array> {
    const targetBlob = blob
    if ('arrayBuffer' in targetBlob) {
      try {
        // 获取 ArrayBuffer
        const arrayBuffer = await blob.arrayBuffer();
        return new Uint8Array(arrayBuffer);
      } catch (e) {
        console.warn('原生 Blob.arrayBuffer 失败，使用 FileReader 回退', e)
      }
    }
    return new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = () => {
        if (reader.result instanceof ArrayBuffer) {
          resolve(new Uint8Array(reader.result))
        } else {
          reject(new Error('FileReader 读取失败，结果不是 ArrayBuffer'))
        }
      }
      reader.onerror = () => reject(new Error('FileReader 读取出错'))
      reader.readAsArrayBuffer(targetBlob)
    })
  }

  /**
   * 批量转换文件
   */
  static async convertBatch(
    files: FileItem[], 
    settings: ConverterSettings, 
    outputFolder: string | null
  ): Promise<void> {
    
    if (files.length === 0) return;

    // 1. 确定目标目录
    let targetDir = outputFolder
    if (!targetDir) {
      try {
        targetDir = await downloadDir()
      } catch (e) {
        throw new Error("无法获取下载目录！")
      }
    }

    // 2. 准备转换参数
    const targetFormat = settings.format === 'jpeg' ? 'jpeg' : settings.format
    const quality = settings.quality[0] / 100
    const ext = settings.format === 'jpeg' ? 'jpg' : 'png'

    // 3. 遍历每个文件进行转换
    for (const file of files) {
      file.status = 'converting'
      file.progress = 0
      try {
        // --- A. 单文件调用 heic2any ---
        // 传入单个 file.fileObject
        // multiple: true (确保多帧图片返回 Blob 数组，单帧返回长度为 1 的数组)
        const result = await heic2any({
          blob: file.fileObject!,
          toType: targetFormat,
          quality: quality,
          multiple: true // 显式设置为 true，统一处理返回值为 Blob[]
        }) as Blob[] // 既然我们显式要了 multiple: true，就当做数组处理

        // --- 处理多帧 ---
        file.progress = 20
        for (let i = 0; i < result.length; i++) {
          const blob = result[i]
          const uint8Array = await this.blobToArrayBuffer(blob)

          // 构造文件名 (处理多帧情况：文件名_0.jpg, 文件名_1.jpg)
          const nameParts = file.name.split('.')
          const baseName = nameParts.slice(0, -1).join('.')
          const fileName = result.length > 1 ? `${baseName}_${i}.${ext}` : `${baseName}.${ext}`
          // 构造路径
          const targetPath = `${targetDir}/${fileName}`
          file.convertedFilePath = targetPath
          // 写入
          await writeFile(targetPath, uint8Array)
          file.progress = 20 + 60/result.length*(i+1)
        }
      } catch (error) {
        console.error(`文件 ${file.name} 处理失败:`, error)
        file.status = 'error'
      }
      // --- 更新状态 ---
        // 完成状态
        file.status = 'done'
        file.progress = 100
    }
  }
}