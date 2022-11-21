<script setup lang="ts">
import { ConnectionType } from '../enums';
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import MongoConnection from "./connection/MongoConnection.vue";
import PostgresConnection from './connection/PostgresConnection.vue';
import RedisConnection from './connection/RedisConnection.vue';

const currentDatabaseTypeSelection = ref(ConnectionType.None)

async function getCurrentDatabaseTypeSelection(): Promise<void> {
    let databaseType = await invoke("get_currently_selected_new_connection", {})
    currentDatabaseTypeSelection.value = databaseType as ConnectionType;
}

async function setCurrentConnectionSelection(connectionType: ConnectionType): Promise<void> {
    await invoke("set_currently_selected_new_connection", {
        whatevs: connectionType as number
    });
    currentDatabaseTypeSelection.value = await invoke("get_currently_selected_new_connection", {}) 
}
</script>

<template>
    <div>
        <div v-if="currentDatabaseTypeSelection == ConnectionType.Mongo">
            <MongoConnection />
        </div>
        <div v-else-if="currentDatabaseTypeSelection == ConnectionType.Postgres">
            <PostgresConnection />
        </div>
        <div v-else-if="currentDatabaseTypeSelection == ConnectionType.Redis">
            <RedisConnection />
        </div>
        <div v-else>
            <p>Choose a connection type</p>
            <button type="button" @click=setCurrentConnectionSelection(ConnectionType.Mongo)>MongoDB</button>
            <button type="button" @click=setCurrentConnectionSelection(ConnectionType.Postgres)>Postgres</button>
            <button type="button" @click=setCurrentConnectionSelection(ConnectionType.Redis)>Redis</button>
        </div>
        {{ currentDatabaseTypeSelection }} 
    </div>
</template>

