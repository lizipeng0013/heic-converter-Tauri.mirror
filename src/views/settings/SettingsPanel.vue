<script setup lang="ts">
import { useConversionStore } from "@/stores/conversionStore";
import { Settings2, FolderOpen } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Separator } from "@/components/ui/separator";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { downloadDir } from "@tauri-apps/api/path";

const store = useConversionStore();

const selectOutputFolder = async () => {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: "选择输出目录",
    });
    if (result) store.setOutputFolder(result);
  } catch (e) {
    console.error(e);
  }
};

const openOutputFolder = async () => {
  try {
    const folderPath = store.outputFolder || (await downloadDir());
    await openPath(folderPath);
  } catch (error) {
    console.error("打开目录失败：", error);
    const folderPath =
      store.outputFolder || (await downloadDir().catch(() => "下载目录"));
    alert(`目录路径：${folderPath}\n请手动在文件管理器中打开此目录`);
  }
};
</script>

<template>
  <aside class="w-80 border-l bg-card flex flex-col shrink-0 h-full">
    <div class="p-6 flex-1 overflow-y-auto">
      <h2
        class="text-lg font-semibold tracking-tight mb-6 flex items-center gap-2"
      >
        <Settings2 :size="18" class="text-muted-foreground" /> 转换设置
      </h2>
      <div class="space-y-6">
        <div class="space-y-3">
          <label class="text-sm font-medium leading-none">目标格式</label>
          <div class="grid grid-cols-2 gap-2 bg-muted p-1 rounded-md">
            <button
              @click="store.updateSettings({ format: 'jpeg' })"
              class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              :class="
                store.settings.format === 'jpeg'
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
              "
            >
              JPG / JPEG
            </button>
            <button
              @click="store.updateSettings({ format: 'png' })"
              class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              :class="
                store.settings.format === 'png'
                  ? 'bg-background text-foreground shadow-sm'
                  : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
              "
            >
              PNG
            </button>
          </div>
        </div>
        <Separator />
        <div class="space-y-4">
          <div class="flex justify-between items-center">
            <label class="text-sm font-medium leading-none">图片质量</label>
            <span class="text-sm font-mono text-muted-foreground"
              >{{ store.settings.quality[0] }}%</span
            >
          </div>
          <Slider
            :model-value="store.settings.quality"
            @update:model-value="(v) => store.updateSettings({ quality: v })"
            :max="100"
            :min="10"
            :step="5"
            class="w-full"
          />
          <p class="text-xs text-muted-foreground leading-relaxed">
            更高的质量将导致文件体积变大。此选项仅对 JPG 格式有效。
          </p>
        </div>
        <Separator />
        <div class="space-y-3">
          <label
            class="text-sm font-medium leading-none flex items-center justify-between"
            >输出目录</label
          >
          <Button
            variant="outline"
            size="sm"
            class="w-full justify-start gap-2"
            @click="selectOutputFolder"
            ><FolderOpen :size="16" />{{
              store.outputFolder || "选择输出目录"
            }}</Button
          >
          <p class="text-xs text-muted-foreground mt-1">
            未选择时，将保存到系统的“下载”文件夹。
          </p>
        </div>
      </div>
    </div>
    <div class="p-6 border-t bg-muted/20">
      <Button
        @click="store.startConversion"
        :disabled="store.isConverting"
        class="w-full h-11 text-base"
      >
        开始批量转换
      </Button>
    </div>
  </aside>
</template>
