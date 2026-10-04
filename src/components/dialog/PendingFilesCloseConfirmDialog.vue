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
        <DialogTitle>{{ $t("dialog.closeTitle") }}</DialogTitle>
        <DialogDescription> {{ $t("dialog.pendingCloseBody") }} </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button variant="outline" :disabled="isClosing" @click="handleCancel">{{
          $t("dialog.cancel")
        }}</Button>
        <Button :disabled="isClosing" @click="handleConfirm">{{ $t("dialog.close") }}</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
