<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Popover, PopoverButton, PopoverPanel } from "@headlessui/vue";
import {
  ChevronDownIcon,
  Bars3Icon,
  ArrowLongLeftIcon,
  ArrowsPointingOutIcon,
  XMarkIcon,
  MinusIcon,
  Cog8ToothIcon,
  EyeDropperIcon as SolidEyeDropperIcon,
} from "@heroicons/vue/20/solid";
import { EyeDropperIcon as OutlineEyeDropperIcon } from "@heroicons/vue/24/outline";
import { CommandLineIcon } from "@heroicons/vue/24/outline";
import ThemeChange from "@/components/ThemeChange.vue";
import LanguageSwitcher from "@/components/LanguageSwitcher.vue";
import { useTranslationLang } from "@/layout/hooks/useTranslationLang";
import { Window } from "@tauri-apps/api/window";
import { useSidebarStore } from "@/store/modules/sidebar";
import { invoke } from "@tauri-apps/api/core";

// 定义日志内容的响应式变量
const logContent = ref("");
const showLogModal = ref(false);

// 读取日志文件的方法
const loadLogFile = async () => {
  try {
    logContent.value = ""; // 重置日志内容

    // 调用 Rust 后端的命令来读取日志文件
    const content = (await invoke("read_log_file")) as string;

    logContent.value = content; // 将日志内容赋值到 logContent
  } catch (error) {
    console.error("Error reading log file:", error);
    logContent.value = "Failed to load log file content.";
  }
};

// 打开日志模态框并加载日志内容
const openLogModal = async () => {
  await loadLogFile();
  showLogModal.value = true;
};

// 关闭日志模态框
const closeLogModal = () => {
  showLogModal.value = false;
};

const sidebarStore = useSidebarStore();

const closeSidebar = () => {
  if (sidebarStore.isClosed) {
    sidebarStore.openSidebar();
  } else {
    sidebarStore.closeSidebar();
  }
};
const { t } = useTranslationLang();
const route = useRoute();
const router = useRouter();
const appWindow = Window.getCurrent(); // 获取当前窗口实例
let onTop = ref(false);

const goBack = () => {
  router.back(); // 返回上一个路由
};

// 添加窗口控制功能
const minimizeWindow = () => {
  appWindow.minimize();
};

const toggleMaximizeWindow = () => {
  appWindow.toggleMaximize();
};

const closeWindow = () => {
  appWindow.close();
};

const setAlwaysOnTop = () => {
  onTop.value = !onTop.value;
  appWindow.setAlwaysOnTop(onTop.value);
};
</script>

<template>
  <!-- Navbar -->
  <nav
    data-tauri-drag-region
    class="items-center content-center bg-base-100 dark:bg-base-200"
  >
    <Bars3Icon class="size-6 z-30" @click="closeSidebar" />
    <div
      data-tauri-drag-region
      class="w-full flex flex-nowrap justify-end items-center content-center"
    >
      <div
        v-if="$route.meta.showBackButton"
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        aria-label="Language"
        @click="goBack"
      >
        <ArrowLongLeftIcon class="ml-1 mr-1 h-6 w-6 size-6" />
      </div>
      <Popover v-slot="{ open }" class="relative">
        <PopoverButton
          :class="open ? 'text-neutral-content' : 'text-neutral-content'"
          class="group btn-sm inline-flex items-center rounded-md bg-neutral px-3 py-2 text-base font-medium focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
        >
          <span>{{ t("buttons.theme") }}</span>
          <ChevronDownIcon
            :class="open ? 'text-neutral-content' : 'text-neutral-content'"
            class="ml-1 h-6 w-6 size-6 transition duration-150 ease-in-out"
            aria-hidden="true"
          />
        </PopoverButton>

        <transition
          enter-active-class="transition duration-200 ease-out"
          enter-from-class="translate-y-1 opacity-0"
          enter-to-class="translate-y-0 opacity-100"
          leave-active-class="transition duration-150 ease-in"
          leave-from-class="translate-y-0 opacity-100"
          leave-to-class="translate-y-1 opacity-0"
        >
          <PopoverPanel
            class="absolute z-10 -translate-x-1/2 transform px-4 sm:px-0 lg:max-w-3xl"
          >
            <ThemeChange :themes="[]" />
          </PopoverPanel>
        </transition>
      </Popover>
      <div
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        @click="setAlwaysOnTop"
      >
        <SolidEyeDropperIcon v-show="onTop" class="ml-1 mr-1 h-6 w-6 size-6" />
        <OutlineEyeDropperIcon
          v-show="!onTop"
          class="ml-1 mr-1 h-6 w-6 size-6"
        />
      </div>
      <div
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        @click="openLogModal"
      >
        <CommandLineIcon class="ml-1 mr-1 h-6 w-6 size-6" />
      </div>
      <LanguageSwitcher />
      <!-- 窗口控制按钮 -->
      <div
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        @click="minimizeWindow"
      >
        <MinusIcon class="ml-1 h-6 w-6 size-6" />
      </div>
      <div
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        @click="toggleMaximizeWindow"
      >
        <ArrowsPointingOutIcon class="ml-1 h-6 w-6 size-6" />
      </div>
      <div
        tabindex="0"
        role="button"
        class="btn btn-ghost pl-2 pr-2"
        @click="closeWindow"
      >
        <XMarkIcon class="ml-1 h-6 w-6 size-6" />
      </div>
    </div>
  </nav>

  <!-- 日志内容模态框 -->
  <div v-if="showLogModal" class="modal modal-open">
    <div
      class="modal-box max-w-4xl w-full transform translate-x-[100px]"
      style="right: 0"
    >
      <h3 class="font-bold text-lg">Log File Content</h3>
      <pre class="whitespace-pre-wrap overflow-y-auto h-96 max-h-[70vh]">
      {{ logContent }}
    </pre
      >
      <div class="modal-action">
        <button class="btn" @click="closeLogModal">Close</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
nav {
  position: sticky;
  top: 0;
  left: 0;
  z-index: 10;
  display: flex;
  grid-gap: 24px;
  align-items: center;
  height: 46px;
  padding: 0 6px 0 0;
}

nav::before {
  position: absolute;
  bottom: -40px;
  left: 0;
  width: 40px;
  height: 40px;
  content: "";
  border-radius: 50%;
  box-shadow: -20px -20px 0 oklch(var(--b1));
}
</style>
