<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  class?: string
}>()

const computedClass = computed(() => {
  return `inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground ${props.class || ''}`
})
</script>

<template>
  <div :class="computedClass">
    <slot />
  </div>
</template>