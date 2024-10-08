<script setup lang="ts">
import { Squares2X2Icon } from "@heroicons/vue/24/solid";
import { useTranslationLang } from "@/layout/hooks/useTranslationLang";
import { useRoute } from "vue-router";
import { LifebuoyIcon } from "@heroicons/vue/24/solid";
import { Square3Stack3DIcon } from "@heroicons/vue/20/solid";
import { Cog6ToothIcon } from "@heroicons/vue/20/solid";
import { Cog8ToothIcon } from "@heroicons/vue/20/solid";
import { onMounted, ref } from "vue";
import { useSidebarStore } from "@/store/modules/sidebar";
const sidebarStore = useSidebarStore();
const sidebar = ref<HTMLElement | null>(null); // 临时的 ref
const { t } = useTranslationLang();
const route = useRoute(); // 获取当前路由信息

onMounted(() => {
  // 手动将 sideBar 赋值给 store 中的 sideBar ref
  sidebarStore.sideBar = sidebar.value;
  const handleResize = () => {
    if (window.innerWidth < 768) {
      sidebarStore.closeSidebar();
    } else {
      sidebarStore.openSidebar();
    }
  };
  // 初始化时检查窗口大小
  handleResize();
  // 添加窗口大小改变的监听器
  window.addEventListener("resize", handleResize);
});
</script>

<template>
  <div ref="sidebar" class="sidebar bg-base-100">
    <a href="#" class="logo ml-2">
      <!-- <CogIcon class="size-6" /> -->
      <img
        v-show="sidebarStore.isClosed"
        src="@/assets/32x32.png"
        alt="My PNG Image"
      />
      <div v-show="!sidebarStore.isClosed" class="text-secondary pr-1">
        <span class="text-primary">Dicom</span>Simulator
      </div>
    </a>
    <ul class="side-menu">
      <!-- 判断路径中是否包含 /dicom -->
      <li :class="{ active: route.path.includes('/dicom') }">
        <router-link to="/dicom">
          <LifebuoyIcon class="icon-size ml-2" style="pointer-events: none" />
          <span v-show="!sidebarStore.isClosed">{{ t("sidebar.dicom") }}</span>
        </router-link>
      </li>
      <!-- 判断路径中是否包含 /hl7 -->
      <li :class="{ active: route.path.includes('/hl7') }">
        <router-link to="/hl7">
          <Square3Stack3DIcon class="icon-size ml-2" />
          <span v-show="!sidebarStore.isClosed">{{ t("sidebar.hl7") }}</span>
        </router-link>
      </li>
      <!-- 判断路径中是否包含 /settings -->
      <li :class="{ active: route.path.includes('/worklistsetting') }">
        <router-link to="/worklistsetting">
          <Cog6ToothIcon class="icon-size ml-2" />
          <span v-show="!sidebarStore.isClosed">{{
            t("sidebar.setting")
          }}</span>
        </router-link>
      </li>
      <li :class="{ active: route.path.includes('/mimSetting') }">
        <router-link to="/mimSetting">
          <Cog8ToothIcon class="icon-size ml-2" />
          <span v-show="!sidebarStore.isClosed">{{
            t("sidebar.mimsetting")
          }}</span>
        </router-link>
      </li>
      <li :class="{ active: route.path.includes('/settinghl7') }">
        <router-link to="/settinghl7">
          <Cog8ToothIcon class="icon-size ml-2" />
          <span v-show="!sidebarStore.isClosed">{{
            t("sidebar.hl7setting")
          }}</span>
        </router-link>
      </li>
      <li :class="{ active: route.path.includes('/settingmessage') }">
        <router-link to="/settingmessage">
          <Cog8ToothIcon class="icon-size ml-2" />
          <span v-show="!sidebarStore.isClosed">{{
            t("sidebar.hl7messagesetting")
          }}</span>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<style scoped>


/* .sidebar .side-menu li a .bx {
  display: flex;
  justify-content: center;
  min-width: calc(60px - ((4px + 6px) * 2));
  font-size: 1.6rem;
} */

@media screen and (width <=768px) {
  .sidebar {
    width: 200px;
  }
}

.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 2000;
  width: 230px;
  height: 100%;
  overflow-x: hidden;
  scrollbar-width: none;
  transition: all 0.3s ease;
}

.sidebar::-webkit-scrollbar {
  display: none;
}

.sidebar.close {
  width: 60px;
}

.sidebar .logo {
  z-index: 500;
  box-sizing: content-box;
  display: flex;
  align-items: center;
  height: 56px;
  padding-bottom: 20px;
  font-size: 24px;
  font-weight: 700;
}

.sidebar .side-menu {
  width: 100%;
  margin-top: 48px;
}

.sidebar .side-menu li {
  height: 48px;
  padding: 4px;
  margin-left: 6px;
  background: oklch(var(--b1));
  border-radius: 48px 0 0 48px;
}

.sidebar .side-menu li.active {
  position: relative;
  background: oklch(var(--b2));
}

.sidebar .side-menu li.active::before {
  position: absolute;
  top: -40px;
  right: 0;
  z-index: -1;
  width: 40px;
  height: 40px;
  pointer-events: none;
  content: "";
  border-radius: 50%;
  box-shadow: 20px 20px 0 oklch(var(--b2));
}

.sidebar .side-menu li.active::after {
  position: absolute;
  right: 0;
  bottom: -40px;
  z-index: 1;
  width: 40px;
  height: 40px;
  pointer-events: none;
  content: "";
  border-radius: 50%;
  box-shadow: 20px -20px 0 oklch(var(--b2));
}

.sidebar .side-menu li a {
  display: flex;
  align-items: center;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  font-size: 16px;
  color: oklch(var(--bc));
  white-space: nowrap;
  background: oklch(var(--b1));
  border-radius: 48px;
  transition: all 0.3s ease;
}

.icon-size {
  width: 24px; /* 您可以根据实际图标的大小调整 */
  min-width: 24px; /* 防止缩小 */
  height: 24px;
  min-height: 24px;

  /* pointer-events: none; */
  pointer-events: auto;
  cursor: pointer;
  transition: all 0.3s ease; /* 保持过渡效果 */
}

.sidebar .side-menu li a svg {
  margin-right: 5px;
  pointer-events: none;
}

.sidebar .side-menu li.active a {
  color: oklch(var(--nc));
  background: oklch(var(--n));
}

.sidebar.close .side-menu li a {
  width: calc(48px - (4px * 2));
  transition: all 0.3s ease;
}
</style>
