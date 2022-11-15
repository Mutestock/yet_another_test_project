<script setup lang="ts">
import { ConnectionType } from '../enums';
import { usePrimaryStore } from '../store/primaryStore';
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import MongoConnection from "./connection/MongoConnection.vue";
import PostgresConnection from './connection/PostgresConnection.vue';
import RedisConnection from './connection/RedisConnection.vue';
let primaryStore = usePrimaryStore();

const currentDatabaseTypeSelection = ref(ConnectionType.None)
const stuff = ref("cake")

async function getCurrentDatabaseTypeSelection() {
    let databaseType = await invoke("get_currently_selected_new_connection", {})
    currentDatabaseTypeSelection.value = databaseType as ConnectionType;;
}

async function cake(connectionType: ConnectionType) {
    await invoke("set_currently_selected_new_connection", {
        whatevs: connectionType as number
    });
}
 

</script>

<template>
    <div v-if="primaryStore.connectionType == ConnectionType.Mongo">
        <MongoConnection />
    </div>
    <div v-else-if="primaryStore.connectionType == ConnectionType.Postgres">
        <PostgresConnection />
    </div>
    <div v-else-if="primaryStore.connectionType == ConnectionType.Redis">
        <RedisConnection />
    </div>
    <div v-else>
        <p>Choose a connection type</p>
        <button type="button" @click=cake(ConnectionType.Mongo)>MongoDB</button>
        <button type="button"
            @click="primaryStore.setCurrentConnectionSelection(ConnectionType.Postgres)">Postgres</button>
        <button type="button" @click="primaryStore.setCurrentConnectionSelection(ConnectionType.Redis)">Redis</button>
    </div>
    <button type="button" @click=getCurrentDatabaseTypeSelection()>Boop</button>
    {{currentDatabaseTypeSelection}}
</template>