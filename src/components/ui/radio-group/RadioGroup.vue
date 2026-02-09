<script setup lang="ts">
import { RadioGroupRoot, useForwardPropsEmits } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { cn } from "@/lib/utils"

interface Props {
  class?: HTMLAttributes["class"]
  modelValue?: string
  defaultValue?: string
  disabled?: boolean
  name?: string
  required?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  "update:modelValue": [value: string]
}>()

const forwarded = useForwardPropsEmits(props, emit)
</script>

<template>
  <RadioGroupRoot v-bind="forwarded" :class="cn('grid gap-2', props.class)">
    <slot />
  </RadioGroupRoot>
</template>