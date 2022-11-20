<script setup lang="ts">
import Router from "./components/Router.vue";
import ConnectionSideNav from "./components/ConnectionSideNav.vue";
</script>

<template>
  <div class="container">
    <Router />
  </div>
  <div class="bg-gray-900 w-32">
    <ConnectionSideNav />
  </div>
</template>
