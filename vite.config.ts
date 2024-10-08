import { type UserConfigExport, type ConfigEnv, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import {
  root,
  alias,
  pathResolve,
  wrapperEnv,
  __APP_INFO__,
} from "./build/utils";
const host = process.env.TAURI_DEV_HOST;
import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";

// https://vitejs.dev/config/
export default ({ mode }: ConfigEnv): UserConfigExport => {
  wrapperEnv(loadEnv(mode, root));
  return {
    plugins: [
      vue(),
      VueI18nPlugin({
        jitCompilation: false,
        include: [pathResolve("../locales/**")],
      }),
    ],
    root,
    resolve: {
      alias,
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
            protocol: "ws",
            host,
            port: 1421,
          }
        : undefined,
      warmup: {
        clientFiles: ["./index.html", "./src/{views,components}/*"],
      },
      watch: {
        // 3. tell vite to ignore watching `src-tauri`
        ignored: ["**/src-tauri/**"],
      },
    },
    build: {
      // https://cn.vitejs.dev/guide/build.html#browser-compatibility
      target: "es2015",
      sourcemap: false,
      // 消除打包大小超过500kb警告
      chunkSizeWarningLimit: 4000,
      rollupOptions: {
        input: {
          index: pathResolve("./index.html", import.meta.url),
        },
        // 静态资源分类打包
        output: {
          chunkFileNames: "static/js/[name]-[hash].js",
          entryFileNames: "static/js/[name]-[hash].js",
          assetFileNames: "static/[ext]/[name]-[hash].[ext]",
        },
      },
    },
    define: {
      __INTLIFY_PROD_DEVTOOLS__: false,
      __APP_INFO__: JSON.stringify(__APP_INFO__),
    },
  };
};
