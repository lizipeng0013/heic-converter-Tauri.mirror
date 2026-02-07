<script setup lang="ts">
import TitleBar from "@/components/layout/TitleBar.vue";
import FileListArea from "@/views/file/FileListArea.vue";
import SettingsPanel from "@/views/settings/SettingsPanel.vue";
import StatusBar from "@/views/status/StatusBar.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
</script>

<template>
  <TooltipProvider>
    <div class="h-screen w-screen flex flex-col overflow-hidden bg-muted text-foreground relative">
      <TitleBar />
      <main class="flex-1 flex overflow-hidden pointer-events-auto p-2 gap-2">
        <FileListArea />
        <SettingsPanel />
      </main>
      <StatusBar />
      <!--    <InputFile />-->
    </div>
  </TooltipProvider>
</template>
