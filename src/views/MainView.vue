<script setup lang="ts">
import TitleBar from '@/components/layout/TitleBar.vue'
import FileListArea from '@/views/file/FileListArea.vue'
import SettingsPanel from '@/views/settings/SettingsPanel.vue'
import StatusBar from '@/views/status/StatusBar.vue'

</script>

<template>
  <div class="h-screen w-screen flex flex-col overflow-hidden bg-background text-foreground relative">
    <TitleBar />
    <main class="flex-1 flex overflow-hidden pointer-events-auto">
      <FileListArea />
      <SettingsPanel />
    </main>
    <StatusBar />
  </div>
</template>