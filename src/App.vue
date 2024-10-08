<script setup lang="ts">
import { onBeforeMount, onMounted } from "vue";
import { themeChange } from "theme-change";
import Layout from "@/layout/index.vue";
import { useI18n } from "vue-i18n";
import { useGlobal } from "@pureadmin/utils";
onMounted(() => {
  themeChange(false);
});
const { locale, t } = useI18n();
const { $storage, $config } = useGlobal<GlobalPropertiesApi>();

onBeforeMount(() => {
  locale.value = $storage.locale?.locale ?? "en";
});
</script>

<template>
  <!-- <Layout /> -->
  <RouterView />
</template>
