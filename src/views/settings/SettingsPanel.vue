<script setup lang="ts">
import { useConversionStore } from "@/stores/conversionStore";
import { Settings2, FolderOpen, Square } from "@lucide/vue";
import DropdownMenu from "@/components/DropdownMenu.vue";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { onMounted, computed, ref } from "vue";
import type { FormatInfo, OutputFormat } from "@/types";
import { error } from "@tauri-apps/plugin-log";
import { readStoredLocale, setStoredLocale, type StoredLocale } from "@/i18n";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const store = useConversionStore();
const storedLocale = ref<StoredLocale>(readStoredLocale());

const localeOptions = computed<{ value: StoredLocale; label: string }[]>(() => [
  { value: "system", label: t("settings.followSystem") },
  { value: "en", label: "English" },
  { value: "zh-Hans", label: "简体中文" },
  { value: "zh-Hant", label: "繁體中文（港澳台）" },
]);

const selectLocale = (value: string) => {
  storedLocale.value = value as StoredLocale;
  setStoredLocale(storedLocale.value);
};

const onFormatPick = (value: string) => {
  selectFormat(value as OutputFormat);
};

const formatTriggerClass = computed(() => [
  "w-full inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
  !isQuickFormat(store.settings.format)
    ? "bg-background text-foreground shadow-sm"
    : "text-muted-foreground hover:text-foreground hover:bg-background/50",
]);

onMounted(async () => {
  try {
    store.setOutputFolder(await downloadDir());
  } catch (e) {
    void error(`获取默认下载目录失败: ${e}`);
  }
});

const selectOutputFolder = async () => {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: t("settings.selectOutputFolder"),
    });
    if (result) store.setOutputFolder(result);
  } catch (e) {
    void error(`选择输出目录失败: ${e}`);
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

  return path.substring(0, startLength) + "..." + path.substring(path.length - endLength);
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
  return formats.filter((f) => !quickFormats.some((qf) => qf.value === f.value));
});

// 当前格式信息
const currentFormat = computed(() => {
  return formats.find((f) => f.value === store.settings.format);
});

// 选择格式
const selectFormat = (format: OutputFormat) => {
  store.updateSettings({ format });
};

// 检查是否为常用格式
const isQuickFormat = (format: OutputFormat) => {
  return quickFormats.some((qf) => qf.value === format);
};

// 质量说明文字
const qualityDescription = computed(() => {
  const format = currentFormat.value;
  if (!format) return "";
  if (format.value === "jpeg" || format.value === "webp") {
    return t("settings.qualityTooltipHigh");
  } else {
    return t("settings.qualityTooltipUnsupported");
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
    return t("settings.more");
  }
  return currentFormat.value?.label || t("settings.more");
});

// 是否显示质量控制区域
const showQualityControl = computed(() => {
  return isQualityEnabled.value;
});

// 按钮文字
const buttonText = computed(() => {
  // 优先级1：正在停止（禁用）
  if (store.isStopping) {
    return t("settings.stopping");
  }

  // 优先级2：正在准备（禁用）
  if (store.isPreparing) {
    return t("settings.preparing");
  }

  // 优先级3：正在转换（可点击停止）
  if (store.isConverting) {
    return t("settings.stopConverting");
  }

  // 优先级4：非转换状态
  const pendingCount = store.stats.waiting;
  const hasPendingFiles = pendingCount > 0;

  // 如果有待转换文件
  if (hasPendingFiles) {
    // 从未开始过转换 → 显示"开始批量转换"
    // 曾经开始过转换 → 显示"继续转换"
    return store.hasStartedConversion
      ? t("settings.continueBatch", { count: pendingCount })
      : t("settings.startBatch");
  }

  // 没有待转换文件 → 显示"开始批量转换"（会被禁用）
  return t("settings.startBatch");
});

// 按钮变体
const buttonVariant = computed(() => {
  return store.isConverting ? "destructive" : "default";
});

// 按钮图标
const buttonIcon = computed(() => {
  return store.isConverting ? Square : null;
});
</script>

<template>
  <aside class="w-80 bg-card flex flex-col shrink-0 h-full rounded-lg">
    <div class="p-6 flex-1 overflow-y-auto">
      <h2 class="text-lg font-semibold tracking-tight mb-6 flex items-center gap-2">
        <Settings2 :size="18" class="text-muted-foreground" /> {{ $t("settings.title") }}
      </h2>
      <div class="space-y-6">
        <div class="space-y-3">
          <label for="language-select" class="text-sm font-medium leading-none">{{
            $t("settings.language")
          }}</label>
          <DropdownMenu
            id="language-select"
            :items="localeOptions"
            :model-value="storedLocale"
            button-class="flex h-9 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-1 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            menu-class="left-0 w-full"
            @update:model-value="selectLocale"
          />
        </div>
        <div class="space-y-3">
          <label class="text-sm font-medium leading-none">{{ $t("settings.targetFormat") }}</label>
          <div class="grid grid-cols-3 gap-2 bg-muted p-1 rounded-md">
            <!-- 常用格式按钮 -->
            <button
              v-for="fmt in quickFormats"
              :key="fmt.value"
              class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              :class="
                store.settings.format === fmt.value
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
              "
              @click="selectFormat(fmt.value)"
            >
              {{ fmt.label }}
            </button>
            <!-- 更多格式下拉按钮 -->
            <DropdownMenu
              :items="otherFormats"
              :model-value="store.settings.format"
              :button-class="formatTriggerClass"
              menu-class="right-0 w-32"
              @update:model-value="onFormatPick"
            >
              <template #label>{{ dropdownLabel }}</template>
            </DropdownMenu>
          </div>
          <p class="text-xs text-muted-foreground mt-1">
            {{ $t("settings.currentFormat", { label: currentFormat?.label ?? "" }) }}
          </p>
        </div>
        <div v-if="showQualityControl" class="space-y-4">
          <div class="flex justify-between items-center">
            <label class="text-sm font-medium leading-none">{{ $t("settings.quality") }}</label>
            <span class="text-sm font-mono text-muted-foreground"
              >{{ store.settings.quality[0] }}%</span
            >
          </div>
          <Slider
            :model-value="store.settings.quality"
            :max="100"
            :min="10"
            :step="5"
            class="w-full"
            @update:model-value="(v) => store.updateSettings({ quality: v })"
          />
          <p class="text-xs text-muted-foreground leading-relaxed">
            {{ qualityDescription }}
          </p>
        </div>
        <div class="space-y-3">
          <label class="text-sm font-medium leading-none flex items-center justify-between">{{
            $t("settings.outputFolder")
          }}</label>
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
                  store.outputFolder ? truncatedOutputFolder : $t("settings.selectOutputFolder")
                }}</span>
              </Button>
            </TooltipTrigger>
            <TooltipContent v-if="store.outputFolder">
              <p>{{ store.outputFolder }}</p>
            </TooltipContent>
          </Tooltip>
          <p class="text-xs text-muted-foreground mt-1">{{ $t("settings.outputFolderHint") }}</p>
        </div>
      </div>
    </div>
    <div class="p-6 bg-muted/20">
      <Button
        :variant="buttonVariant"
        :disabled="(!store.isConverting && store.stats.waiting === 0) || store.isStopping"
        class="w-full h-11 text-sm"
        @click="store.isConverting ? store.stopConversion() : store.startConversion()"
      >
        <component :is="buttonIcon" v-if="buttonIcon" :size="16" class="mr-2" />
        {{ buttonText }}
      </Button>
    </div>
  </aside>
</template>
