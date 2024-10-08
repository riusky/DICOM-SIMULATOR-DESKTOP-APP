import { useI18n } from "vue-i18n";
import { useRoute } from "vue-router";
import { onBeforeMount } from "vue";
import { useGlobal } from "@pureadmin/utils";

export function useTranslationLang() {
  const { locale, t } = useI18n();
  const route = useRoute();
  const { $storage } = useGlobal<GlobalPropertiesApi>();
  function translationCh() {
    $storage.locale = { locale: "zh" };
    locale.value = "zh";
  }

  function translationEn() {
    $storage.locale = { locale: "en" };
    locale.value = "en";
  }

  onBeforeMount(() => {
    locale.value = $storage.locale?.locale ?? "zh";
  });

  return {
    t,
    route,
    locale,
    translationCh,
    translationEn,
  };
}
