import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./style.css"; // 确保有 Shadcn CSS

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);
app.mount("#app");
