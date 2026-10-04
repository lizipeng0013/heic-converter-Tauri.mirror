import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { i18n, initLocale } from "./i18n";
import "./style.css"; // 确保有 Shadcn CSS

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);
app.use(i18n);
initLocale();
app.mount("#app");
