<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { debug, error } from "@tauri-apps/plugin-log";
import {
  Image as ImageIcon,
  Sun,
  Moon,
  Minus,
  X,
  Maximize2,
  Pin,
  MonitorDown,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";

import { useDark, useToggle } from "@vueuse/core";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip/";
import { useConversionStore } from "@/stores/conversionStore";
import CloseConfirmDialog from "@/components/dialog/CloseConfirmDialog.vue";
import { CloseAction } from "@/types";

const isDark = useDark({
  selector: "html",
  attribute: "class",
  valueDark: "dark",
  valueLight: "light",
  initialValue: "auto",
});
const toggleDark = useToggle(isDark);
const conversionStore = useConversionStore();

const minWindow = () => invoke("minimize_window");
const maxWindow = () => invoke("toggle_maximize_window");
const closeWindow = () => invoke("close_window");
const toggleTop = () => invoke("toggle_always_on_top");
const dragWindow = () => invoke("drag_window");

const minimizeToTray = async () => {
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

const showCloseDialog = ref(false);
const isClosing = ref(false);

const handleCloseRequested = async () => {
  if (conversionStore.isConverting || conversionStore.isPreparing) {
    showCloseDialog.value = true;
  } else {
    await closeWindow();
  }
};

const handleCloseConfirm = async (action: CloseAction) => {
  isClosing.value = true;

  try {
    if (action === "minimize") {
      await minimizeToTray();
    } else if (action === "exit") {
      await closeWindow();
    }
  } catch (e) {
    void error(`处理关闭请求失败: ${e}`);
  } finally {
    isClosing.value = false;
  }
};

const mouseDownPosition = ref<{ x: number; y: number } | null>(null);
const isTitleDragging = ref(false);

const handleMouseDown = (e: MouseEvent) => {
  if (e.button !== 0) return;
  mouseDownPosition.value = { x: e.clientX, y: e.clientY };
  isTitleDragging.value = false;
};

const handleMouseMove = (e: MouseEvent) => {
  if (!mouseDownPosition.value) return;
  const deltaX = Math.abs(e.clientX - mouseDownPosition.value.x);
  const deltaY = Math.abs(e.clientY - mouseDownPosition.value.y);
  if (deltaX > 5 || deltaY > 5) {
    if (!isTitleDragging.value) {
      dragWindow();
      isTitleDragging.value = true;
      mouseDownPosition.value = null;
    }
  }
};

const handleMouseUp = () => {
  mouseDownPosition.value = null;
  isTitleDragging.value = false;
};

const handleMouseLeave = () => {
  mouseDownPosition.value = null;
  isTitleDragging.value = false;
};

const onDoubleClick = () => {
  if (isTitleDragging.value) return;
  maxWindow();
};
</script>

<template>
  <header
    class="h-12 px-4 flex items-center justify-between shrink-0 select-none relative z-50 bg-titlebar border-b border-border/30"
  >
    <!-- 标题栏可拖拽区域 -->
    <div
      class="flex-1 h-full flex items-center pointer-events-auto cursor:grabbing"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @mouseleave="handleMouseLeave"
      @dblclick="onDoubleClick"
    >
      <!-- 左侧Logo -->
      <div class="flex items-center gap-2 font-semibold text-base tracking-tight text-foreground">
        <div class="bg-primary text-primary-foreground p-1.5 rounded-md">
          <ImageIcon :size="18" />
        </div>
        <span>HEIC Converter</span>
      </div>
      <!-- 右键菜单区域 -->
      <div class="flex-1 h-full">
        <ContextMenu>
          <ContextMenuTrigger as-child>
            <div class="w-full h-full"></div>
          </ContextMenuTrigger>
          <ContextMenuContent class="w-48">
            <ContextMenuItem @click="maxWindow"
              ><Maximize2 class="mr-2 h-4 w-4" /> 最大化/还原</ContextMenuItem
            >
            <ContextMenuItem @click="minWindow"
              ><Minus class="mr-2 h-4 w-4" /> 最小化</ContextMenuItem
            >
            <ContextMenuItem @click="toggleTop"
              ><Pin class="mr-2 h-4 w-4" /> 置顶窗口</ContextMenuItem
            >
            <ContextMenuSeparator />
            <ContextMenuItem class="text-destructive" @click="handleCloseRequested"
              ><X class="mr-2 h-4 w-4" /> 关闭</ContextMenuItem
            >
          </ContextMenuContent>
        </ContextMenu>
      </div>
    </div>
    <!-- 标题栏控制按钮区域 -->
    <div class="flex items-center gap-1 pointer-events-auto">
      <div class="w-[1px] h-6 bg-slate-200 dark:bg-slate-700 mr-2"></div>
      <Tooltip>
        <TooltipTrigger>
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 hover:text-yellow-400 dark:hover:text-yellow-400"
            @click="toggleDark()"
          >
            <Sun v-if="isDark" class="h-[1.2rem] w-[1.2rem]" />
            <Moon v-else class="h-[1.2rem] w-[1.2rem]" />
            <span class="sr-only">切换主题</span>
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>切换主题</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger>
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="minimizeToTray"
          >
            <MonitorDown :size="16" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>最小化到托盘</p>
        </TooltipContent>
      </Tooltip>
      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
        @click="minWindow"
        ><Minus :size="16"
      /></Button>
      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
        @click="maxWindow"
        ><Maximize2 :size="16"
      /></Button>
      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-red-500 hover:text-white transition-colors"
        @click="handleCloseRequested"
        ><X :size="16"
      /></Button>
    </div>
    <CloseConfirmDialog
      v-model:open="showCloseDialog"
      :is-closing="isClosing"
      @confirm="handleCloseConfirm"
    />
  </header>
</template>
