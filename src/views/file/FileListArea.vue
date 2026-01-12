<script setup lang="ts">
import { ref } from 'vue'
import { useConversionStore } from '@/stores/conversionStore'
import { formatSize } from '@/utils'
import { Upload, Trash2, FileImage, CheckCircle2, Loader2, FolderOpen } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { revealItemInDir } from '@tauri-apps/plugin-opener'
import { FileItem } from '@/types/index'

const convertionStore = useConversionStore()
const isFileDragging = ref(false)

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  isFileDragging.value = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    convertionStore.addFiles(files)
  }
}

const handleDragEnter = (e: DragEvent) => {
  e.preventDefault()
  isFileDragging.value = true
}

const handleDragLeave = (e: DragEvent) => {
  e.preventDefault()
  isFileDragging.value = false
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
}

const handleInputFiles = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    convertionStore.addFiles(target.files)
  }
}

const handleOpenFileDir = async (file: FileItem) => {
  try {
    await revealItemInDir(file.convertedFilePath)
  } catch (error) {
    console.error('打开目录并定位文件失败：', error)
  }
}

</script>

<template>

  <section class="flex-1 flex flex-col border-r min-w-0 bg-muted/10 h-full relative z-0">

    <div class="h-12 px-4 flex items-center justify-between border-b shrink-0">
      <h3 class="text-sm font-medium text-muted-foreground">文件队列 ({{ convertionStore.stats.total }})</h3>
      <Button v-if="convertionStore.stats.total > 0" variant="ghost" size="sm" class="h-8 text-xs" @click="convertionStore.clearFiles">清空列表</Button>
    </div>

    <div class="flex-1 w-full overflow-y-auto p-2 space-y-1"
      @dragenter="handleDragEnter"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
      :class="{
        'bg-primary/10': isFileDragging,       // 【背景】明显变蓝
      }"
    >
      <div v-if="convertionStore.stats.total === 0" class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground/50 pointer-events-none">
        <div class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4 border-2 border-dashed">
          <Upload />
        </div>
        <p class="font-medium" :class="isFileDragging ? 'text-primary' : ''">拖拽 HEIC 文件到此处</p>
        <p class="text-sm mt-1 opacity-70" :class="isFileDragging ? 'font-medium text-primary' : ''">支持 .heic, .heif 格式</p>
      </div>

      <!-- TODO: 待增加快捷打开目录的按钮 -->
      <Card v-for="file in convertionStore.files" :key="file.id" class="group overflow-hidden transition-colors hover:border-primary/50">
        <CardContent class="p-1 flex items-center justify-between gap-2">
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <div class="h-7 w-7 shrink-0 rounded bg-secondary flex items-center justify-center text-secondary-foreground">
              <FileImage :size="14" />
            </div>
            <div class="flex flex-col justify-center gap-0.5 overflow-hidden">
              <div class="flex items-center gap-2">
                <p class="text-sm font-medium truncate">{{ file.name }}</p>
                <Badge :variant="file.status === 'done' ? 'default' : 'secondary'" class="h-5 px-1.5 text-[10px] flex-shrink-0">
                  <span v-if="file.status === 'done'" class="flex items-center gap-0.5"><CheckCircle2 :size="10" /> 完成</span>
                  <span v-else-if="file.status === 'converting'" class="flex items-center gap-0.5 text-yellow-500 dark:text-yellow-400"><Loader2 :size="10" class="animate-spin" /> {{ file.progress }}%</span>
                  <span v-else>等待</span>
                </Badge>
              </div>
              <div class="text-xs text-muted-foreground">{{ formatSize(file.size) }}</div>
              <div v-if="file.status === 'converting'" class="h-1.5 w-full bg-secondary rounded-full overflow-hidden">
                <div class="h-full bg-primary transition-all duration-300 ease-out" :style="{ width: file.progress + '%' }"></div>
              </div>
            </div>
          </div>
          <div class="flex items-center gap-1 shrink-0">
            <Button 
              v-if="file.convertedFilePath"
              variant="ghost" size="icon" class="h-7 w-7 opacity-0 group-hover:opacity-100 text-slate-500 hover:text-primary dark:hover:text-primary transition-colors" 
              title="打开转换成功的文件所在目录"
              @click="handleOpenFileDir(file)"
            >
              <FolderOpen :size="14" />
            </Button>
            <Button 
              variant="ghost" size="icon" class="h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground hover:text-destructive" 
              title="从文件队列中移除"
              @click="convertionStore.removeFile(file.id)"
            >
              <Trash2 :size="14" />
            </Button>
          </div>
        </CardContent>
      </Card>

    </div>

    <div class="p-2 border-t bg-card shrink-0 relative z-10" >
      <div class="relative w-full">
        <input type="file" multiple accept=".heic,.heif" class="hidden" id="fileInput" ref="fileInput" @change="handleInputFiles">
        <label for="fileInput" class="cursor-pointer flex items-center justify-center w-full h-10 rounded-md border border-input bg-background px-8 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground">
          <Upload :size="16" class="mr-2" /> 选择文件
        </label>
      </div>
    </div>

  </section>
</template>