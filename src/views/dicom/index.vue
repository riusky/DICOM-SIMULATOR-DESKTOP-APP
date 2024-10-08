<script setup lang="ts">
import { ref } from "vue";
import { NumberedListIcon } from "@heroicons/vue/24/solid";
import { Square3Stack3DIcon } from "@heroicons/vue/24/solid";
import { CubeTransparentIcon } from "@heroicons/vue/24/solid";
import { CircleStackIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import { useTranslationLang } from "@/layout/hooks/useTranslationLang";

const { t } = useTranslationLang();
const router = useRouter();
// Define a ref to store the index of the currently active item
const activeItem = ref<number | null>(null);
const showAlert = ref(false);
const countdown = ref(3.0); // Initial countdown value is 3.0 seconds
const activeItemTitle = ref(""); // Title of the currently clicked item
// Handle click event
const handleItemClick = (index: number, item) => {
  activeItem.value = index;
  if (["UPS", "DCM"].includes(item.title)) {
    // Display the alert message
    showAlert.value = true;
    countdown.value = 3.0;
    activeItemTitle.value = item.title; // Set the title of the current item
    const interval = setInterval(() => {
      countdown.value = parseFloat((countdown.value - 0.1).toFixed(2)); // Keep two decimal places
      if (countdown.value <= 0) {
        clearInterval(interval); // Clear the countdown timer
        showAlert.value = false; // Hide the alert message
      }
    }, 100);
    return;
  }
  router.push(item.url);
};

// Define the items data
const items = [
  {
    title: t("dicom.mpps"),
    url: "dicom/mpps",
    status: t("dicom.workliststatus"),
    iconClass: "bg-secondary-content text-secondary",
    iconComponent: Square3Stack3DIcon,
  },
  {
    title: t("dicom.worklist"),
    url: "dicom/workList",
    status: t("dicom.mpps_mark"),
    iconClass: "bg-primary-content text-primary",
    iconComponent: NumberedListIcon,
  },
  {
    title: "C-STORE",
    url: "/dicom/cstore",
    status: "headless CStore & other CStore",
    iconClass: "bg-accent-content text-accent",
    iconComponent: CubeTransparentIcon,
  },
  {
    title: "DCM",
    url: "dcm",
    status: "Under Development",
    iconClass: "bg-neutral-content text-neutral",
    iconComponent: CircleStackIcon,
  },
];
</script>

<template>
  <div class="header">
    <div class="left">
      <h1 class="text-secondary">Dicom</h1>
    </div>
  </div>

  <div v-if="showAlert" role="alert" class="alert alert-info">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      class="h-6 w-6 shrink-0 stroke-current"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
    <span
      >"{{ activeItemTitle }}" is under development. This message will close in
      {{ countdown }} seconds...</span
    >
  </div>

  <!-- Insights -->
  <ul class="insights">
    <li
      v-for="(item, index) in items"
      :key="index"
      :class="[
        'transition duration-300 ease-in-out p-4 rounded-lg cursor-pointer',
        {
          active: activeItem === index,
        },
      ]"
      @click="handleItemClick(index, item)"
    >
      <i :class="['bx', item.iconClass]">
        <component :is="item.iconComponent" />
      </i>
      <span class="info">
        <h3>{{ item.title }}</h3>
        <p>{{ item.status }}</p>
      </span>
    </li>
  </ul>
  <!-- End of Insights -->
</template>

<style scoped>
.header {
  display: flex;
  flex-wrap: wrap;
  grid-gap: 16px;
  align-items: center;
  justify-content: space-between;
}

.header .left h1 {
  margin-bottom: 10px;
  font-size: 36px;
  font-weight: 600;
}

.header .report {
  display: flex;
  grid-gap: 10px;
  align-items: center;
  justify-content: center;
  height: 36px;
  padding: 0 16px;
  font-weight: 500;
  color: var(--light);
  background: var(--primary);
  border-radius: 36px;
}

.insights {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  grid-gap: 24px;
  margin-top: 36px;
}

.insights li {
  display: flex;
  grid-gap: 24px;
  align-items: center;
  padding: 24px;
  cursor: pointer;
  background: oklch(var(--b1));
  border-radius: 20px;
}

.insights li .bx {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 80px;
  height: 80px;
  font-size: 36px;
  border-radius: 10px;
}

.insights li .info h3 {
  font-size: 24px;
  font-weight: 600;
  color: oklch(var(--bc));
}

.insights li .info p {
  color: oklch(var(--bc));
}

.insights li:hover,
.active {
  color: var(--light);
  background-color: var(--primary);
  box-shadow: 0 4px 10px rgb(0 0 0 / 20%);
}

@media screen and (width <= 576px) {
  .insights {
    grid-template-columns: 1fr;
  }

  .bottom-data .header {
    min-width: 340px;
  }

  .bottom-data .orders table {
    min-width: 340px;
  }

  .bottom-data .reminders .task-list {
    min-width: 340px;
  }
}

.input {
  padding: 8px;
  margin-right: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.button {
  padding: 8px 16px;
  color: white;
  cursor: pointer;
  background-color: #007bff;
  border: none;
  border-radius: 4px;
}

.button:hover {
  background-color: #0056b3;
}

p {
  margin-top: 20px;
  font-size: 18px;
}
</style>
