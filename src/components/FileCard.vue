<script setup lang="ts">
import { FileImage, Trash2, Loader2, AlertCircle, FolderOpen, Clock } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import { formatSize } from '@/utils'
import { useConversionStore } from '@/stores/conversionStore'
import type { FileItem } from '@/types'
import { ref, computed } from 'vue'

interface Props {
  file: FileItem
  isTaskFile?: boolean  // 是否是任务文件（待转换或正在转换）
  isErrorFile?: boolean // 是否是错误文件
  isCompletedFile?: boolean // 是否是已完成文件
}

const props = withDefaults(defineProps<Props>(), {
  isTaskFile: false,
  isErrorFile: false,
  isCompletedFile: false
})

const conversionStore = useConversionStore()

// 判断是否正在处理中（正在转换或正在停止）
const isProcessing = computed(() =>
  props.isTaskFile && (conversionStore.isConverting || conversionStore.isStopping)
)

const handleOpenFileDir = async () => {
  if (props.file.convertedFilePath) {
    const { revealItemInDir } = await import('@tauri-apps/plugin-opener')
    await revealItemInDir(props.file.convertedFilePath)
  }
}

</script>

<template>
  <Card class="group overflow-hidden transition-colors hover:border-primary/50 mr-3">
    <CardContent class="p-1 flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 min-w-0 flex-1 max-w-[calc(100%-3rem)]">
        <div class="h-7 w-7 shrink-0 rounded bg-secondary flex items-center justify-center text-secondary-foreground">
          <FileImage :size="14" />
        </div>
        <div class="flex flex-col justify-center gap-0.5 overflow-hidden min-w-0 flex-1">
          <div class="flex items-center gap-2">
            <p class="text-sm font-medium truncate flex-1 min-w-0">{{ file.name }}</p>
          </div>

          <div class="text-xs text-muted-foreground">
            {{ formatSize(file.size) }}
          </div>
        </div>
      </div>

      <div class="flex items-center gap-1 shrink-0 mr-2">
        <!-- 处理中徽章（任务文件，且正在转换或停止中） -->
        <Badge v-if="isProcessing" variant="default" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center">
          <Loader2 :size="10" class="animate-spin" />
          处理中
        </Badge>

        <!-- 等待徽章（任务文件，且未开始转换） -->
        <Badge v-if="isTaskFile && !isProcessing" variant="secondary" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-16 justify-center">
          <Clock :size="10" />
          等待
        </Badge>

        <!-- 错误徽章（错误文件） -->
        <Tooltip v-if="isErrorFile && file.error">
          <TooltipTrigger as-child>
            <Badge variant="destructive" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center cursor-help">
              <AlertCircle :size="10" /> 失败
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p class="max-w-xs break-words">{{ file.error }}</p>
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
            <p>打开转换成功的文件所在目录</p>
          </TooltipContent>
        </Tooltip>

        <!-- 删除按钮 -->
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground hover:text-destructive"
              @click="isCompletedFile ? conversionStore.removeCompletedFile(file.path) : (isErrorFile ? conversionStore.removePath(file.path) : conversionStore.removePath(file.path))"
            >
              <Trash2 :size="14" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{{ isCompletedFile ? '从已完成列表中移除' : (isErrorFile ? '从失败列表中移除' : '从任务列表中移除') }}</p>
          </TooltipContent>
        </Tooltip>
      </div>
    </CardContent>
  </Card>
</template>