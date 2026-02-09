<script setup lang="ts">
import { RadioGroupItem, useForwardPropsEmits } from "reka-ui"
import { cn } from "@/lib/utils"
import { Circle } from "lucide-vue-next"
import type { HTMLAttributes } from "vue"

interface Props {
  class?: HTMLAttributes["class"]
  value: string
  id?: string
  disabled?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  "select": [event: Event]
}>()

const forwarded = useForwardPropsEmits(props, emit)
</script>

<template>
  <RadioGroupItem
    v-bind="forwarded"
    :class="
      cn(
        'aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
        props.class
      )
    "
  >
    <Circle class="h-2.5 w-2.5 fill-current text-current" />
  </RadioGroupItem>
</template>