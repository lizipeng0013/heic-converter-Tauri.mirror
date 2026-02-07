<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, nextTick } from "vue";
import { useConversionStore } from "@/stores/conversionStore";
import { formatSize } from "@/utils";
import {
  Upload,
  Trash2,
  CheckCircle2,
  Loader2,
  FolderOpen,
  AlertCircle,
  Clock,
  ChevronDown,
} from "lucide-vue-next";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { useVirtualizer } from "@tanstack/vue-virtual";
import FileCard from "@/components/FileCard.vue";
import { listen, TauriEvent, UnlistenFn } from "@tauri-apps/api/event";
import { info, debug } from "@tauri-apps/plugin-log";
import { alertSevere } from "@/utils/useError.ts";

const conversionStore = useConversionStore();
const isFileDragging = ref(false);

// 按集合分组
const taskFiles = computed(() => conversionStore.taskFiles); // 任务文件
const errorFiles = computed(() => conversionStore.errorFilesList); // 转换失败的文件

// 虚拟滚动容器引用
const taskListRef = ref<HTMLElement | null>(null);
const errorListRef = ref<HTMLElement | null>(null);
const completedListRef = ref<HTMLElement | null>(null);

// 虚拟滚动配置
const itemSize = 68; // 每个文件卡片的估计高度（像素）
const itemPadding = 4; // 每个文件卡片的上下内边距（像素）

// 任务文件的虚拟滚动
const taskVirtualizerOptions = computed(() => ({
  count: taskFiles.value.length,
  getScrollElement: () => taskListRef.value,
  estimateSize: () => itemSize,
  overscan: 5,
}));

const taskVirtualizer = useVirtualizer(taskVirtualizerOptions);
const taskVirtualRows = computed(() =>
  taskVirtualizer.value.getVirtualItems(),
);
const taskTotalSize = computed(() =>
  taskVirtualizer.value.getTotalSize(),
);

// 转换失败的虚拟滚动
const errorVirtualizerOptions = computed(() => ({
  count: errorFiles.value.length,
  getScrollElement: () => errorListRef.value,
  estimateSize: () => itemSize,
  overscan: 5,
}));

const errorVirtualizer = useVirtualizer(errorVirtualizerOptions);
const errorVirtualRows = computed(() =>
  errorVirtualizer.value.getVirtualItems(),
);
const errorTotalSize = computed(() => errorVirtualizer.value.getTotalSize());

// 已完成的虚拟滚动
const completedVirtualizerOptions = computed(() => ({
  count: conversionStore.completedFiles.length,
  getScrollElement: () => completedListRef.value,
  estimateSize: () => itemSize,
  overscan: 5,
}));

const completedVirtualizer = useVirtualizer(completedVirtualizerOptions);
const completedVirtualRows = computed(() =>
  completedVirtualizer.value.getVirtualItems(),
);
const completedTotalSize = computed(() =>
  completedVirtualizer.value.getTotalSize(),
);

let unlistenDragEnter: UnlistenFn | null = null;
let unlistenDragDrop: UnlistenFn | null = null;
let unlistenDragLeave: UnlistenFn | null = null;
let unlistenConversion: UnlistenFn | null = null;
let unlistenBatchFinished: UnlistenFn | null = null;
let unlistenConversionStarted: UnlistenFn | null = null;
let unlistenConversionStopped: UnlistenFn | null = null;

onMounted(async () => {
  // 监听：文件悬停在窗口任意位置
  unlistenDragEnter = await listen(TauriEvent.DRAG_ENTER, () => {
    isFileDragging.value = true;
  });

  // 监听：放下文件
  unlistenDragDrop = await listen(TauriEvent.DRAG_DROP, (event) => {
    // 1. 先结束动画状态
    isFileDragging.value = false;
    // 2. 直接处理，不做区域判断
    const payload = event.payload as any;
    const paths = payload.paths as string[];
    conversionStore.addPaths(paths);
  });

  // 监听：取消 (离开窗口或拖到别处去了)
  unlistenDragLeave = await listen(TauriEvent.DRAG_LEAVE, () => {
    isFileDragging.value = false;
  });

  unlistenConversion = await listen("conversion-update", (event) => {
    const payload = event.payload as any;
    const { path, status, output_path, error } = payload;

    // 如果正在停止转换，只处理 done 状态的更新（让正在转换的文件可以完成）
    if (conversionStore.isStopping) {
      if (status === "done") {
        conversionStore.updateFileSuccess(path, output_path);
      } else if (status === "error") {
        conversionStore.updateFileError(path, error);
      }
      return;
    }

    // 正常转换状态下处理所有更新
    if (status === "done") {
      conversionStore.updateFileSuccess(path, output_path);
    } else if (status === "error") {
      conversionStore.updateFileError(path, error);
    }
  });

  unlistenBatchFinished = await listen("conversion-batch-finished", (event) => {
    info(`转换任务完成`);
    conversionStore.handleBatchFinished();
  });

  unlistenConversionStarted = await listen("conversion-started", (event) => {
    info(`收到转换开始事件`);
    conversionStore.handleStarted();
  });

  unlistenConversionStopped = await listen("conversion-stopped", (event) => {
    info(`收到停止完成事件`);
    conversionStore.handleStopped();
  });
});

onUnmounted(() => {
  unlistenDragEnter?.();
  unlistenDragDrop?.();
  unlistenDragLeave?.();
  unlistenConversion?.();
  unlistenBatchFinished?.();
  unlistenConversionStarted?.();
  unlistenConversionStopped?.();
});

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
    alertSevere("选择文件失败：" + error);
  }
};

const handleOpenFileDir = async (file: any) => {
  try {
    if (file.convertedFilePath) {
      const { revealItemInDir } = await import("@tauri-apps/plugin-opener");
      await revealItemInDir(file.convertedFilePath);
    }
  } catch (error) {
    alertSevere("打开目录并定位文件失败：" + error);
  }
};
</script>

<style scoped>
.group-section {
  margin-bottom: 8px;
}

.group-section:last-child {
  margin-bottom: 0;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  font-size: 11px;
  font-weight: 500;
  color: hsl(var(--muted-foreground));
  background: hsl(var(--muted));
  border-radius: 6px;
  margin-bottom: 4px;
}

.group-header.text-destructive {
  color: hsl(var(--destructive));
  background: hsl(var(--destructive) / 0.1);
}

.virtual-list {
  height: 100%;
  overflow-y: auto;
}

.virtual-item {
  padding: 0;
  box-sizing: border-box;
}

.virtual-item > * {
  padding: 4px 0;
  box-sizing: border-box;
}
</style>

<template>
  <section
    class="flex-1 flex flex-col min-w-0 bg-card h-full relative z-0 rounded-lg"
  >
    <div class="h-12 px-4 flex items-center justify-between shrink-0">
      <Tabs v-model="conversionStore.activeTab" class="w-full">
        <TabsList class="h-8">
          <TabsTrigger value="pending" class="text-xs">
            任务列表 ({{ conversionStore.files.length }})
          </TabsTrigger>
          <TabsTrigger value="completed" class="text-xs">
            已完成 ({{ conversionStore.completedFiles.length }})
          </TabsTrigger>
          <TabsTrigger
            v-if="conversionStore.errorFiles.length > 0"
            value="error"
            class="text-xs"
          >
            转换失败 ({{ conversionStore.errorFiles.length }})
          </TabsTrigger>
        </TabsList>
      </Tabs>
      <Button
        v-if="
          conversionStore.activeTab === 'pending' &&
          conversionStore.files.length > 0
        "
        variant="ghost"
        size="sm"
        class="h-8 text-xs ml-2"
        @click="conversionStore.clearPaths()"
        >清空列表</Button
      >
      <Button
        v-if="
          conversionStore.activeTab === 'error' &&
          conversionStore.errorFiles.length > 0
        "
        variant="ghost"
        size="sm"
        class="h-8 text-xs ml-2"
        @click="conversionStore.clearPaths()"
        >清空失败</Button
      >
      <Button
        v-if="
          conversionStore.activeTab === 'completed' &&
          conversionStore.completedFiles.length > 0
        "
        variant="ghost"
        size="sm"
        class="h-8 text-xs ml-2"
        @click="conversionStore.clearCompletedFiles()"
        >清空已完成</Button
      >
    </div>

    <div
      class="flex-1 w-full overflow-hidden pl-2 pt-2 pb-2 relative"
      :class="{
        'bg-primary/10': isFileDragging, // 【背景】明显变蓝
      }"
    >
      <!-- 空状态：任务标签页 -->
      <div
        v-if="
          conversionStore.activeTab === 'pending' &&
          conversionStore.files.length === 0
        "
        class="absolute inset-0 flex flex-col items-center justify-center"
      >
        <button
          @click="selectFilesWithDialog"
          class="h-16 w-16 rounded-full bg-primary text-primary-foreground flex items-center justify-center mb-4 hover:scale-110 transition-transform shadow-lg"
          :class="isFileDragging ? 'scale-110 bg-primary/90' : ''"
        >
          <Upload :size="24" />
        </button>
        <p class="font-medium text-muted-foreground" :class="isFileDragging ? 'text-primary' : ''">
          拖拽 HEIC 文件到此处
        </p>
        <p
          class="text-sm mt-1 opacity-70 text-muted-foreground"
          :class="isFileDragging ? 'font-medium text-primary' : ''"
        >
          支持 .heic, .heif 格式
        </p>
      </div>

      <!-- 空状态：已完成标签页 -->
      <div
        v-if="
          conversionStore.activeTab === 'completed' &&
          conversionStore.completedFiles.length === 0
        "
        class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground/50 pointer-events-none"
      >
        <div
          class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4 border-2 border-dashed"
        >
          <CheckCircle2 />
        </div>
        <p class="font-medium">暂无已完成的文件</p>
        <p class="text-sm mt-1 opacity-70">转换完成的文件将显示在这里</p>
      </div>

      <!-- 待转换标签页：显示任务列表 -->

      <template v-if="conversionStore.activeTab === 'pending'">
        <div v-if="conversionStore.files.length > 0" class="flex-1 h-full overflow-y-auto">
          <div
            ref="taskListRef"
            class="virtual-list h-full"
          >
            <div
              :style="{
                height: `${taskTotalSize}px`,
                width: '100%',
                position: 'relative',
              }"
            >
              <div
                v-for="virtualRow in taskVirtualRows"
                :key="virtualRow.key"
                class="virtual-item"
                :style="{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: '100%',
                  height: `${virtualRow.size}px`,
                  transform: `translateY(${virtualRow.start}px)`,
                }"
              >
                <FileCard
                  :file="taskFiles[virtualRow.index]"
                  :is-task-file="true"
                />
              </div>
            </div>
          </div>
        </div>
      </template>

      <!-- 转换失败标签页：显示所有转换失败的文件 -->

      <template v-if="conversionStore.activeTab === 'error'">
        <!-- 空状态 -->
        <div
          v-if="conversionStore.errorFiles.length === 0"
          class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground/50 pointer-events-none"
        >
          <div
            class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4 border-2 border-dashed"
          >
            <CheckCircle2 />
          </div>
          <p class="font-medium">暂无转换失败的文件</p>
          <p class="text-sm mt-1 opacity-70">转换失败的文件将显示在这里</p>
        </div>

        <!-- 错误文件列表 -->
        <div class="flex-1 h-full overflow-y-auto">
          <div
            ref="errorListRef"
            class="virtual-list h-full"
          >
            <div
              :style="{
                height: `${errorTotalSize}px`,
                width: '100%',
                position: 'relative',
              }"
            >
              <div
                v-for="virtualRow in errorVirtualRows"
                :key="virtualRow.key"
                class="virtual-item"
                :style="{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: '100%',
                  height: `${virtualRow.size}px`,
                  transform: `translateY(${virtualRow.start}px)`,
                }"
              >
                <FileCard
                  :file="errorFiles[virtualRow.index]"
                  :is-error-file="true"
                />
              </div>
            </div>
          </div>
        </div>
      </template>

      <!-- 已完成标签页：显示所有已完成的文件 -->

      <template v-if="conversionStore.activeTab === 'completed'">
        <div class="flex-1 h-full overflow-y-auto">
          <div
            ref="completedListRef"
            class="virtual-list h-full"
          >
            <div
              :style="{
                height: `${completedTotalSize}px`,
                width: '100%',
                position: 'relative',
              }"
            >
              <div
                v-for="virtualRow in completedVirtualRows"
                :key="virtualRow.key"
                class="virtual-item"
                :style="{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: '100%',
                  height: `${virtualRow.size}px`,
                  transform: `translateY(${virtualRow.start}px)`,
                }"
              >
                <FileCard
                  :file="conversionStore.completedFiles[virtualRow.index]"
                  :is-completed-file="true"
                />
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <div v-if="conversionStore.files.length > 0" class="p-2 shrink-0 relative z-10">
      <div class="relative w-full">
        <Button
          variant="default"
          size="default"
          class="w-full h-10 text-sm"
          @click="selectFilesWithDialog"
        >
          <Upload :size="16" class="mr-2" /> 选择文件
        </Button>
      </div>
    </div>
  </section>
</template>
