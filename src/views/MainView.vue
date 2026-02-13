<script setup lang="ts">
import { ref, computed, onUnmounted } from "vue";
import { ask } from "@tauri-apps/plugin-dialog";
import { debug, error } from "@tauri-apps/plugin-log";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { TrayIcon } from "@tauri-apps/api/tray";
import { Menu, MenuItem } from "@tauri-apps/api/menu";
import TitleBar from "@/components/layout/TitleBar.vue";
import FileListArea from "@/views/file/FileListArea.vue";
import SettingsPanel from "@/views/settings/SettingsPanel.vue";
import StatusBar from "@/views/status/StatusBar.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useConversionStore } from "@/stores/conversionStore";
import ConvertingCloseConfirmDialog from "@/components/dialog/ConvertingCloseConfirmDialog.vue";
import PendingFilesCloseConfirmDialog from "@/components/dialog/PendingFilesCloseConfirmDialog.vue";
import { CloseAction } from "@/types";
import { defaultWindowIcon } from "@tauri-apps/api/app";

const conversionStore = useConversionStore();

// 关闭确认对话框状态
const showCloseDialog = ref(false);
const showPendingCloseDialog = ref(false);
const isClosing = ref(false);

// 是否有待转换文件
const hasPendingFiles = computed(() => conversionStore.files.length > 0);

// 获取当前窗口实例
const appWindow = getCurrentWindow();

// 托盘实例
let trayInstance: TrayIcon | null = null;

// 创建托盘
const createTray = async () => {
  try {
    // 检查托盘是否已存在
    if (trayInstance != null) {
      return;
    }

    // 创建托盘菜单
    const menu = await Menu.new();

    // 显示窗口菜单项
    const showItem = await MenuItem.new({
      id: "show",
      text: "显示窗口",
      action: async () => {
        await appWindow.show();
        await appWindow.unminimize();
        await appWindow.setFocus();
      },
    });

    // 退出菜单项
    const quitItem = await MenuItem.new({
      id: "quit",
      text: "退出",
      action: () => {
        handleTrayQuitRequest();
      },
    });

    await menu.append(showItem);
    await menu.append(quitItem);

    const appIcon = await defaultWindowIcon();

    // 创建托盘图标（使用应用图标）
    trayInstance = await TrayIcon.new({
      id: "main-tray",
      ...(appIcon && { icon: appIcon }),
      menu: menu,
      menuOnLeftClick: false,
      tooltip: "HEIC 图片格式转换器",
      action: async (event) => {
        switch (event.type) {
          case "Click":
            if (event.button == "Left" && event.buttonState == "Up") {
              await appWindow.show();
              await appWindow.unminimize();
              await appWindow.setFocus();
            }
            break;
        }
      },
    });

    void debug("托盘创建成功");
  } catch (e) {
    void error(`创建托盘失败: ${e}`);
  }
};

// 处理最小化到托盘
const handleHideToTray = async () => {
  void debug("点击最小化到托盘按钮");
  try {
    // 检查托盘是否已存在（直接查询后端状态）
    const existingTray = await TrayIcon.getById("main-tray");
    if (!existingTray) {
      await createTray();
    }
    // 隐藏窗口
    await appWindow.hide();
    void debug("窗口隐藏成功");
  } catch (e) {
    void error(`最小化到托盘失败: ${e}`);
  }
};

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
    if (action === "hideToTray") {
      // 最小化到托盘：隐藏窗口
      await handleHideToTray();
    } else if (action === "exit") {
      // 退出应用：关闭主窗口
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
      await appWindow.close();
    }
  } else {
    // 直接退出
    await appWindow.close();
  }
};

onUnmounted(() => {
  // 托盘会在应用关闭时自动清理，无需手动清理
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
        @minimize-to-tray="handleHideToTray"
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
