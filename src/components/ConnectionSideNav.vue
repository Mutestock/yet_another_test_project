<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const activeConnections = ref([])

async function getAllConnections(){
    activeConnections.value = await invoke("get_all_active_connections");
}

</script>

<template>
    <button type="button" @click="getAllConnections()">Refresh</button>
    <table id="connection-side-nav-table">
        <tr v-for="connection in activeConnections">
            {{connection}}
        </tr>
    </table>
</template>
