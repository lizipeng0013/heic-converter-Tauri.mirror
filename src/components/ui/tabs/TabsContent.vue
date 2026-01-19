<script setup lang="ts">
import { inject, computed, type Ref } from 'vue'

const props = defineProps<{
  value: string
  class?: string
}>()

const state = inject<Ref<string>>('tabs-state')

const isActive = computed(() => state?.value === props.value)

const computedClass = computed(() => {
  return `ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ${props.class || ''}`
})
</script>

<template>
  <div
    v-if="isActive"
    :class="computedClass"
  >
    <slot />
  </div>
</template>