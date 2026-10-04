<script setup lang="ts">
import { FileImage, Trash2, Loader2, AlertCircle, FolderOpen, Clock } from "@lucide/vue";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { formatSize } from "@/utils";
import { useConversionStore } from "@/stores/conversionStore";
import { error } from "@tauri-apps/plugin-log";
import type { FileItem } from "@/types";
import { computed } from "vue";

interface Props {
  file: FileItem;
  isTaskFile?: boolean; // 是否是任务文件（待转换或正在转换）
  isErrorFile?: boolean; // 是否是错误文件
  isCompletedFile?: boolean; // 是否是已完成文件
}

const props = withDefaults(defineProps<Props>(), {
  isTaskFile: false,
  isErrorFile: false,
  isCompletedFile: false,
});

const conversionStore = useConversionStore();

// 判断是否正在处理中（正在转换或正在停止）
const isProcessing = computed(
  () => props.isTaskFile && (conversionStore.isConverting || conversionStore.isStopping)
);

// 已完成项显示"源文件名 → 生成文件名"（生成名取事件回报的实际输出路径）
// ponytail: 仅展示规划路径的 basename，与磁盘极端竞争下的最终名可能有毫秒级偏差
const displayName = computed(() => {
  if (props.isCompletedFile && props.file.convertedFilePath) {
    const outputName =
      props.file.convertedFilePath.split(/[\\/]/).pop() ?? props.file.convertedFilePath;
    return `${props.file.name} → ${outputName}`;
  }
  return props.file.name;
});

const handleOpenFileDir = async () => {
  if (props.file.convertedFilePath) {
    try {
      const { revealItemInDir } = await import("@tauri-apps/plugin-opener");
      await revealItemInDir(props.file.convertedFilePath);
    } catch (e) {
      void error(`打开文件所在目录失败: ${e}`);
    }
  }
};
</script>

<template>
  <Card class="group overflow-hidden transition-colors hover:border-primary/50 mr-3">
    <CardContent class="p-1 flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 min-w-0 flex-1 max-w-[calc(100%-3rem)]">
        <div
          class="h-7 w-7 shrink-0 rounded bg-secondary flex items-center justify-center text-secondary-foreground"
        >
          <FileImage :size="14" />
        </div>
        <div class="flex flex-col justify-center gap-0.5 overflow-hidden min-w-0 flex-1">
          <div class="flex items-center gap-2">
            <p class="text-sm font-medium truncate flex-1 min-w-0" :title="displayName">
              {{ displayName }}
            </p>
          </div>

          <div class="text-xs text-muted-foreground">
            {{ file.size === 0 ? $t("card.loading") : formatSize(file.size) }}
          </div>
        </div>
      </div>

      <div class="flex items-center gap-1 shrink-0 mr-2">
        <!-- 处理中徽章（任务文件，且正在转换或停止中） -->
        <Badge
          v-if="isProcessing"
          variant="default"
          class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center"
        >
          <Loader2 :size="10" class="animate-spin" />
          {{ $t("card.badgeProcessing") }}
        </Badge>

        <!-- 等待徽章（任务文件，且未开始转换） -->
        <Badge
          v-if="isTaskFile && !isProcessing"
          variant="secondary"
          class="h-5 px-1.5 text-[10px] flex-shrink-0 w-16 justify-center"
        >
          <Clock :size="10" />
          {{ $t("card.badgeWaiting") }}
        </Badge>

        <!-- 错误徽章（错误文件） -->
        <Tooltip v-if="isErrorFile && file.errorMessage">
          <TooltipTrigger as-child>
            <Badge
              variant="destructive"
              class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center cursor-help"
            >
              <AlertCircle :size="10" /> {{ $t("card.badgeFailed") }}
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p class="max-w-xs break-words">{{ file.errorMessage }}</p>
          </TooltipContent>
        </Tooltip>

        <!-- 打开文件按钮（已完成文件） -->
        <Tooltip v-if="isCompletedFile">
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 opacity-0 group-hover:opacity-100 text-slate-500 hover:text-primary dark:hover:text-primary transition-colors"
              @click="handleOpenFileDir"
            >
              <FolderOpen :size="14" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{{ $t("card.openDir") }}</p>
          </TooltipContent>
        </Tooltip>

        <!-- 删除按钮 -->
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground hover:text-destructive"
              @click="
                isCompletedFile
                  ? conversionStore.removeCompletedFile(file.path)
                  : isErrorFile
                    ? conversionStore.removePath(file.path)
                    : conversionStore.removePath(file.path)
              "
            >
              <Trash2 :size="14" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>
              {{
                isCompletedFile
                  ? $t("card.removeCompleted")
                  : isErrorFile
                    ? $t("card.removeError")
                    : $t("card.removeTask")
              }}
            </p>
          </TooltipContent>
        </Tooltip>
      </div>
    </CardContent>
  </Card>
</template>
