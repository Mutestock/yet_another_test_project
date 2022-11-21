<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

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
</script>

<template>
    <div class="card">
        <input id="username-input" v-model="username" placeholder="Username" />
        <input id="password-input" v-model="password" placeholder="Password" type="password" />
        <input id="host-input" v-model="host" placeholder="Host" />
        <input id="port-input" v-model="port" placeholder="Port" />
        <input id="database-input" v-model="database" placeholder="Database" />
        <button type="button" @click=pingPostgres()>Connect</button>
    </div>
    <p>{{ pingResult }}</p>
</template>