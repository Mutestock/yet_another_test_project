<script setup lang="ts">
import { ref } from "vue";
import { handleRedirect } from "../router";

const redirected = ref("")

const redirectMongo = async () => { await handleRedirect("mongo") }
const redirectPostgres = async () => { await handleRedirect("postgres") }
const redirectRedis = async () => { await handleRedirect("redis") }


</script>

<template>
    <div>
        <p>Choose a connection type</p>
        <div v-if="!(redirected)">
            <button type="button" @click=redirectMongo()>MongoDB</button>
            <button type="button" @click=redirectPostgres()>Postgres</button>
            <button type="button" @click=redirectRedis()>Redis</button>
        </div>
    </div>
</template>

