<script setup lang="ts">
import LayNavbar from "./components/lay-navbar/index.vue";
import LayContent from "./components/lay-content/index.vue";
import NavVertical from "./components/lay-sidebar/NavVertical.vue";
</script>

<template>
  <NavVertical />
  <div
    class="content bg-base-200 border-r-dashed border-b-dashed border-b-8 border-r-8 border-base-100"
  >
    <LayNavbar />
    <LayContent />
  </div>
</template>

<style>


@media screen and (width <= 768px) {
  .content {
    left: 200px;
    width: calc(100% - 60px);
  }
}

:root {
  --light: #f6f6f9;
  --primary: #1976d2;
  --light-primary: #cfe8ff;
  --grey: #eee;
  --dark-grey: #aaa;
  --dark: #363949;
  --danger: #d32f2f;
  --light-danger: #fecdd3;
  --warning: #fbc02d;
  --light-warning: #fff2c6;
  --success: #388e3c;
  --light-success: #bbf7d0;
}

* {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
  font-family: Poppins, sans-serif;
}

html {
  overflow: hidden;
}

.content {
  position: relative;
  left: 230px;
  width: calc(100% - 230px);
  height: 100%;
  transition: all 0.3s ease;
}

.sidebar.close ~ .content {
  left: 60px;
  width: calc(100% - 60px);
}

/* @import url("https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700;800&display=swap"); */

/* body {
  overflow-x: hidden;
  overflow-y: hidden;
} */
</style>
