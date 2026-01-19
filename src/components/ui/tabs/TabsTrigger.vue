<script setup lang="ts">
import { inject, computed, type Ref } from 'vue'

const props = defineProps<{
  value: string
  class?: string
}>()

const state = inject<Ref<string>>('tabs-state')
const updateState = inject<(value: string) => void>('tabs-update-state')

const isActive = computed(() => state?.value === props.value)

const handleClick = () => {
  if (updateState) {
    updateState(props.value)
  }
}

const computedClass = computed(() => {
  const baseClasses = 'inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50'
  const activeClasses = 'bg-background text-foreground shadow'
  const inactiveClasses = 'text-muted-foreground hover:text-foreground'
  return `${baseClasses} ${isActive.value ? activeClasses : inactiveClasses} ${props.class || ''}`
})
</script>

<template>
  <button
    type="button"
    :class="computedClass"
    @click="handleClick"
  >
    <slot />
  </button>
</template>