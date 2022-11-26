<script setup lang="ts">
import Router from "./components/Router.vue";
import ConnectionSideNav from "./components/ConnectionSideNav.vue";
</script>

<template>
  <div class="columns-2 gap-8">
    <!---<ConnectionSideNav class="h-screen w-32"/>-->
    <div class="bg-gray-900 w-32 h-screen">
      <ConnectionSideNav />
    </div>
    <Router class="h-screen"/>
  </div>
</template>
