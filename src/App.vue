<script setup lang="ts">
import MainView from '@/views/MainView.vue'
</script>

<template>
  <div id="app" class="app-container">
    <MainView />
  </div>
</template>

<style>
/* 全局容器样式（可选，通常在 main.css 处理） */
.app-container {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}
</style>