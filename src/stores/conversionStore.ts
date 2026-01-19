import { defineStore } from "pinia";
import { ref, reactive, computed } from "vue";
import type {FileItem, ConverterSettings} from "@/types";
import { info, debug, warn } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import {stat} from "@tauri-apps/plugin-fs";
import { alertSevere} from "@/utils/useError"

export const useConversionStore = defineStore("conversion", () => {
  void debug("useConversionStore");
  // --- State ---
  const files = reactive<FileItem[]>([]);
  const completedFiles = reactive<FileItem[]>([]); // 已完成的文件
  const activeTab = ref<'pending' | 'completed'>('pending'); // 当前激活的标签页
  const settings = ref<ConverterSettings>({ format: "jpeg", quality: [90] });
  const isConverting = ref(false);
  const isStopping = ref(false); // 标记是否正在停止转换
  const hasStartedConversion = ref(false); // 标记是否曾经开始过转换
  const outputFolder = ref<string | null>(null);
  const isReadyForConversion = ref(false);
  const spendTime = ref<number | null>(null);
  // --- Getters ---
  const stats = computed(() => ({
    total: files.length + completedFiles.length, // 总计包括待转换和已完成的文件
    done: completedFiles.length, // 已完成的文件数量
    pending: files.filter((f) => f.status === "pending").length, // 待转换的文件数量
    converting: files.filter((f) => f.status === "converting").length, // 正在转换的文件数量
    error: files.filter((f) => f.status === "error").length, // 失败的文件数量
    completed: completedFiles.length, // 已完成的文件数量（与 done 相同）
  }));

  // --- Actions ---
  const addPaths = async (paths: string[]) => {
    if (!paths || paths.length === 0) return;
    await info(`前端addPaths获取到: ${paths}`);
    
    // 如果待转换列表为空，重置 hasStartedConversion 标志
    // 这样新导入的文件会显示"开始批量转换"而不是"继续转换"
    if (files.length === 0) {
      hasStartedConversion.value = false;
    }
    
    const filePromises = paths.map(async (path) => {
      const ext = path.split(".").pop()?.toLowerCase();
      if (ext !== "heic" && ext !== "heif") {
        await warn(`跳过非HEIC/HEIF文件： ${path}`);
        return null;
      }
      try {
        const meta = await stat(path);
        const nameParts = path.split(/[/\\]/);
        const name = nameParts[nameParts.length - 1];
        return {
          path: path,
          name: name,
          size: meta.size,
          status: "pending" as const,
          progress: 0,
        };
      } catch (error) {
        alertSevere("无法获取文件大小！" + path + error);
        return null;
      }
    });
    const results = await Promise.all(filePromises);
    
    // 过滤掉无效的文件并添加到列表
    results.forEach((file) => {
      if (file) {
        const isDuplicate = files.some(f => f.path === file.path);
        if (!isDuplicate) {
          files.push(file);
        } else {
          warn(`检测到重复导入文件，已跳过：${file.path}`);
        }
      }
    });
    isReadyForConversion.value = true;
  };

  const updateFileStatus = (
    path: string,
    status: string,
    progress: number
  ) => {
    const file = files.find((f) => f.path === path);
    if (file) {
      file.status = status as any;
      file.progress = progress;
    }
  };

  const updateFileSuccess = (path: string, output_path: string)=> {
    const fileIndex = files.findIndex((f) => f.path === path);
    if (fileIndex !== -1) {
      const file = files[fileIndex];
      file.status = "done";
      file.progress = 100;
      file.convertedFilePath = output_path;
      
      // 从待转换列表移除，添加到已完成列表
      files.splice(fileIndex, 1);
      completedFiles.push(file);
    }
  }

  const updateFileError = (path: string, error: string)=> {
    const file = files.find((f) => f.path === path);
    if (file) {
      file.status = "error";
      file.progress = 0;
      file.error = error;
    }
  }

  const removePath = (path: string) => {
    // 从 reactive 数组中删除元素
    // 方法 1: splice
    const index = files.findIndex((f) => f.path === path);
    if (index !== -1) {
      files.splice(index, 1);
    }
  };

  const clearPaths = () => {
    // 重置为空数组
    // files.length = 0 也可以
    files.splice(0, files.length);
    // 清零耗时
    spendTime.value = null;
    // 重置转换状态
    isReadyForConversion.value = false;
    isConverting.value = false;
    isStopping.value = false;
    hasStartedConversion.value = false;
  };

  const clearCompletedFiles = () => {
    // 只清空已完成文件
    completedFiles.splice(0, completedFiles.length);
  };

  const removeCompletedFile = (path: string) => {
    // 从已完成列表中删除指定文件
    const index = completedFiles.findIndex((f) => f.path === path);
    if (index !== -1) {
      completedFiles.splice(index, 1);
    }
  };

  const setActiveTab = (tab: 'pending' | 'completed') => {
    activeTab.value = tab;
  };

  const updateSettings = (newSettings: Partial<ConverterSettings>) => {
    settings.value = { ...settings.value, ...newSettings };
  };

  const setOutputFolder = (path: string | null) => {
    outputFolder.value = path;
  };

  // --- 重写开始转换逻辑：调用批量方法 ---
  const startConversion = async () => {
    await info(`前端开始执行转换逻辑...`)
    
    // 如果正在转换或正在停止，不允许启动新的转换任务
    if (isConverting.value || isStopping.value) {
      void warn(`转换任务正在进行中或正在停止，忽略新的转换请求`);
      return;
    }

    const pending = files.filter((f) => f.status === "pending");

    if (pending.length === 0) {
      void warn(`收到转换请求，但没有待转换的文件，这可能是一个前端状态同步错误。`);
      return;
    }

    const paths = pending.map((f) => f.path);

    isConverting.value = true;
    isStopping.value = false;
    hasStartedConversion.value = true;
    try {
      await invoke("convert_images", {
        paths: paths,
        targetType: settings.value.format,
        outputFolder: outputFolder.value,
        quality: settings.value.quality[0],
      });
      await info(`已发起转换任务`);
    } catch (error) {
      alertSevere("转换任务执行失败！" + error)
      isConverting.value = false;
      isStopping.value = false;
    }
  };

  // 停止转换
  const stopConversion = async () => {
    await info(`前端停止转换...`)
    try {
      // 设置停止标志，防止在停止过程中启动新的转换任务
      isStopping.value = true;
      
      // 立即更新UI状态，让用户感觉立即停止
      isConverting.value = false;
      
      // 由于转换速度很快，让正在转换的文件继续完成，只是停止后续文件的转换
      // 不再重置正在转换的文件状态
      
      await invoke("stop_conversion");
      await info(`已停止转换任务`);
      
      // 等待一段时间确保后端停止完成
      setTimeout(() => {
        isStopping.value = false;
      }, 1000);
    } catch (error) {
      alertSevere("停止转换任务失败！" + error)
      isStopping.value = false;
    }
  };

  return {
    files,
    completedFiles,
    activeTab,
    settings,
    isConverting,
    isStopping,
    hasStartedConversion,
    isReadyForConversion,
    outputFolder,
    spendTime,
    stats,
    addPaths,
    updateFileStatus,
    updateFileSuccess,
    updateFileError,
    removePath,
    removeCompletedFile,
    clearPaths,
    clearCompletedFiles,
    setActiveTab,
    updateSettings,
    setOutputFolder,
    startConversion,
    stopConversion,
  };
});
