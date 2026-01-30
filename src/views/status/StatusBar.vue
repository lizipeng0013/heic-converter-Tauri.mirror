<script setup lang="ts">
import { Clock } from "lucide-vue-next";
import { useConversionStore } from "@/stores/conversionStore";

const store = useConversionStore();
</script>

<template>
  <div
    class="h-8 px-4 flex items-center justify-between border-t bg-muted/20 text-xs text-muted-foreground"
  >
    <div class="flex items-center gap-4">
      <Clock :size="12" />
      <span>总耗时: <span v-if="store.spendTime !== null" class="font-medium">{{ store.spendTime.toFixed(1) }}s</span><span v-else>...</span></span>
    </div>
  </div>
</template>
