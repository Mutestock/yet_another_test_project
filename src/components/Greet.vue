<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const pingResult = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("greet", { name: name.value });
}

async function pingMongo() {
    let res: boolean = await invoke("ping_mongo", {
        username: "root",
        password: "example",
        host: "localhost",
        port: "27017",
        database: "test_mongodb_database",
    });
    if (res) {
        pingResult.value = "Connected";
    } else if (!res) {
        pingResult.value = "Connection failed";
    } else {
        console.log(res);
        pingResult.value = "Sad days";
    }
}
</script>

<template>
    <div class="card">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="button" @click="greet()">Greet</button>
        <button type="button" @click="pingMongo()">Ping Mongo</button>
    </div>

    <p>{{ greetMsg }}</p>
    <p>{{ pingResult }}</p>
</template>
