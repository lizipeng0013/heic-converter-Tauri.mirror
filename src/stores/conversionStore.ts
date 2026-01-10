import { defineStore } from 'pinia'
import { ref, reactive, computed } from 'vue'
import type { FileItem, ConverterSettings } from '@/types'
import { ConversionService } from '@/services/conversionService'

// --- 【日志 1】检测 Store 是否被正确初始化 ---
console.log("[Store] 正在初始化 conversionStore...")

export const useConversionStore = defineStore('conversion', () => {
  // --- State ---
  const files = reactive<FileItem[]>([])
  const settings = ref<ConverterSettings>({ format: 'jpeg', quality: [90] })
  const isConverting = ref(false)
  const outputFolder = ref<string | null>(null)

  // --- Getters ---
  const stats = computed(() => ({
    total: files.length,
    done: files.filter(f => f.status === 'done').length,
    pending: files.filter(f => f.status === 'idle').length
  }))

  // --- Actions ---
  
  const addFiles = (fileList: FileList) => {
    if (!fileList || fileList.length === 0) return;
    Array.from(fileList).forEach(file => {
      const ext = file.name.split('.').pop()?.toLowerCase()
      if (ext === 'heic' || ext === 'heif') {
        console.log(`文件 ${file.name} 符合，正在添加...`) // 调试 3
        files.push({
          id: Math.random().toString(36).substr(2, 9),
          name: file.name,
          size: file.size,
          status: 'idle',
          progress: 0,
          fileObject: file
        })
      }
    })
  }

  const removeFile = (id: string) => {
    // 从 reactive 数组中删除元素
    // 方法 1: splice
    const index = files.findIndex(f => f.id === id)
    if (index !== -1) {
      files.splice(index, 1)
    }
  }

  const clearFiles = () => {
    // 重置为空数组
    // files.length = 0 也可以
    files.splice(0, files.length)
  }

  const updateSettings = (newSettings: Partial<ConverterSettings>) => {
    settings.value = { ...settings.value, ...newSettings }
  }

  const setOutputFolder = (path: string | null) => {
    outputFolder.value = path
  }

  // --- 重写开始转换逻辑：调用批量方法 ---
  const startConversion = async () => {
    if (isConverting.value) return
    
    const pending = files.filter(f => f.status === 'idle')
    
    if (pending.length === 0) {
      alert("没有可转换的文件")
      return
    }

    isConverting.value = true

    try {
      // 【关键修改】：调用 Service 的批量转换方法
      // 传入的是文件数组和设置
      await ConversionService.convertBatch(pending, settings.value, outputFolder.value)
    } catch (e) {
      console.error("转换过程发生错误", e)
      alert("转换过程发生未知错误！" + (e as string))
    } finally {
      isConverting.value = false
    }
  }

  return {
    files,
    settings,
    isConverting,
    outputFolder,
    stats,
    addFiles,
    removeFile,
    clearFiles,
    updateSettings,
    setOutputFolder,
    startConversion
  }
})