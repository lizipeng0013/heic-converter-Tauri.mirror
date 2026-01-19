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
      <span>已完成: {{ store.stats.done }}</span>
      <span>待处理: {{ store.stats.pending }}</span>
      <span v-if="store.stats.converting > 0" class="text-primary">转换中: {{ store.stats.converting }}</span>
      <span v-if="store.stats.error > 0" class="text-destructive">失败: {{ store.stats.error }}</span>
      <span v-if="store.spendTime !== null">耗时: {{ store.spendTime }}s</span>
    </div>
    <div v-if="store.isConverting">正在处理中...</div>
    <div v-else-if="store.stats.done > 0 && store.stats.pending === 0 && store.stats.error === 0">
      全部完成
    </div>
    <div v-else-if="store.stats.done > 0 && store.stats.pending === 0 && store.stats.error > 0">
      部分完成
    </div>
  </div>
</template>
