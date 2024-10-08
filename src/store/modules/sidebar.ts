// stores/sidebar.ts
import { defineStore } from "pinia";
import { ref } from "vue";

export const useSidebarStore = defineStore("sidebar", () => {
  const sideBar = ref<HTMLElement | null>(null);
  const isClosed = ref(false);

  const closeSidebar = () => {
    sideBar.value?.classList.add("close");
    isClosed.value = true;
  };

  const openSidebar = () => {
    sideBar.value?.classList.remove("close");
    isClosed.value = false;
  };

  return {
    sideBar,
    isClosed,
    closeSidebar,
    openSidebar,
  };
});
