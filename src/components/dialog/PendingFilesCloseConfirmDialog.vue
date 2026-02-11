<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

interface Props {
  open: boolean;
  isClosing?: boolean;
}

interface Emits {
  (e: "update:open", value: boolean): void;
  (e: "confirm"): void;
  (e: "cancel"): void;
}

withDefaults(defineProps<Props>(), {
  isClosing: false,
});
const emit = defineEmits<Emits>();

const handleConfirm = () => {
  emit("confirm");
  emit("update:open", false);
};

const handleCancel = () => {
  emit("cancel");
  emit("update:open", false);
};
</script>

<template>
  <Dialog :open="open" @update:open="(value) => emit('update:open', value)">
    <DialogContent class="sm:max-w-[400px]">
      <DialogHeader>
        <DialogTitle>确认关闭</DialogTitle>
        <DialogDescription> 任务队列中仍有文件未处理，确认要关闭窗口吗？ </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button variant="outline" :disabled="isClosing" @click="handleCancel">取消</Button>
        <Button :disabled="isClosing" @click="handleConfirm">关闭</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
