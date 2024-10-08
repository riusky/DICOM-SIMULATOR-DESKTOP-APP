<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import WorkListSet from "./components/WorklistSet.vue";
// Store the message returned from Rust
const greetingMessage = ref("");

// Store the user's input name
const name = ref("");

// Call the Tauri command
const greet = async () => {
  try {
    greetingMessage.value = await invoke<string>("greet", {
      name: name.value,
    });
  } catch (error) {
    console.error("Failed to invoke greet command:", error);
  }
};
</script>

<template>
  <div class="header">
    <div class="left">
      <h1 class="text-secondary">MPPS</h1>
    </div>
  </div>

  <WorkListSet />
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

.greeting-demo {
  margin-top: 50px;
  text-align: center;
}

@media screen and (width <=576px) {
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
</style>
