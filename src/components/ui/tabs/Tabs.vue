<script setup lang="ts">
import { provide, ref, type Ref } from 'vue'

const props = defineProps<{
  defaultValue?: string
  modelValue?: string
  class?: string
}>()

const emits = defineEmits<{
  (e: 'update:modelValue', payload: string): void
}>()

const state = ref(props.defaultValue || props.modelValue || '') as Ref<string>

provide('tabs-state', state)

const updateState = (value: string) => {
  state.value = value
  emits('update:modelValue', value)
}

provide('tabs-update-state', updateState)
</script>

<template>
  <div :class="props.class">
    <slot />
  </div>
</template>