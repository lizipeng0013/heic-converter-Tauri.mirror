<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import { useConversionStore } from "@/stores/conversionStore";
import { formatSize } from "@/utils";
import {
  Upload,
  Trash2,
  FileImage,
  CheckCircle2,
  Loader2,
  FolderOpen,
  AlertCircle,
} from "lucide-vue-next";
import { open } from '@tauri-apps/plugin-dialog'
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { FileItem } from "@/types/index";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip/";

import {listen, TauriEvent, UnlistenFn} from "@tauri-apps/api/event";
import {info} from "@tauri-apps/plugin-log";
import {alertSevere} from "@/utils/useError.ts";

const conversionStore = useConversionStore();
const isFileDragging = ref(false);
const fileRefs = ref<Record<string, HTMLElement>>({});
const setFileRef = (path: string, el: any) => {
  if (el) {
    fileRefs.value[path] = el;
  } else {
    delete fileRefs.value[path];
  }
}
let unlistenDragEnter: UnlistenFn | null = null;
let unlistenDragDrop: UnlistenFn | null = null;
let unlistenDragLeave: UnlistenFn | null = null;
let unlistenConversion: UnlistenFn | null = null;
let unlistenBatchFinished: UnlistenFn | null = null;

onMounted(async () => {
  // 监听：文件悬停在窗口任意位置
  unlistenDragEnter = await listen(TauriEvent.DRAG_ENTER, () => {
    isFileDragging.value = true;
    info(`Drag Enter`);
  });

  // 监听：放下文件
  unlistenDragDrop = await listen(TauriEvent.DRAG_DROP, (event) => {
    // 1. 先结束动画状态
    isFileDragging.value = false;
    info(`Drag Drop`);
    // 2. 直接处理，不做区域判断
    // const paths = event.payload as string[];
    const payload = event.payload as any;
    const paths = payload.paths as string[];
    conversionStore.addPaths(paths);
  });

  // 监听：取消 (离开窗口或拖到别处去了)
  unlistenDragLeave = await listen(TauriEvent.DRAG_LEAVE, () => {
    isFileDragging.value = false;
    info(`Drag Leave`);
  });

  unlistenConversion = await listen("conversion-update", (event) => {
    // 如果已经停止转换，忽略这些更新
    if (!conversionStore.isConverting) {
      return;
    }
    
    const payload = event.payload as any;
    const {path, status, progress, output_path, error } = payload;
    if (status === "done") {
      conversionStore.updateFileSuccess(path, output_path);
    } else if (status === "converting") {
      conversionStore.updateFileStatus(path, "converting", progress);
      if (progress === 0) {
        const el = fileRefs.value[path];
        if (el) {
          el.scrollIntoView({
            behavior: "smooth",
            block: "center",
            inline: "nearest",
          });
        }
      }
    } else if (status === "error") {
      conversionStore.updateFileError(path, error);
    }
  })

  unlistenBatchFinished = await listen("conversion-batch-finished", (event) => {
    info(`转换任务完成`);
    // 只有在转换状态下才设置为 false
    if (conversionStore.isConverting) {
      conversionStore.isConverting = false;
    }
    const payload = event.payload as any;
    let spendTime = (payload.spend_time/1000).toFixed(1)
    info(`转换耗时：${spendTime}s`);
    conversionStore.spendTime = parseFloat(spendTime);
    conversionStore.isReadyForConversion = false;
  })

});

onUnmounted(() => {
  unlistenDragEnter?.();
  unlistenDragDrop?.();
  unlistenDragLeave?.();
  unlistenConversion?.();
  unlistenBatchFinished?.();
});

// import { ConversionService } from "@/services/conversionService"; // 你的服务

// 新的文件选择函数
const selectFilesWithDialog = async () => {
  try {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "HEIC/HEIF 文件",
          extensions: ["heic", "heif"],
        },
      ],
    });

    if (!selected) return;

    const filePaths = Array.isArray(selected) ? selected : [selected];
    await conversionStore.addPaths(filePaths);
  } catch (error) {
    alertSevere("选择文件失败：" + error)
  }
};

const handleOpenFileDir = async (file: FileItem) => {
  try {
    if (file.convertedFilePath) {
      await revealItemInDir(file.convertedFilePath);
    }
  } catch (error) {
    alertSevere("打开目录并定位文件失败：" + error);
  }
};

</script>

<template>
  <section
    class="flex-1 flex flex-col border-r min-w-0 bg-muted/10 h-full relative z-0"
  >
    <div class="h-12 px-4 flex items-center justify-between border-b shrink-0">
      <h3 class="text-sm font-medium text-muted-foreground">
        文件队列 ({{ conversionStore.stats.total }})
      </h3>
      <Button
        v-if="conversionStore.stats.total > 0"
        variant="ghost"
        size="sm"
        class="h-8 text-xs"
        @click="conversionStore.clearPaths()"
        >清空列表</Button
      >
    </div>

    <div
      class="flex-1 w-full overflow-y-auto p-2 space-y-1"
      :class="{
        'bg-primary/10': isFileDragging, // 【背景】明显变蓝
      }"
    >
      <div
        v-if="conversionStore.stats.total === 0"
        class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground/50 pointer-events-none"
      >
        <div
          class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4 border-2 border-dashed"
        >
          <Upload />
        </div>
        <p class="font-medium" :class="isFileDragging ? 'text-primary' : ''">
          拖拽 HEIC 文件到此处
        </p>
        <p
          class="text-sm mt-1 opacity-70"
          :class="isFileDragging ? 'font-medium text-primary' : ''"
        >
          支持 .heic, .heif 格式
        </p>
      </div>

      <Card
        v-for="file in conversionStore.files"
        :key="file.path"
        class="group overflow-hidden transition-colors hover:border-primary/50"
      >
        <CardContent class="p-1 flex items-center justify-between gap-2">
          <div class="flex items-center gap-2 min-w-0 flex-1 max-w-[calc(100%-2rem)]"
               :ref="(el) => setFileRef(file.path, el)"
          >
            <div
              class="h-7 w-7 shrink-0 rounded bg-secondary flex items-center justify-center text-secondary-foreground"
            >
              <FileImage :size="14" />
            </div>
            <div class="flex flex-col justify-center gap-0.5 overflow-hidden min-w-0 flex-1">
              <div class="flex items-center gap-2">
                <p class="text-sm font-medium truncate flex-1 min-w-0">{{ file.name }}</p>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Button
                          v-if="file.convertedFilePath"
                          variant="ghost"
                          size="icon"
                          class="h-7 w-7 opacity-0 group-hover:opacity-100 text-slate-500 hover:text-primary dark:hover:text-primary transition-colors"
                          @click="handleOpenFileDir(file)"
                      >
                        <FolderOpen :size="14" />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>打开转换成功的文件所在目录</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                <TooltipProvider v-if="file.status === 'error' && file.error">
                  <Tooltip>
                    <TooltipTrigger as-child>
                      <Badge
                        variant="destructive"
                        class="h-5 px-1.5 text-[10px] flex-shrink-0 cursor-help w-16 justify-center"
                      >
                        <span class="flex items-center gap-0.5">
                          <AlertCircle :size="12" />
                          失败
                        </span>
                      </Badge>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p class="max-w-xs break-words">{{ file.error }}</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                <Badge
                  v-else
                  :variant="file.status === 'done' ? 'default' : 'secondary'"
                  class="h-5 px-1.5 text-[10px] flex-shrink-0 w-16 justify-center"
                >
                  <span
                    v-if="file.status === 'done'"
                    class="flex items-center gap-0.5"
                    ><CheckCircle2 :size="10" /> 完成</span
                  >
                  <span
                    v-else-if="file.status === 'converting'"
                    class="flex items-center gap-0.5 text-yellow-500 dark:text-yellow-400"
                    ><Loader2 :size="10" class="animate-spin" />
                    {{ file.progress }}%</span
                  >
                  <span v-else>等待</span>
                </Badge>
              </div>
              <div class="text-xs text-muted-foreground">
                {{ formatSize(file.size) }}
              </div>
              <div
                v-if="file.status === 'converting' || file.status === 'done'"
                class="h-1.5 w-full bg-secondary rounded-full overflow-hidden"
              >
                <div
                  class="h-full bg-primary transition-all duration-300 ease-out"
                  :class="{
                    'bg-green-500': file.status === 'done',  // 完成后变绿
                    'bg-primary': file.status === 'converting'
                  }"
                  :style="{ width: file.progress + '%' }"
                ></div>
              </div>
            </div>
          </div>
          <div class="flex items-center gap-1 shrink-0 w-8 justify-end">

            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <Button
                    variant="ghost"
                    size="icon"
                    class="h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground hover:text-destructive"
                    @click="conversionStore.removePath(file.path)"
                  >
                    <Trash2 :size="14" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>
                  <p>从文件队列中移除</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
        </CardContent>
      </Card>
    </div>

    <div class="p-2 border-t bg-card shrink-0 relative z-10">
      <div class="relative w-full">
        
        <label
          for="fileInput"
          class="cursor-pointer flex items-center justify-center w-full h-10 rounded-md border border-input bg-background px-8 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground"
          @click="selectFilesWithDialog"
        >
          <Upload :size="16" class="mr-2" /> 选择文件
        </label>
      </div>
    </div>
  </section>
</template>
