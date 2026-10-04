<script setup lang="ts">
import { ref, computed, onUnmounted, watch } from "vue";
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
import { useI18n } from "vue-i18n";
import { i18n } from "@/i18n";
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

// 构建托盘菜单（语言切换后可重复构建刷新文案）
const buildTrayMenu = async () => {
  const menu = await Menu.new();

  // 显示窗口菜单项
  const showItem = await MenuItem.new({
    id: "show",
    text: t("tray.showWindow"),
    action: async () => {
      await appWindow.show();
      await appWindow.unminimize();
      await appWindow.setFocus();
    },
  });

  // 退出菜单项
  const quitItem = await MenuItem.new({
    id: "quit",
    text: t("tray.exit"),
    action: () => {
      handleTrayQuitRequest();
    },
  });

  await menu.append(showItem);
  await menu.append(quitItem);
  return menu;
};

// 创建托盘
const createTray = async () => {
  try {
    // 检查托盘是否已存在
    if (trayInstance != null) {
      return;
    }

    // 创建托盘菜单
    const menu = await buildTrayMenu();

    const appIcon = await defaultWindowIcon();

    // 创建托盘图标（使用应用图标）
    trayInstance = await TrayIcon.new({
      id: "main-tray",
      ...(appIcon && { icon: appIcon }),
      menu: menu,
      menuOnLeftClick: false,
      tooltip: t("app.name"),
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

const { t } = useI18n();

// 语言切换后就地刷新托盘菜单与提示（创建时定格的文案不会自动更新）
watch(i18n.global.locale, async () => {
  const tray = trayInstance;
  if (!tray) return;
  try {
    await tray.setTooltip(t("app.name"));
    await tray.setMenu(await buildTrayMenu());
  } catch (e) {
    void error(`刷新托盘语言失败: ${e}`);
  }
});

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
    try {
      await appWindow.close();
    } catch (e) {
      void error(`关闭窗口失败: ${e}`);
    }
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
  try {
    if (conversionStore.isConverting || conversionStore.isPreparing) {
      // 转换进行中，使用原生对话框确认
      const confirmed = await ask(t("confirm.exitBody"), {
        title: t("confirm.exitTitle"),
        kind: "warning",
      });
      if (confirmed) {
        await appWindow.close();
      }
    } else {
      // 直接退出
      await appWindow.close();
    }
  } catch (e) {
    void error(`处理退出请求失败: ${e}`);
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
        :app-name="$t('app.name')"
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
