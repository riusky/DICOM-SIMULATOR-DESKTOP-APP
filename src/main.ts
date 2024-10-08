import { createApp } from "vue";
import App from "./App.vue";
import { useI18n } from "@/plugins/i18n";
import { getPlatformConfig } from "./config";
import { injectResponsiveStorage } from "@/utils/responsive";
import { setupStore } from "@/store";
import router from "./router";
// 引入重置样式
import "./style/reset.scss";
// 导入公共样式
import "./style/index.scss";
// 一定要在main.ts中导入tailwind.css，防止vite每次hmr都会请求src/style/index.scss整体css文件导致热更新慢的问题
import "./style/tailwind.css";

const app = createApp(App);
getPlatformConfig(app).then(async (config) => {
  setupStore(app);
  app.use(router);
  await router.isReady();
  injectResponsiveStorage(app, config);
  app.use(useI18n);
  console.log(config);
  app.mount("#app");
});
