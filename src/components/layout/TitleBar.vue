<script setup lang="ts">
import { ref, computed } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Sun, Moon, Minus, X, Maximize2, Pin, MonitorDown } from "lucide-vue-next";
import AppIcon from "@/assets/64x64.png";
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

// ============================================================
// 📝 Props 接口定义
// ============================================================

interface TitleBarProps {
  /** 应用名称 */
  appName?: string;
  /** 是否显示 Logo */
  showLogo?: boolean;
  /** 是否显示主题切换按钮 */
  showThemeButton?: boolean;
  /** 是否显示托盘按钮 */
  showTrayButton?: boolean;
  /** 是否显示最小化按钮 */
  showMinimizeButton?: boolean;
  /** 是否显示最大化按钮 */
  showMaximizeButton?: boolean;
  /** 是否显示关闭按钮 */
  showCloseButton?: boolean;
  /** 是否启用窗口拖拽 */
  enableDrag?: boolean;
  /** 标题栏高度：'tiny' | 'small' | 'medium' | 'large' */
  height?: "tiny" | "small" | "medium" | "large";
}

const props = withDefaults(defineProps<TitleBarProps>(), {
  appName: "Application",
  showLogo: true,
  showThemeButton: true,
  showTrayButton: false,
  showMinimizeButton: true,
  showMaximizeButton: true,
  showCloseButton: true,
  enableDrag: true,
  height: "medium",
});

// 标题栏高度映射
const heightClass = computed(() => {
  const heightMap = {
    tiny: "h-6", // 24px
    small: "h-8", // 32px
    medium: "h-10", // 40px
    large: "h-12", // 48px
  };
  return heightMap[props.height];
});

// 图标尺寸映射 - 根据标题栏高度自适应
const iconSizeClass = computed(() => {
  const sizeMap = {
    tiny: "w-5 h-5", // 20px
    small: "w-6 h-6", // 24px
    medium: "w-7 h-7", // 28px
    large: "w-8 h-8", // 32px
  };
  return sizeMap[props.height];
});

// 文字大小映射 - 根据标题栏高度自适应
const textSizeClass = computed(() => {
  const sizeMap = {
    tiny: "text-xs", // 12px
    small: "text-sm", // 14px
    medium: "text-base", // 16px
    large: "text-base", // 16px
  };
  return sizeMap[props.height];
});

// ============================================================
// 📝 Emits 接口定义
// ============================================================

interface TitleBarEmits {
  /** 关闭窗口请求 - 需要业务逻辑处理（确认对话框等） */
  "close-request": [];
  /** 最小化到托盘请求 - 需要业务逻辑处理（托盘功能） */
  "minimize-to-tray": [];
}

const emit = defineEmits<TitleBarEmits>();

// ============================================================
// ⚙️ 通用逻辑 - 以下代码通常不需要修改
// ============================================================

// 主题切换
const isDark = useDark({
  selector: "html",
  attribute: "class",
  valueDark: "dark",
  valueLight: "light",
  initialValue: "auto",
});
const toggleDark = useToggle(isDark);

// 获取当前窗口实例
const appWindow = getCurrentWindow();

// 本地状态跟踪窗口置顶状态（因为 isAlwaysOnTop() 在某些情况下返回不可靠）
const isAlwaysOnTopLocal = ref(false);

// 窗口拖拽函数 - 使用 Tauri 2 前端 API
const dragWindow = () => appWindow.startDragging();

// 窗口操作函数 - 直接调用 Tauri 2 前端 API
const handleMinimize = async () => {
  console.log("[TitleBar] minimize called");
  try {
    await appWindow.minimize();
    console.log("[TitleBar] minimize success");
  } catch (e) {
    console.error("[TitleBar] minimize error:", e);
  }
};

const handleMaximize = async () => {
  console.log("[TitleBar] toggleMaximize called");
  try {
    await appWindow.toggleMaximize();
    console.log("[TitleBar] toggleMaximize success");
  } catch (e) {
    console.error("[TitleBar] toggleMaximize error:", e);
  }
};

const handleToggleTop = async () => {
  console.log("[TitleBar] toggleAlwaysOnTop called");
  try {
    // 使用本地状态来切换，而不是依赖 isAlwaysOnTop() 的返回值
    const newState = !isAlwaysOnTopLocal.value;
    console.log(
      "[TitleBar] local state before:",
      isAlwaysOnTopLocal.value,
      ", setting to:",
      newState
    );
    await appWindow.setAlwaysOnTop(newState);
    isAlwaysOnTopLocal.value = newState;
    console.log("[TitleBar] setAlwaysOnTop to:", newState, "success");
  } catch (e) {
    console.error("[TitleBar] toggleAlwaysOnTop error:", e);
  }
};

// 窗口拖拽逻辑
const mouseDownPosition = ref<{ x: number; y: number } | null>(null);
const isTitleDragging = ref(false);

const handleMouseDown = (e: MouseEvent) => {
  if (!props.enableDrag) return;
  if (e.button !== 0) return;
  mouseDownPosition.value = { x: e.clientX, y: e.clientY };
  isTitleDragging.value = false;
};

const handleMouseMove = (e: MouseEvent) => {
  if (!props.enableDrag) return;
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

const onDoubleClick = async () => {
  if (!props.enableDrag) return;
  if (isTitleDragging.value) return;
  await handleMaximize();
};
</script>

<template>
  <header
    :class="[
      'px-4 flex items-center justify-between shrink-0 select-none relative z-50 bg-titlebar border-b border-border/30',
      heightClass,
    ]"
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
      <!-- 左侧Logo区域 - 支持插槽 -->
      <slot name="logo">
        <div
          v-if="showLogo"
          :class="[
            'flex items-center gap-2 font-medium tracking-tight text-foreground',
            textSizeClass,
          ]"
        >
          <img :src="AppIcon" :class="iconSizeClass" alt="App Icon" />
          <span>{{ appName }}</span>
        </div>
      </slot>

      <!-- 右键菜单区域 -->
      <div class="flex-1 h-full">
        <ContextMenu>
          <ContextMenuTrigger as-child>
            <div class="w-full h-full"></div>
          </ContextMenuTrigger>
          <ContextMenuContent class="w-48">
            <ContextMenuItem v-if="showMaximizeButton" @click="handleMaximize()">
              <Maximize2 class="mr-2 h-4 w-4" /> 最大化/还原
            </ContextMenuItem>
            <ContextMenuItem v-if="showMinimizeButton" @click="handleMinimize()">
              <Minus class="mr-2 h-4 w-4" /> 最小化
            </ContextMenuItem>
            <ContextMenuItem @click="handleToggleTop()">
              <Pin class="mr-2 h-4 w-4" /> 置顶窗口
            </ContextMenuItem>
            <ContextMenuSeparator v-if="showCloseButton" />
            <ContextMenuItem
              v-if="showCloseButton"
              class="text-destructive"
              @click="emit('close-request')"
            >
              <X class="mr-2 h-4 w-4" /> 关闭
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
    </div>

    <!-- 标题栏控制按钮区域 -->
    <div class="flex items-center gap-1 pointer-events-auto">
      <!-- 右侧自定义内容插槽 -->
      <slot name="right" />

      <div class="w-[1px] h-6 bg-slate-200 dark:bg-slate-700 mr-2"></div>

      <!-- 主题切换按钮 -->
      <Tooltip v-if="showThemeButton">
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

      <!-- 额外按钮插槽 -->
      <slot name="extra-buttons" />

      <!-- 托盘按钮 -->
      <Tooltip v-if="showTrayButton">
        <TooltipTrigger>
          <Button
            data-testid="tray-button"
            variant="ghost"
            size="icon"
            class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="emit('minimize-to-tray')"
          >
            <MonitorDown :size="16" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>最小化到托盘</p>
        </TooltipContent>
      </Tooltip>

      <!-- 最小化按钮 -->
      <Button
        data-testid="minimize-button"
        v-if="showMinimizeButton"
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
        @click="handleMinimize()"
      >
        <Minus :size="16" />
      </Button>

      <!-- 最大化按钮 -->
      <Button
        data-testid="maximize-button"
        v-if="showMaximizeButton"
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800"
        @click="handleMaximize()"
      >
        <Maximize2 :size="16" />
      </Button>

      <!-- 关闭按钮 - 保留 emit，需要业务逻辑处理 -->
      <Button
        data-testid="close-button"
        v-if="showCloseButton"
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-slate-500 hover:bg-red-500 hover:text-white transition-colors"
        @click="emit('close-request')"
      >
        <X :size="16" />
      </Button>
    </div>
  </header>
</template>
