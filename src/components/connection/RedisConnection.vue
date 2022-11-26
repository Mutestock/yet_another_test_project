
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import router from "../../router";

const username = ref("")
const password = ref("")
const host = ref("")
const port = ref("")
const pingResult = ref(false)

async function pingRedis(): Promise<void> {
    pingResult.value = await invoke("ping_redis", {
        username: username.value,
        password: password.value,
        host: host.value,
        port: port.value
    });
}

async function connectRedis(): Promise<void> {
    pingResult.value = await invoke("new_redis_connection", {
        username: username.value,
        password: password.value,
        host: host.value,
        port: port.value
    });
    await router.push("Home");
}
</script>

<template>
    <p>New Redis Connection</p>
    <p>Ping Result: {{ pingResult }}</p>
    <div class="h-screen rows=2 gap-8">
        <input id="username-input" v-model="username" placeholder="Username" />
        <input id="password-input" v-model="password" placeholder="Password" type="password" />
        <input id="host-input" v-model="host" placeholder="Host" />
        <input id="port-input" v-model="port" placeholder="Port" />
        <button type="button" @click=pingRedis()>Ping</button>
        <button type="button" @click=connectRedis()>Connect</button>
    </div>
</template>