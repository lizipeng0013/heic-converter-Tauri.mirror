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

const selectedAction = ref<CloseAction>("hideToTray");

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
        <DialogTitle>{{ $t("dialog.chooseActionTitle") }}</DialogTitle>
        <DialogDescription> {{ $t("dialog.convertingBody") }} </DialogDescription>
      </DialogHeader>
      <div class="py-4">
        <div class="flex flex-col gap-3">
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              v-model="selectedAction"
              type="radio"
              value="hideToTray"
              :disabled="isClosing"
              class="h-4 w-4 cursor-pointer"
            />
            <span class="text-sm font-medium" :class="{ 'opacity-50': isClosing }">
              {{ $t("dialog.hideToTray") }}
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
            <span class="text-sm font-medium" :class="{ 'opacity-50': isClosing }">
              {{ $t("dialog.quit") }}
            </span>
          </label>
        </div>
      </div>
      <DialogFooter>
        <Button variant="outline" :disabled="isClosing" @click="handleCancel">{{
          $t("dialog.cancel")
        }}</Button>
        <Button :disabled="isClosing" @click="handleConfirm">{{ $t("dialog.confirm") }}</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
