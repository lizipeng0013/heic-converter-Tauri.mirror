<script setup lang="ts">
import { ChevronDown } from "@lucide/vue";
import { onUnmounted, ref, watch } from "vue";

export interface DropdownItem {
  value: string;
  label: string;
}

withDefaults(
  defineProps<{
    items: DropdownItem[];
    modelValue: string;
    buttonClass?: string | string[];
    menuClass?: string;
    id?: string;
  }>(),
  { buttonClass: "", menuClass: "", id: undefined }
);

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

const open = ref(false);
const root = ref<HTMLElement | null>(null);

const selectedLabelOf = (value: string, items: DropdownItem[]) =>
  items.find((i) => i.value === value)?.label ?? "";

const toggle = () => {
  open.value = !open.value;
};

const select = (value: string) => {
  emit("update:modelValue", value);
  open.value = false;
};

// 点击菜单与触发按钮之外关闭（组件自管理，调用方无需挂全局监听）
const handleOutside = (event: MouseEvent) => {
  if (root.value && !root.value.contains(event.target as Node)) open.value = false;
};

watch(open, (isOpen) => {
  if (isOpen) document.addEventListener("click", handleOutside);
  else document.removeEventListener("click", handleOutside);
});

onUnmounted(() => document.removeEventListener("click", handleOutside));
</script>

<template>
  <div ref="root" class="relative">
    <button :id="id" :class="buttonClass" @click="toggle">
      <span class="flex-1">
        <slot name="label" :label="selectedLabelOf(modelValue, items)">
          {{ selectedLabelOf(modelValue, items) }}
        </slot>
      </span>
      <ChevronDown :size="14" class="text-muted-foreground shrink-0" />
    </button>
    <div
      v-if="open"
      class="absolute top-full mt-1 bg-popover text-popover-foreground rounded-md border shadow-md z-50"
      :class="menuClass"
    >
      <div class="p-1">
        <button
          v-for="item in items"
          :key="item.value"
          class="w-full text-left px-3 py-2 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground transition-colors"
          :class="{
            'bg-accent text-accent-foreground': modelValue === item.value,
          }"
          @click="select(item.value)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
  </div>
</template>
