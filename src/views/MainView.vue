<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ask } from "@tauri-apps/plugin-dialog";
import { debug, error } from "@tauri-apps/plugin-log";
import TitleBar from "@/components/layout/TitleBar.vue";
import FileListArea from "@/views/file/FileListArea.vue";
import SettingsPanel from "@/views/settings/SettingsPanel.vue";
import StatusBar from "@/views/status/StatusBar.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useConversionStore } from "@/stores/conversionStore";
import ConvertingCloseConfirmDialog from "@/components/dialog/ConvertingCloseConfirmDialog.vue";
import PendingFilesCloseConfirmDialog from "@/components/dialog/PendingFilesCloseConfirmDialog.vue";
import { CloseAction } from "@/types";
import { getCurrentWindow } from "@tauri-apps/api/window";

const conversionStore = useConversionStore();

// 关闭确认对话框状态
const showCloseDialog = ref(false);
const showPendingCloseDialog = ref(false);
const isClosing = ref(false);

// 是否有待转换文件
const hasPendingFiles = computed(() => conversionStore.files.length > 0);

// 获取当前窗口实例
const appWindow = getCurrentWindow();

// 处理关闭请求
const handleCloseRequest = async () => {
  if (conversionStore.isConverting || conversionStore.isPreparing) {
    // 转换进行中，显示转换中确认对话框
    showCloseDialog.value = true;
  } else if (hasPendingFiles.value) {
    // 有待转换文件，显示待转换确认对话框
    showPendingCloseDialog.value = true;
  } else {
    // 直接关闭 - 使用 Tauri 2 前端 API
    await appWindow.close();
  }
};

// 处理关闭确认（转换中）
const handleCloseConfirm = async (action: CloseAction) => {
  isClosing.value = true;

  try {
    if (action === "minimize") {
      await invoke("show_tray");
      await invoke("hide_window");
    } else if (action === "exit") {
      // 使用 Tauri 2 前端 API
      await appWindow.close();
    }
  } catch (e) {
    void error(`处理关闭请求失败: ${e}`);
  } finally {
    isClosing.value = false;
  }
};

// 处理待转换文件关闭确认
const handlePendingCloseConfirm = async () => {
  isClosing.value = true;

  try {
    // 使用 Tauri 2 前端 API
    await appWindow.close();
  } catch (e) {
    void error(`处理关闭请求失败: ${e}`);
  } finally {
    isClosing.value = false;
  }
};

// 处理托盘退出请求
const handleTrayQuitRequest = async () => {
  if (conversionStore.isConverting || conversionStore.isPreparing) {
    // 转换进行中，使用原生对话框确认
    const confirmed = await ask("转换正在进行中，确认要停止转换并退出吗？", {
      title: "确认退出",
      kind: "warning",
    });
    if (confirmed) {
      void invoke("force_exit");
    }
  } else {
    // 直接退出
    void invoke("force_exit");
  }
};

// 处理最小化到托盘
const handleMinimizeToTray = async () => {
  void debug("点击最小化到托盘按钮");
  try {
    await invoke("show_tray");
    void debug("托盘显示成功");
    await invoke("hide_window");
    void debug("窗口隐藏成功");
  } catch (e) {
    void error(`最小化到托盘失败: ${e}`);
  }
};

// 监听托盘退出请求事件
let unlistenTrayQuitRequest: (() => void) | null = null;

onMounted(async () => {
  unlistenTrayQuitRequest = await listen("tray-quit-request", () => {
    handleTrayQuitRequest();
  });
});

onUnmounted(() => {
  if (unlistenTrayQuitRequest) {
    unlistenTrayQuitRequest();
  }
});
</script>

<template>
  <TooltipProvider>
    <div class="h-screen w-screen flex flex-col overflow-hidden bg-muted text-foreground relative">
      <TitleBar
        app-name="HEIC 图片格式转换器"
        :show-tray-button="true"
        height="medium"
        @close-request="handleCloseRequest"
        @minimize-to-tray="handleMinimizeToTray"
      />
      <main class="flex-1 flex overflow-hidden pointer-events-auto p-2 gap-2">
        <FileListArea />
        <SettingsPanel />
      </main>
      <StatusBar />
      <!--    <InputFile />-->
    </div>
  </TooltipProvider>

  <!-- 关闭确认对话框（转换中） -->
  <ConvertingCloseConfirmDialog
    v-model:open="showCloseDialog"
    :is-closing="isClosing"
    @confirm="handleCloseConfirm"
  />

  <!-- 关闭确认对话框（有待转换文件） -->
  <PendingFilesCloseConfirmDialog
    v-model:open="showPendingCloseDialog"
    :is-closing="isClosing"
    @confirm="handlePendingCloseConfirm"
  />
</template>
