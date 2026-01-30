<script setup lang="ts">
import { useConversionStore } from "@/stores/conversionStore";

const store = useConversionStore();
</script>

<template>
  <div
    class="h-8 px-4 flex items-center justify-between border-t bg-muted/20 text-xs text-muted-foreground"
  >
    <div class="flex items-center gap-4">
      <span>总计: {{ store.stats.total }}</span>
      <span>已完成: {{ store.stats.completed }}</span>
      <span>待处理: {{ store.stats.waiting }}</span>
      <span v-if="store.stats.processing > 0" class="text-primary">转换中: {{ store.stats.processing }}</span>
      <span v-if="store.stats.failed > 0" class="text-destructive">失败: {{ store.stats.failed }}</span>
      <span v-if="store.spendTime !== null">耗时: {{ store.spendTime }}s</span>
    </div>
    <div v-if="store.isConverting">正在处理中...</div>
    <div v-else-if="store.stats.completed > 0 && store.stats.waiting === 0 && store.stats.failed === 0">
      全部完成
    </div>
    <div v-else-if="store.stats.completed > 0 && store.stats.waiting === 0 && store.stats.failed > 0">
      部分完成
    </div>
  </div>
</template>
