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
  const settings = ref<ConverterSettings>({ format: "jpeg", quality: [90] });
  const isConverting = ref(false);
  const outputFolder = ref<string | null>(null);
  const isReadyForConversion = ref(false);
  const spendTime = ref<number | null>(null);
  // --- Getters ---
  const stats = computed(() => ({
    total: files.length,
    done: files.filter((f) => f.status === "done").length,
    pending: files.filter((f) => f.status === "pending").length,
    error: files.filter((f) => f.status === "error").length,
  }));

  // --- Actions ---
  const addPaths = async (paths: string[]) => {
    if (!paths || paths.length === 0) return;
    await info(`前端addPaths获取到: ${paths}`);
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
    const file = files.find((f) => f.path === path);
    if (file) {
      file.status = "done";
      file.progress = 100;
      file.convertedFilePath = output_path;
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
    if (isConverting.value) return;

    const pending = files.filter((f) => f.status === "pending");

    if (pending.length === 0) {
      void warn(`收到转换请求，但没有待转换的文件，这可能是一个前端状态同步错误。`);
      return;
    }

    const paths = pending.map((f) => f.path);

    isConverting.value = true;
    try {
      await invoke("convert_images", {
        paths: paths,
        targetType: settings.value.format,
        outputFolder: outputFolder.value,
      });
      await info(`已发起转换任务`);
    } catch (error) {
      alertSevere("转换任务执行失败！" + error)
    }
  };

  return {
    files,
    settings,
    isConverting,
    isReadyForConversion,
    outputFolder,
    spendTime,
    stats,
    addPaths,
    updateFileStatus,
    updateFileSuccess,
    updateFileError,
    removePath,
    clearPaths,
    updateSettings,
    setOutputFolder,
    startConversion,
  };
});
