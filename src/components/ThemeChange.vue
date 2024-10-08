<template>
  <div
    tabindex="0"
    :class="`dropdown-content bg-base-200 text-base-content rounded-box top-px h-[28.6rem] max-h-[calc(100vh-10rem)] w-56 overflow-y-auto border border-white/5 shadow-2xl outline outline-1 outline-black/5 ${contentClasses}`"
  >
    <div class="grid grid-cols-1 gap-3 p-3">
      <button
        v-for="theme in themes"
        :key="theme"
        class="outline-base-content text-start outline-offset-4"
        :data-set-theme="theme"
        data-act-class="[&_svg]:visible"
      >
        <span
          :data-theme="theme"
          class="bg-base-100 rounded-btn text-base-content block w-full cursor-pointer font-sans"
        >
          <span class="grid grid-cols-5 grid-rows-3">
            <span
              class="col-span-5 row-span-3 row-start-1 flex items-center gap-2 px-4 py-3"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="invisible h-3 w-3 shrink-0"
              >
                <path
                  d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"
                />
              </svg>
              <span class="flex-grow text-sm">
                {{ theme }}
              </span>
              <span class="flex h-full shrink-0 flex-wrap gap-1">
                <span class="bg-primary rounded-badge w-2" />
                <span class="bg-secondary rounded-badge w-2" />
                <span class="bg-accent rounded-badge w-2" />
                <span class="bg-neutral rounded-badge w-2" />
              </span>
            </span>
          </span>
        </span>
      </button>
      <a
        class="outline-base-content overflow-hidden rounded-lg"
        href="/theme-generator/"
      >
        <div
          class="hover:bg-neutral hover:text-neutral-content w-full cursor-pointer font-sans"
        >
          <div class="flex gap-2 p-3">
            <svg
              width="24"
              height="24"
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4 fill-current"
              viewBox="0 0 512 512"
            >
              <path d="M96,208H48a16,16,0,0,1,0-32H96a16,16,0,0,1,0,32Z" />
              <line x1="90.25" y1="90.25" x2="124.19" y2="124.19" />
              <path
                d="M124.19,140.19a15.91,15.91,0,0,1-11.31-4.69L78.93,101.56a16,16,0,0,1,22.63-22.63l33.94,33.95a16,16,0,0,1-11.31,27.31Z"
              />
              <path
                d="M192,112a16,16,0,0,1-16-16V48a16,16,0,0,1,32,0V96A16,16,0,0,1,192,112Z"
              />
              <line x1="293.89" y1="90.25" x2="259.95" y2="124.19" />
              <path
                d="M260,140.19a16,16,0,0,1-11.31-27.31l33.94-33.95a16,16,0,0,1,22.63,22.63L271.27,135.5A15.94,15.94,0,0,1,260,140.19Z"
              />
              <line x1="124.19" y1="259.95" x2="90.25" y2="293.89" />
              <path
                d="M90.25,309.89a16,16,0,0,1-11.32-27.31l33.95-33.94a16,16,0,0,1,22.62,22.63l-33.94,33.94A16,16,0,0,1,90.25,309.89Z"
              />
              <path
                d="M219,151.83a26,26,0,0,0-36.77,0l-30.43,30.43a26,26,0,0,0,0,36.77L208.76,276a4,4,0,0,0,5.66,0L276,214.42a4,4,0,0,0,0-5.66Z"
              />
              <path
                d="M472.31,405.11,304.24,237a4,4,0,0,0-5.66,0L237,298.58a4,4,0,0,0,0,5.66L405.12,472.31a26,26,0,0,0,36.76,0l30.43-30.43h0A26,26,0,0,0,472.31,405.11Z"
              />
            </svg>
            <div class="flex-grow text-sm font-bold">
              <!-- {{ t("Make your theme") }}! -->
              Make your theme
            </div>
          </div>
        </div>
      </a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, defineProps, computed } from "vue";
import { themeChange } from "theme-change";

const props = defineProps<{
  themes: string[];
  dropdownClasses?: string;
  contentClasses?: string;
}>();

onMounted(() => {
  themeChange(false);
});
const contentClasses = computed(() => props.contentClasses ?? "mt-5");
const themes = computed(() =>
  props.themes.length > 0
    ? props.themes
    : [
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
      ],
);
</script>
