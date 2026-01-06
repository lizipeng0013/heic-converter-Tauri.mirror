<script setup>
import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
import heic2any from "heic2any";

// const greetMsg = ref("");
// const name = ref("");
// --- 状态数据 ---
const files = ref([]);
const targetFormat = ref('image/jpeg');
const isDragOver = ref(false);
const isProcessing = ref(false);
const toasts = ref([])

// --- 辅助函数 ---
const uid = () => Date.now().toString(36) + Math.random().toString(36).substr(2);

const showToast = (message, type = 'success') => {
  const id = uid();
  toasts.value.push({ id, message, type });
  setTimeout(() => {
    toasts.value = toasts.value.filter(t => t.id !== id);
  }, 3000);
};

// --- 文件处理逻辑 ---
const handleFileSelect = (event) => {
  const selected = Array.from(event.target.files);
  addFiles(selected);
  event.target.value = ''; // 重置 input 以便重复选择同一文件
};

const handleDrop = (event) => {
  isDragOver.value = false;
  const dropped = Array.from(event.dataTransfer.files);
  addFiles(dropped);
};

const addFiles = (newFiles) => {
  newFiles.forEach(file => {
    // 简单的文件类型过滤
    const isHeic = file.name.toLowerCase().endsWith('.heic') || 
                   file.name.toLowerCase().endsWith('.heif') ||
                   file.type === 'image/heic' || 
                   file.type === 'image/heif';
    
    if (isHeic) {
      files.value.push({
        id: uid(),
        originalFile: file,
        status: 'pending', // pending, processing, done, error
        blob: null,
        preview: `https://picsum.photos/seed/${uid()}/48/48.jpg` // 占位预览图
      });
    } else {
      showToast(`跳过非 HEIC 文件: ${file.name}`, 'error');
    }
  });
};

const removeFile = (index) => {
  files.value.splice(index, 1);
};

const clearFiles = () => {
  files.value = [];
};

// --- 转换核心逻辑 ---
const startConversion = async () => {
  if (files.value.length === 0) return;
  
  isProcessing.value = true;
  // 只处理待处理或之前失败的文件
  const pendingFiles = files.value.filter(f => f.status === 'pending' || f.status === 'error');
  
  for (const fileItem of pendingFiles) {
    fileItem.status = 'processing';
    
    try {
      // 调用 heic2any 库进行转换
      const resultBlob = await heic2any({
        blob: fileItem.originalFile,
        toType: targetFormat.value,
        quality: 0.8
      });
      
      // heic2any 返回的可能是单个 blob 或 array (如果是多页图)，这里取第一个
      fileItem.blob = Array.isArray(resultBlob) ? resultBlob[0] : resultBlob;
      fileItem.status = 'done';
      
      // 更新预览图为转换后的图（本地 Blob URL）
      fileItem.preview = URL.createObjectURL(fileItem.blob);
      
    } catch (err) {
      console.error('转换失败:', err);
      fileItem.status = 'error';
      // 稍微延时避免 UI 卡顿
      await new Promise(r => setTimeout(r, 100)); 
    }
  }
  
  isProcessing.value = false;
  showToast('批量转换完成！');
};

// --- 下载逻辑 ---
const downloadFile = (fileItem) => {
  if (!fileItem.blob) return;
  
  const ext = targetFormat.value === 'image/jpeg' ? 'jpg' : 'png';
  const newName = fileItem.originalFile.name.replace(/\.[^/.]+$/, "") + `.${ext}`;
  
  const link = document.createElement('a');
  link.href = URL.createObjectURL(fileItem.blob);
  link.download = newName;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};

// --- UI 状态辅助 ---
const getStatusClass = (status) => {
  switch(status) {
    case 'pending': return 'status-pending';
    case 'processing': return 'status-processing';
    case 'done': return 'status-success';
    case 'error': return 'status-error';
    default: return '';
  }
};

const getStatusText = (status) => {
  switch(status) {
    case 'pending': return '待处理';
    case 'processing': return '转换中';
    case 'done': return '完成';
    case 'error': return '失败';
    default: return '';
  }
};

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<template>

  <div class="app-container">
    <header>
      <h1>
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
        HEIC Converter
        <span class="badge">Tauri App</span>
      </h1>
      <div style="font-size: 0.8rem; color: var(--text-muted);">
        基于 Vue 3 + heic2any
      </div>
    </header>

    <main>
      <!-- 左侧：控制面板 -->
      <aside class="controls">
        <!-- 格式选择 -->
        <div class="control-group">
          <h3>输出格式</h3>
          <div class="format-options">
            <label class="radio-label">
              <input type="radio" value="image/jpeg" v-model="targetFormat">
              <div class="radio-tile">
                <span class="format-icon">JPG</span>
              </div>
            </label>
            <label class="radio-label">
              <input type="radio" value="image/png" v-model="targetFormat">
              <div class="radio-tile">
                <span class="format-icon">PNG</span>
              </div>
            </label>
          </div>
        </div>

        <!-- 上传区域 -->
        <div class="control-group" style="flex: 1; display: flex; flex-direction: column;">
          <h3>添加文件</h3>
          <div 
            class="drop-zone" 
            :class="{ 'drag-over': isDragOver }"
            @dragover.prevent="isDragOver = true"
            @dragleave.prevent="isDragOver = false"
            @drop.prevent="handleDrop"
          >
            <input type="file" class="drop-input" multiple accept=".heic, .heif" @change="handleFileSelect">
            <div class="drop-icon">📁</div>
            <div class="drop-text">
              <strong>点击或拖拽</strong><br>
              HEIC/HEIF 文件到此处
            </div>
          </div>
          
          <div style="margin-top: 1rem; display: flex; gap: 0.5rem;">
            <button class="btn btn-primary" @click="startConversion" :disabled="files.length === 0 || isProcessing">
              <span v-if="isProcessing">转换中...</span>
              <span v-else>开始批量转换</span>
            </button>
          </div>
          <button class="btn btn-secondary" @click="clearFiles" v-if="files.length > 0">清空列表</button>
        </div>
      </aside>

      <!-- 右侧：文件列表 -->
      <section class="file-list-container">
        <div class="list-header">
          <span>待处理文件 ({{ files.length }})</span>
          <span style="font-size: 0.8rem; font-weight: normal; color: var(--text-muted);">
            目标: {{ targetFormat === 'image/jpeg' ? 'JPG' : 'PNG' }}
          </span>
        </div>

        <div class="file-list" v-if="files.length > 0">
          <div class="file-item" v-for="(file, index) in files" :key="file.id">
            <img :src="file.preview" class="file-preview" alt="preview">
            
            <div class="file-info">
              <div class="file-name" :title="file.originalFile.name">{{ file.originalFile.name }}</div>
              <div class="file-meta">
                <span>{{ (file.originalFile.size / 1024).toFixed(1) }} KB</span>
                <span v-if="file.status === 'done'">→ {{ (file.blob.size / 1024).toFixed(1) }} KB</span>
              </div>
              <div class="progress-bar-container" v-if="file.status === 'processing'">
                <div class="progress-bar" style="width: 50%; animation: pulse 1s infinite;"></div>
              </div>
            </div>

            <div class="file-status" :class="getStatusClass(file.status)">
              {{ getStatusText(file.status) }}
            </div>

            <button 
              class="download-btn" 
              @click="downloadFile(file)" 
              title="下载转换后的文件"
              v-if="file.status === 'done'"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                <polyline points="7 10 12 15 17 10"></polyline>
                <line x1="12" y1="15" x2="12" y2="3"></line>
              </svg>
            </button>
            <button 
              class="download-btn" 
              @click="removeFile(index)" 
              title="移除"
              v-else
              style="border-color: transparent; color: var(--text-muted);"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </div>
        </div>

        <div class="empty-state" v-else>
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#334155" stroke-width="1.5">
            <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"></path>
            <polyline points="13 2 13 9 20 9"></polyline>
          </svg>
          <p>暂无文件，请添加 HEIC 图片</p>
        </div>
      </section>
    </main>

    <!-- 消息提示 -->
    <div class="toast-container">
      <div class="toast" :class="toast.type" v-for="toast in toasts" :key="toast.id">
        <span v-if="toast.type === 'success'">✅</span>
        <span v-else>⚠️</span>
        <span>{{ toast.message }}</span>
      </div>
    </div>
  </div>

  <!-- <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main> -->

</template>

<style scoped>

/* CSS 变量定义 */
.app-container {
  --primary: #3b82f6;
  --primary-hover: #2563eb;
  --bg-color: #0f172a;
  --card-bg: #1e293b;
  --text-main: #f1f5f9;
  --text-muted: #94a3b8;
  --border-color: #334155;
  --success: #10b981;
  --error: #ef4444;
  --radius: 8px;

  font-family: 'Inter', sans-serif;
  background-color: var(--bg-color);
  color: var(--text-main);
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Header */
header {
  padding: 1rem 2rem;
  background-color: var(--card-bg);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

h1 { font-size: 1.25rem; font-weight: 600; display: flex; align-items: center; gap: 0.5rem; }
.badge { background: var(--primary); font-size: 0.75rem; padding: 0.2rem 0.5rem; border-radius: 4px; }

/* Main Layout */
main {
  flex: 1;
  display: flex;
  padding: 2rem;
  gap: 2rem;
  overflow: hidden;
}

/* Controls Section */
.controls {
  flex: 0 0 300px;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.control-group {
  background: var(--card-bg);
  padding: 1.5rem;
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
}

.control-group h3 { font-size: 0.9rem; color: var(--text-muted); margin-bottom: 1rem; text-transform: uppercase; letter-spacing: 0.05em; }

/* Format Selection */
.format-options { display: flex; gap: 1rem; }
.radio-label {
  flex: 1;
  cursor: pointer;
  position: relative;
}
.radio-label input { position: absolute; opacity: 0; }
.radio-tile {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1rem;
  background: var(--bg-color);
  border: 2px solid var(--border-color);
  border-radius: var(--radius);
  transition: all 0.2s;
}
.radio-label input:checked + .radio-tile {
  border-color: var(--primary);
  background: rgba(59, 130, 246, 0.1);
}
.format-icon { font-size: 1.5rem; margin-bottom: 0.5rem; font-weight: bold; }

/* Drop Zone */
.drop-zone {
  flex: 1;
  border: 2px dashed var(--border-color);
  border-radius: var(--radius);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  transition: all 0.2s;
  cursor: pointer;
  min-height: 200px;
  position: relative;
}
.drop-zone:hover, .drop-zone.drag-over {
  border-color: var(--primary);
  background: rgba(59, 130, 246, 0.05);
}
.drop-icon { font-size: 3rem; color: var(--text-muted); margin-bottom: 1rem; }
.drop-text { color: var(--text-muted); }
.drop-input { position: absolute; width: 100%; height: 100%; opacity: 0; cursor: pointer; }

/* Action Buttons */
.btn {
  width: 100%;
  padding: 0.75rem;
  border: none;
  border-radius: var(--radius);
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}
.btn-primary { background: var(--primary); color: white; }
.btn-primary:hover { background: var(--primary-hover); }
.btn-primary:disabled { background: var(--border-color); cursor: not-allowed; opacity: 0.6; }
.btn-secondary { background: var(--border-color); color: var(--text-main); margin-top: 0.5rem; }
.btn-secondary:hover { background: #475569; }

/* File List */
.file-list-container {
  flex: 1;
  background: var(--card-bg);
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.list-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
}
.file-list {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* File Item Card */
.file-item {
  display: flex;
  align-items: center;
  background: var(--bg-color);
  padding: 0.75rem;
  border-radius: var(--radius);
  border: 1px solid var(--border-color);
  gap: 1rem;
  animation: fadeIn 0.3s ease;
}
.file-preview {
  width: 48px;
  height: 48px;
  background: #334155;
  border-radius: 4px;
  object-fit: cover;
}
.file-info { flex: 1; min-width: 0; }
.file-name { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-weight: 500; font-size: 0.9rem; }
.file-meta { font-size: 0.75rem; color: var(--text-muted); display: flex; gap: 0.5rem; margin-top: 0.25rem; }
.file-status {
  font-size: 0.8rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-weight: 500;
  min-width: 60px;
  text-align: center;
}
.status-pending { background: var(--border-color); color: var(--text-muted); }
.status-processing { background: #eab308; color: black; }
.status-success { background: rgba(16, 185, 129, 0.2); color: var(--success); }
.status-error { background: rgba(239, 68, 68, 0.2); color: var(--error); }
.download-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-main);
  width: 32px;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
.download-btn:hover { border-color: var(--primary); color: var(--primary); }

/* Progress Bar */
.progress-bar-container {
  width: 100%;
  height: 4px;
  background: var(--border-color);
  margin-top: 0.5rem;
  border-radius: 2px;
  overflow: hidden;
}
.progress-bar { height: 100%; background: var(--primary); transition: width 0.3s ease; }

/* Toast Notification */
.toast-container {
  position: fixed;
  bottom: 2rem;
  right: 2rem;
  z-index: 100;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.toast {
  background: var(--card-bg);
  border: 1px solid var(--border-color);
  padding: 1rem;
  border-radius: var(--radius);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.5);
  animation: slideIn 0.3s ease;
  min-width: 250px;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
.toast.success { border-left: 4px solid var(--success); }
.toast.error { border-left: 4px solid var(--error); }

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
  gap: 1rem;
}

@keyframes fadeIn { from { opacity: 0; transform: translateY(5px); } to { opacity: 1; transform: translateY(0); } }
@keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }


/* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
} */

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
