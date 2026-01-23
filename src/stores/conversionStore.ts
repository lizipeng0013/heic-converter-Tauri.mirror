import { defineStore } from "pinia";
import { ref, reactive, computed } from "vue";
import type {FileItem, ConverterSettings} from "@/types";
import { debug, warn } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import {stat} from "@tauri-apps/plugin-fs";
import { alertSevere} from "@/utils/useError"

export const useConversionStore = defineStore('conversion', () => {
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

  // 独立计数器，避免每次访问 stats 都执行 filter 操作
  const pendingCount = ref(0);
  const convertingCount = ref(0);
  const errorCount = ref(0);

  // --- Getters ---
  const stats = computed(() => ({
    total: files.length + completedFiles.length, // 总计包括待转换和已完成的文件
    done: completedFiles.length, // 已完成的文件数量
    pending: pendingCount.value, // 待转换的文件数量（直接读取计数器）
    converting: convertingCount.value, // 正在转换的文件数量（直接读取计数器）
    error: errorCount.value, // 失败的文件数量（直接读取计数器）
    completed: completedFiles.length, // 已完成的文件数量（与 done 相同）
  }));

  // --- 辅助函数：更新计数器 ---
  const updateCounters = (oldStatus: string, newStatus: string) => {
    // 减少旧状态的计数
    if (oldStatus === "pending") pendingCount.value--;
    if (oldStatus === "converting") convertingCount.value--;
    if (oldStatus === "error") errorCount.value--;

    // 增加新状态的计数
    if (newStatus === "pending") pendingCount.value++;
    if (newStatus === "converting") convertingCount.value++;
    if (newStatus === "error") errorCount.value++;
  };

  // --- Actions ---
  const addPaths = async (paths: string[]) => {
    if (!paths || paths.length === 0) return;
    debug(`前端addPaths获取到: ${paths}`);

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
          pendingCount.value++; // 更新计数器
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
      const oldStatus = file.status;
      file.status = status as any;
      file.progress = progress;
      updateCounters(oldStatus, status); // 更新计数器
    }
  };

  const updateFileSuccess = (path: string, output_path: string)=> {
    const fileIndex = files.findIndex((f) => f.path === path);
    if (fileIndex !== -1) {
      const file = files[fileIndex];
      const oldStatus = file.status;
      file.status = "done";
      file.progress = 100;
      file.convertedFilePath = output_path;

      updateCounters(oldStatus, "done"); // 更新计数器

      // 从待转换列表移除，添加到已完成列表
      files.splice(fileIndex, 1);
      completedFiles.push(file);
    }
  }

  const updateFileError = (path: string, error: string)=> {
    const file = files.find((f) => f.path === path);
    if (file) {
      const oldStatus = file.status;
      file.status = "error";
      file.progress = 0;
      file.error = error;
      updateCounters(oldStatus, "error"); // 更新计数器
    }
  }

  const removePath = (path: string) => {
    // 从 reactive 数组中删除元素
    // 方法 1: splice
    const index = files.findIndex((f) => f.path === path);
    if (index !== -1) {
      const file = files[index];
      const oldStatus = file.status;
      files.splice(index, 1);

      // 更新计数器
      if (oldStatus === "pending") pendingCount.value--;
      if (oldStatus === "converting") convertingCount.value--;
      if (oldStatus === "error") errorCount.value--;
    }
  };

  const clearPaths = () => {
    // 重置为空数组
    // files.length = 0 也可以
    files.splice(0, files.length);

    // 重置计数器
    pendingCount.value = 0;
    convertingCount.value = 0;
    errorCount.value = 0;

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
    debug(`前端开始执行转换逻辑...`);

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

    // 立即将所有 pending 状态的文件更新为 converting 状态
    // 这样用户点击开始按钮后，所有文件会立即显示为"正在转换"状态
    pending.forEach((file) => {
      file.status = "converting";
      file.progress = 0;
    });

    // 更新计数器
    pendingCount.value = 0;
    convertingCount.value += pending.length;
    debug(`已将 ${pending.length} 个文件状态更新为 converting`);

    try {
      await invoke("convert_images", {
        paths: paths,
        targetType: settings.value.format,
        outputFolder: outputFolder.value,
        quality: settings.value.quality[0],
      });
      debug(`已发起转换任务`);
    } catch (error) {
      alertSevere("转换任务执行失败！" + error)
      isConverting.value = false;
      isStopping.value = false;
      // 如果调用失败，将文件状态重置回 pending
      pending.forEach((file) => {
        file.status = "pending";
        file.progress = 0;
      });

      // 重置计数器
      convertingCount.value = 0;
      pendingCount.value += pending.length;
    }
  };

  // 停止转换
  const stopConversion = async () => {
    debug(`前端停止转换...`)
    try {
      // 设置停止标志，防止在停止过程中启动新的转换任务
      isStopping.value = true;
      
      // 不立即更新 isConverting，保持为 true，让按钮显示"正在停止"
      // 不立即重置文件状态，等待后端正在转换的线程完成
      
      await invoke("stop_conversion");
      debug(`已发送停止转换请求，等待后端正在转换的线程完成`);
    } catch (error) {
      alertSevere("停止转换任务失败！" + error)
      isStopping.value = false;
    }
  };

  // 处理停止完成事件（由后端通知）
  const handleStopped = () => {
    debug(`收到停止完成通知，重置剩余文件状态`);

    // 将所有 converting 状态的文件重置为 pending 状态
    const convertingFiles = files.filter((f) => f.status === "converting");
    convertingFiles.forEach((file) => {
      file.status = "pending";
      file.progress = 0;
    });

    // 更新计数器
    convertingCount.value = 0;
    pendingCount.value += convertingFiles.length;
    debug(`已将 ${convertingFiles.length} 个文件状态从 converting 重置为 pending`);

    // 重置转换状态
    isConverting.value = false;
    isStopping.value = false;
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
    handleStopped,
  };
});
