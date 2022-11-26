<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import router from "../../router";

const username = ref("")
const password = ref("")
const host = ref("")
const port = ref("")
const database = ref("")
const pingResult = ref(false)

async function pingPostgres(): Promise<void> {
    pingResult.value = await invoke("ping_postgres", {
        username: username.value,
        password: password.value,
        host: host.value,
        port: port.value,
        database: database
    });
}

async function connectPostgres(): Promise<void> {
    await invoke("new_postgres_connection", {
        username: username.value,
        password: password.value,
        host: host.value,
        port: port.value,
        database: database
    });
    await router.push("Home");
}


</script>

<template>
    <p>New Postgres Connection</p>
    <p>Ping Result: {{ pingResult }}</p>
    <div class="h-screen rows=2 gap-8">
        <input id="username-input" v-model="username" placeholder="Username" />
        <input id="password-input" v-model="password" placeholder="Password" type="password" />
        <input id="host-input" v-model="host" placeholder="Host" />
        <input id="port-input" v-model="port" placeholder="Port" />
        <input id="database-input" v-model="database" placeholder="Database" />
        <button type="button" @click=pingPostgres()>Ping</button>
        <button type="button" @click=connectPostgres()>Connect</button>
    </div>
</template>