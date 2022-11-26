<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
import { GenericConnection } from "../types";

const activeConnections: Ref<GenericConnection[]> = ref([])

async function getAllConnections() {
    activeConnections.value = await invoke("get_all_active_connections");
}

</script>

<template>
    <button class="block w-full bg-indigo-700 text-gray-200 h-5" @click="getAllConnections()">Refresh</button>
    
    <div v-for="connection in activeConnections">
        <button class="block h-8 w-full border border-r-amber-800 shadow-lime-50">
            {{connection.name}}
        </button>
        <div class="bg-gray-700 block h-8 w-full ">
            {{connection.db_type}}
        </div>
    </div>
</template>
