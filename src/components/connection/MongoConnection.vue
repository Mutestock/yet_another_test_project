
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useMongoStore } from "../../store/mongoStore";
import { MongoOptions } from "../../connectors";

const username = ref("")
const password = ref("")
const host = ref("")
const port = ref("")
const pingResult = ref("")

let mongoStore = useMongoStore()

async function pingMongo() {
    let res: boolean = await invoke("ping_mongo", {
        username: username.value,
        password: password.value,
        host: host.value,
        port: port.value,
        database: ""
    });
    if (res) {
        pingResult.value = "Connected successfully";
        mongoStore.addOption(new MongoOptions(username.value, password.value, host.value, port.value))
    }
    else {
        pingResult.value = "Failed to connect"
    }
}
</script>


<template>
    <div class="card">
        <input id="username-input" v-model="username" placeholder="Username" />
        <input id="password-input" v-model="password" placeholder="Password" type="password" />
        <input id="host-input" v-model="host" placeholder="Host" />
        <input id="port-input" v-model="port" placeholder="Port" />
        <button type="button" @click=pingMongo()>Ping Mongo</button>
    </div>
    <p>{{ pingResult }}</p>
</template>