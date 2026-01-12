<script setup lang="ts">
import { useConversionStore } from "@/stores/conversionStore";

const store = useConversionStore();
</script>

<template>
  <div
    class="h-8 px-4 flex items-center justify-between border-t bg-muted/20 text-xs text-muted-foreground"
  >
    <div>
      <span class="mr-4">总计: {{ store.stats.total }}</span>
      <span class="mr-4">已完成: {{ store.stats.done }}</span>
      <span>待处理: {{ store.stats.pending }}</span>
    </div>
    <div v-if="store.isConverting">正在处理中...</div>
    <div v-else-if="store.stats.done > 0 && store.stats.pending === 0">
      全部完成
    </div>
  </div>
</template>
