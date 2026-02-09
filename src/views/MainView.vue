<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { debug, error } from "@tauri-apps/plugin-log";
import TitleBar from "@/components/layout/TitleBar.vue";
import FileListArea from "@/views/file/FileListArea.vue";
import SettingsPanel from "@/views/settings/SettingsPanel.vue";
import StatusBar from "@/views/status/StatusBar.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useConversionStore } from "@/stores/conversionStore";
import CloseConfirmDialog from "@/components/dialog/CloseConfirmDialog.vue";
import { CloseAction } from "@/types";

const conversionStore = useConversionStore();

// 关闭确认对话框状态
const showCloseDialog = ref(false);
const isClosing = ref(false);

// 处理关闭请求
const handleCloseRequest = async () => {
  if (conversionStore.isConverting || conversionStore.isPreparing) {
    showCloseDialog.value = true;
  } else {
    await invoke("close_window");
  }
};

// 处理关闭确认
const handleCloseConfirm = async (action: CloseAction) => {
  isClosing.value = true;

  try {
    if (action === "minimize") {
      await invoke("show_tray");
      await invoke("hide_window");
    } else if (action === "exit") {
      await invoke("close_window");
    }
  } catch (e) {
    void error(`处理关闭请求失败: ${e}`);
  } finally {
    isClosing.value = false;
  }
};

// 处理最小化请求
const handleMinimizeRequest = async () => {
  await invoke("minimize_window");
};

// 处理最大化请求
const handleMaximizeRequest = async () => {
  await invoke("toggle_maximize_window");
};

// 处理置顶请求
const handleToggleTopRequest = async () => {
  await invoke("toggle_always_on_top");
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
</script>

<template>
  <TooltipProvider>
    <div class="h-screen w-screen flex flex-col overflow-hidden bg-muted text-foreground relative">
      <TitleBar
        app-name="HEIC Converter"
        :show-tray-button="true"
        @close-request="handleCloseRequest"
        @minimize-request="handleMinimizeRequest"
        @maximize-request="handleMaximizeRequest"
        @toggle-top-request="handleToggleTopRequest"
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

  <!-- 关闭确认对话框 -->
  <CloseConfirmDialog
    v-model:open="showCloseDialog"
    :is-closing="isClosing"
    @confirm="handleCloseConfirm"
  />
</template>
