import { defineStore } from "pinia";
import { ref, reactive, computed } from "vue";
import type {FileItem, ConverterSettings} from "@/types";
import { debug, warn } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import {stat} from "@tauri-apps/plugin-fs";
import { alertSevere} from "@/utils/useError"

export const useConversionStore = defineStore('conversion', () => {
  // --- State ---
  const files = reactive<FileItem[]>([]); // 待转换文件（包括等待和正在转换）
  const errorFiles = reactive<FileItem[]>([]); // 转换失败的文件
  const completedFiles = reactive<FileItem[]>([]); // 已完成的文件
  const activeTab = ref<'pending' | 'completed' | 'error'>('pending'); // 当前激活的标签页
  const settings = ref<ConverterSettings>({ format: "jpeg", quality: [90] });
  const isConverting = ref(false);
  const isStopping = ref(false); // 标记是否正在停止转换
  const hasStartedConversion = ref(false); // 标记是否曾经开始过转换
  const outputFolder = ref<string | null>(null);
  const isReadyForConversion = ref(false);
  const spendTime = ref<number | null>(null);

  // 前端耗时统计
  const startTime = ref<number | null>(null); // 转换开始时间戳
  const timerId = ref<number | null>(null); // 定时器ID，用于实时更新耗时
  const accumulatedTime = ref<number>(0); // 累计已消耗的时间（毫秒），用于停止后继续转换
  const isTimerRunning = ref<boolean>(false); // 计时器是否正在运行
  const isPreparing = ref<boolean>(false); // 是否正在准备（线程池创建中）

  // 分组展开状态
  const taskExpanded = ref(true); // 任务分组是否展开
  const errorExpanded = ref(false); // 转换失败分组是否展开

  // --- Getters ---
  const stats = computed(() => ({
    total: files.length + errorFiles.length + completedFiles.length, // 总计包括所有文件
    waiting: isConverting.value || isStopping.value ? 0 : files.length, // 等待转换的文件数量
    processing: isConverting.value || isStopping.value ? files.length : 0, // 正在转换的文件数量
    completed: completedFiles.length, // 已完成的文件数量
    failed: errorFiles.length, // 失败的文件数量
  }));

  // 按集合分组的文件列表（用于 UI 显示）
  const taskFiles = computed(() => files); // 任务文件（待转换或正在转换）
  const errorFilesList = computed(() => errorFiles); // 转换失败的文件

  // --- Actions ---
  const addPaths = async (paths: string[]) => {
    if (!paths || paths.length === 0) return;
    debug(`前端addPaths获取到: ${paths}`);

    // 如果待转换列表为空，重置 hasStartedConversion 标志
    // 这样新导入的文件会显示"开始批量转换"而不是"继续转换"
    if (files.length === 0) {
      hasStartedConversion.value = false;
      // 首次导入时自动展开"任务"分组
      taskExpanded.value = true;
      errorExpanded.value = false;
    }

    // 如果待转换列表为空且已完成列表也为空，说明是全新开始，重置耗时统计
    if (files.length === 0 && completedFiles.length === 0) {
      spendTime.value = null;
      accumulatedTime.value = 0; // 重置累计时间
      debug(`全新任务开始，重置耗时统计`);
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
        };
      } catch (error) {
        alertSevere("无法获取文件大小！" + path + error);
        return null;
      }
    });
    const results = await Promise.all(filePromises);

    // 过滤掉无效的文件并去重
    const validFiles: FileItem[] = [];
    const existingPaths = new Set(files.map(f => f.path));

    results.forEach((file) => {
      if (file && !existingPaths.has(file.path)) {
        validFiles.push(file);
        existingPaths.add(file.path);
      } else if (file) {
        warn(`检测到重复导入文件，已跳过：${file.path}`);
      }
    });

    // 批量添加文件，只触发一次响应式更新
    if (validFiles.length > 0) {
      files.push(...validFiles);
    }

    isReadyForConversion.value = true;
  };

  const updateFileSuccess = (path: string, output_path: string)=> {
    const fileIndex = files.findIndex((f) => f.path === path);
    if (fileIndex !== -1) {
      const file = files[fileIndex];
      file.convertedFilePath = output_path;

      // 从待转换列表移除，添加到已完成列表
      files.splice(fileIndex, 1);
      completedFiles.push(file);
    }
  }

  const updateFileError = (path: string, error: string)=> {
    const fileIndex = files.findIndex((f) => f.path === path);
    if (fileIndex !== -1) {
      const file = files[fileIndex];
      file.error = error;

      // 从待转换列表移除，添加到错误列表
      files.splice(fileIndex, 1);
      errorFiles.push(file);
    }
  }

  const removePath = (path: string) => {
    // 从 files 中删除元素
    const index = files.findIndex((f) => f.path === path);
    if (index !== -1) {
      files.splice(index, 1);
      return;
    }

    // 从 errorFiles 中删除元素
    const errorIndex = errorFiles.findIndex((f) => f.path === path);
    if (errorIndex !== -1) {
      errorFiles.splice(errorIndex, 1);
    }
  };

  const clearPaths = () => {
    // 重置为空数组
    files.splice(0, files.length);
    errorFiles.splice(0, errorFiles.length);

    // 注意：不重置耗时，因为已完成列表没有变化
    // 重置转换状态
    isReadyForConversion.value = false;
    isConverting.value = false;
    isStopping.value = false;
    hasStartedConversion.value = false;
  };

  const clearCompletedFiles = () => {
    // 只清空已完成文件
    completedFiles.splice(0, completedFiles.length);
    // 清空已完成文件时，重置耗时统计
    spendTime.value = null;
    startTime.value = null;
    accumulatedTime.value = 0; // 重置累计时间
    isTimerRunning.value = false; // 停止计时器

    // 停止定时器
    if (timerId.value !== null) {
      clearInterval(timerId.value);
      timerId.value = null;
    }
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

  // --- 开始转换逻辑：调用批量方法 ---
  const startConversion = async () => {
    debug(`前端开始执行转换逻辑...`);

    // 如果正在转换或正在停止，不允许启动新的转换任务
    if (isConverting.value || isStopping.value) {
      void warn(`转换任务正在进行中或正在停止，忽略新的转换请求`);
      return;
    }

    if (files.length === 0) {
      void warn(`收到转换请求，但没有待转换的文件，这可能是一个前端状态同步错误。`);
      return;
    }

    const paths = files.map((f) => f.path);

    isPreparing.value = true;
    isStopping.value = false;
    hasStartedConversion.value = true;

    // 设置为准备状态，等待后端 started 事件后再启动计时器
    isPreparing.value = true;
    isTimerRunning.value = false;

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
      // 重置所有状态，包括准备状态
      isConverting.value = false;
      isStopping.value = false;
      isPreparing.value = false;
    }
  };

  // 停止转换
  const stopConversion = async () => {
    debug(`前端停止转换...`)
    try {
      // 如果正在准备状态，直接重置状态并返回
      if (isPreparing.value) {
        debug(`正在准备状态，取消转换`);
        isPreparing.value = false;
        isConverting.value = false;
        isStopping.value = false;
        return;
      }

      // 立即停止计时器，保存已经消耗的时间
      if (startTime.value !== null && isTimerRunning.value) {
        const elapsed = Date.now() - startTime.value;
        accumulatedTime.value += elapsed;
        spendTime.value = accumulatedTime.value / 1000;
        debug(`停止转换时保存已消耗时间: ${(accumulatedTime.value / 1000).toFixed(2)}s`);
      }

      // 标记计时器为停止状态
      isTimerRunning.value = false;
      startTime.value = null;

      // 停止定时器
      if (timerId.value !== null) {
        clearInterval(timerId.value);
        timerId.value = null;
      }

      // 设置停止标志，防止在停止过程中启动新的转换任务
      isStopping.value = true;

      await invoke("stop_conversion");
      debug(`已发送停止转换请求，等待后端正在转换的线程完成`);
    } catch (error) {
      alertSevere("停止转换任务失败！" + error)
      isStopping.value = false;
    }
  };

  // 处理转换开始事件（由后端通知，线程池创建完成）
  const handleStarted = () => {
    debug(`收到转换开始事件，启动计时器`);

    // 准备完成
    isPreparing.value = false;
    isConverting.value = true;
    // 启动计时器
    isTimerRunning.value = true;
    startTime.value = Date.now();

    // 启动定时器，每100ms更新一次耗时（累加之前已消耗的时间）
    timerId.value = window.setInterval(() => {
      // 必须同时检查 isTimerRunning 和 startTime，确保计时器停止后不再更新
      if (isTimerRunning.value && startTime.value !== null) {
        // 先累加毫秒，最后再转换为秒，避免精度丢失
        spendTime.value = (accumulatedTime.value + (Date.now() - startTime.value)) / 1000;
      }
    }, 100);
  };

  // 处理停止完成事件（由后端通知）
  const handleStopped = () => {
    debug(`收到停止完成通知，重置转换状态`);

    // 重置准备状态和转换状态
    isPreparing.value = false;
    isConverting.value = false;
    isStopping.value = false;

    // 注意：计时器已经在 stopConversion 中停止，这里不需要再次停止
  };

  // 处理批次完成事件（由后端通知，所有文件都处理完毕）
  const handleBatchFinished = () => {
    debug(`收到批次完成通知，停止计时器`);

    // 标记计时器为停止状态
    isTimerRunning.value = false;
    startTime.value = null;

    // 停止定时器
    if (timerId.value !== null) {
      clearInterval(timerId.value);
      timerId.value = null;
    }

    // 重置转换状态
    isConverting.value = false;
    isStopping.value = false;
    isPreparing.value = false;
    isReadyForConversion.value = false;
  };

  // 切换分组展开状态
  const toggleGroupExpansion = (group: 'task' | 'error') => {
    // 如果点击的是当前已展开的分组，则收起它
    if (group === 'task' && taskExpanded.value) {
      taskExpanded.value = false;
      return;
    }
    if (group === 'error' && errorExpanded.value) {
      errorExpanded.value = false;
      return;
    }

    // 否则，展开目标分组，收起其他分组
    taskExpanded.value = group === 'task';
    errorExpanded.value = group === 'error';
  };

  // 重试失败的文件
  const retryErrorFiles = () => {
    if (errorFiles.length === 0) return;

    // 将失败的文件移回待转换列表
    const filesToRetry = [...errorFiles];
    errorFiles.splice(0, errorFiles.length);

    // 清除错误信息
    filesToRetry.forEach(file => {
      delete file.error;
    });

    files.push(...filesToRetry);
    debug(`已将 ${filesToRetry.length} 个失败文件移回待转换列表`);
  };

  return {
    files,
    errorFiles,
    completedFiles,
    activeTab,
    settings,
    isConverting,
    isStopping,
    hasStartedConversion,
    isReadyForConversion,
    outputFolder,
    spendTime,
    startTime,
    timerId,
    accumulatedTime,
    isTimerRunning,
    isPreparing,
    stats,
    taskFiles,
    errorFilesList,
    taskExpanded,
    errorExpanded,
    addPaths,
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
    handleStarted,
    handleStopped,
    handleBatchFinished,
    toggleGroupExpansion,
    retryErrorFiles,
  };
});
