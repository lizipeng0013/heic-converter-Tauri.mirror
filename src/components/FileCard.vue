<script setup lang="ts">
import { FileImage, Trash2, Loader2, AlertCircle, FolderOpen } from 'lucide-vue-next'
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
  showProgress?: boolean
  showError?: boolean
  showCompleted?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showProgress: false,
  showError: false,
  showCompleted: false
})

const conversionStore = useConversionStore()

const handleOpenFileDir = async () => {
  if (props.file.convertedFilePath) {
    const { revealItemInDir } = await import('@tauri-apps/plugin-opener')
    await revealItemInDir(props.file.convertedFilePath)
  }
}

</script>

<template>
  <Card class="group overflow-hidden transition-colors hover:border-primary/50">
    <CardContent class="p-1 flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 min-w-0 flex-1 max-w-[calc(100%-2rem)]">
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
      
      <div class="flex items-center gap-1 shrink-0">
        <!-- 进度徽章（待转换列表） -->
        <Badge v-if="showProgress" variant="default" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center">
          <Loader2 :size="10" class="animate-spin" />
          {{ file.progress }}%
        </Badge>
        
        <!-- 等待徽章（待转换列表） -->
        <Badge v-if="!showProgress && !showError && !showCompleted" variant="secondary" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-16 justify-center">
          等待
        </Badge>
        
        <!-- 错误徽章（待转换列表） -->
        <Tooltip v-if="showError && file.error">
          <TooltipTrigger as-child>
            <Badge variant="destructive" class="h-5 px-1.5 text-[10px] flex-shrink-0 w-20 justify-center cursor-help">
              <AlertCircle :size="10" /> 失败
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p class="max-w-xs break-words">{{ file.error }}</p>
          </TooltipContent>
        </Tooltip>
        
        <!-- 打开文件按钮（已完成标签页） -->
        <Tooltip v-if="showCompleted">
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
              @click="showCompleted ? conversionStore.removeCompletedFile(file.path) : conversionStore.removePath(file.path)"
            >
              <Trash2 :size="14" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{{ showCompleted ? '从已完成列表中移除' : '从文件队列中移除' }}</p>
          </TooltipContent>
        </Tooltip>
      </div>
    </CardContent>
  </Card>
</template>