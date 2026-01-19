<script setup lang="ts">
import { useConversionStore } from "@/stores/conversionStore";
import { Settings2, FolderOpen, ChevronDown } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Separator } from "@/components/ui/separator";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { downloadDir } from "@tauri-apps/api/path";
import {onMounted, onUnmounted, computed, ref} from "vue";
import type { FormatInfo, OutputFormat } from "@/types";

const store = useConversionStore();
const showFormatDropdown = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

onMounted(async () => {
  store.outputFolder = await downloadDir();

  // 添加点击外部关闭下拉框的事件监听
  document.addEventListener('click', handleClickOutside);
})

onUnmounted(() => {
  // 移除事件监听器
  document.removeEventListener('click', handleClickOutside);
})

// 点击外部关闭下拉框
const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    showFormatDropdown.value = false;
  }
}

const selectOutputFolder = async () => {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: "选择输出目录",
    });
    if (result) store.setOutputFolder(result);
  } catch (e) {
    console.error(e);
  }
};

// 截断路径显示，保留开头和结尾
const truncatedOutputFolder = computed(() => {
  if (!store.outputFolder) return "";
  const path = store.outputFolder;
  const maxLength = 35;
  if (path.length <= maxLength) return path;

  // 尝试保留开头和结尾
  const startLength = Math.floor(maxLength / 2) - 2;
  const endLength = maxLength - startLength - 3;

  return path.substring(0, startLength) + '...' + path.substring(path.length - endLength);
});

// 支持的格式列表
const formats: FormatInfo[] = [
  { value: "jpeg", label: "JPEG", supportsQuality: true },
  { value: "png", label: "PNG", supportsQuality: false },
  { value: "webp", label: "WebP", supportsQuality: true },
  { value: "bmp", label: "BMP", supportsQuality: false },
  { value: "tiff", label: "TIFF", supportsQuality: false },
  { value: "ico", label: "ICO", supportsQuality: false },
];

// 常用格式（显示为按钮）
const quickFormats: FormatInfo[] = [
  { value: "jpeg", label: "JPEG", supportsQuality: true },
  { value: "png", label: "PNG", supportsQuality: false },
];

// 其他格式（显示在下拉菜单中）
const otherFormats = computed(() => {
  return formats.filter(f => !quickFormats.some(qf => qf.value === f.value));
});

// 当前格式信息
const currentFormat = computed(() => {
  return formats.find(f => f.value === store.settings.format);
});

// 选择格式
const selectFormat = (format: OutputFormat) => {
  store.updateSettings({ format });
  showFormatDropdown.value = false;
};

// 检查是否为常用格式
const isQuickFormat = (format: OutputFormat) => {
  return quickFormats.some(qf => qf.value === format);
};

// 质量说明文字
const qualityDescription = computed(() => {
  const format = currentFormat.value;
  if (!format) return "";
  if (format.value === "jpeg" || format.value === "webp") {
    return "更高的质量将导致文件体积变大。此选项仅对 JPEG 和 WebP 格式有效。";
  } else {
    return "当前格式不支持质量调整。";
  }
});

// 质量滑块是否可用
const isQualityEnabled = computed(() => {
  const format = currentFormat.value;
  return format?.supportsQuality || false;
});

// 下拉菜单显示的文字
const dropdownLabel = computed(() => {
  if (isQuickFormat(store.settings.format)) {
    return "更多";
  }
  return currentFormat.value?.label || "更多";
});

// 是否显示质量控制区域
const showQualityControl = computed(() => {
  return isQualityEnabled.value;
});

</script>

<template>
  <aside class="w-80 border-l bg-card flex flex-col shrink-0 h-full">
    <div class="p-6 flex-1 overflow-y-auto">
      <h2
        class="text-lg font-semibold tracking-tight mb-6 flex items-center gap-2"
      >
        <Settings2 :size="18" class="text-muted-foreground" /> 转换设置
      </h2>
      <div class="space-y-6">
        <div class="space-y-3">
          <label class="text-sm font-medium leading-none">目标格式</label>
          <div class="grid grid-cols-3 gap-2 bg-muted p-1 rounded-md">
            <!-- 常用格式按钮 -->
            <button
              v-for="fmt in quickFormats"
              :key="fmt.value"
              @click="selectFormat(fmt.value)"
              class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              :class="
                store.settings.format === fmt.value
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
              "
            >
              {{ fmt.label }}
            </button>
            <!-- 更多格式下拉按钮 -->
            <div class="relative" ref="dropdownRef">
              <button
                @click="showFormatDropdown = !showFormatDropdown"
                class="w-full inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                :class="
                  !isQuickFormat(store.settings.format)
                    ? 'bg-background text-foreground shadow-sm'
                    : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
                "
              >
                <span class="flex-1">{{ dropdownLabel }}</span>
                <ChevronDown :size="14" />
              </button>
              <!-- 下拉菜单 -->
              <div
                v-if="showFormatDropdown"
                class="absolute right-0 top-full mt-1 w-32 bg-popover text-popover-foreground rounded-md border shadow-md z-50"
              >
                <div class="p-1">
                  <button
                    v-for="fmt in otherFormats"
                    :key="fmt.value"
                    @click="selectFormat(fmt.value)"
                    class="w-full text-left px-3 py-2 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground transition-colors"
                    :class="{
                      'bg-accent text-accent-foreground': store.settings.format === fmt.value
                    }"
                  >
                    {{ fmt.label }}
                  </button>
                </div>
              </div>
            </div>
          </div>
          <p class="text-xs text-muted-foreground mt-1">
            当前选择：{{ currentFormat?.label }}
          </p>
        </div>
        <Separator v-if="showQualityControl" />
        <div v-if="showQualityControl" class="space-y-4">
          <div class="flex justify-between items-center">
            <label class="text-sm font-medium leading-none">图片质量</label>
            <span class="text-sm font-mono text-muted-foreground"
              >{{ store.settings.quality[0] }}%</span
            >
          </div>
          <Slider
            :model-value="store.settings.quality"
            @update:model-value="(v) => store.updateSettings({ quality: v })"
            :max="100"
            :min="10"
            :step="5"
            class="w-full"
          />
          <p class="text-xs text-muted-foreground leading-relaxed">
            {{ qualityDescription }}
          </p>
        </div>
        <Separator />
        <div class="space-y-3">
          <label
            class="text-sm font-medium leading-none flex items-center justify-between"
            >输出目录</label
          >
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger as-child>
                <Button
                  variant="outline"
                  size="sm"
                  class="w-full justify-start gap-2"
                  @click="selectOutputFolder"
                >
                  <FolderOpen :size="16" />
                  <span class="truncate">{{
                    store.outputFolder ? truncatedOutputFolder : "选择输出目录"
                  }}</span>
                </Button>
              </TooltipTrigger>
              <TooltipContent v-if="store.outputFolder">
                <p>{{ store.outputFolder }}</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <p class="text-xs text-muted-foreground mt-1">
            未选择时，将保存到系统的"下载"文件夹。
          </p>
        </div>
      </div>
    </div>
    <div class="p-6 border-t bg-muted/20">
      <Button
        @click="store.startConversion"
        :disabled="store.isConverting || !store.isReadyForConversion"
        class="w-full h-11 text-base"
      >
        开始批量转换
      </Button>
    </div>
  </aside>
</template>