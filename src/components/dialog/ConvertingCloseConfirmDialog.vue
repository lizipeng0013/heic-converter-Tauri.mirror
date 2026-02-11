<script setup lang="ts">
import { ref } from "vue";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { CloseAction } from "@/types";

interface Props {
  open: boolean;
  isClosing?: boolean;
}

interface Emits {
  (e: "update:open", value: boolean): void;
  (e: "confirm", action: CloseAction): void;
}

withDefaults(defineProps<Props>(), {
  isClosing: false,
});
const emit = defineEmits<Emits>();

const selectedAction = ref<CloseAction>("minimize");

const handleConfirm = () => {
  emit("confirm", selectedAction.value);
  emit("update:open", false);
};

const handleCancel = () => {
  emit("confirm", "cancel");
  emit("update:open", false);
};
</script>

<template>
  <Dialog :open="open" @update:open="(value) => emit('update:open', value)">
    <DialogContent class="sm:max-w-[400px]">
      <DialogHeader>
        <DialogTitle>请选择您的操作</DialogTitle>
        <DialogDescription> 转换正在进行中，您希望如何处理？ </DialogDescription>
      </DialogHeader>
      <div class="py-4">
        <div class="flex flex-col gap-3">
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              v-model="selectedAction"
              type="radio"
              value="minimize"
              :disabled="isClosing"
              class="h-4 w-4 cursor-pointer"
            />
            <span class="text-sm font-medium" :class="{ 'opacity-50': isClosing }">
              最小化到系统托盘
            </span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              v-model="selectedAction"
              type="radio"
              value="exit"
              :disabled="isClosing"
              class="h-4 w-4 cursor-pointer"
            />
            <span class="text-sm font-medium" :class="{ 'opacity-50': isClosing }"> 退出 </span>
          </label>
        </div>
      </div>
      <DialogFooter>
        <Button variant="outline" :disabled="isClosing" @click="handleCancel">取消</Button>
        <Button :disabled="isClosing" @click="handleConfirm">确定</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
