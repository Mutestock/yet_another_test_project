<script setup lang="ts">
import { ConnectionTypes } from '../enums';
import { usePrimaryStore } from '../store/primaryStore';
import MongoConnection from "./connection/MongoConnection.vue";
import PostgresConnection from './connection/PostgresConnection.vue';
import RedisConnection from './connection/RedisConnection.vue';
let primaryStore = usePrimaryStore();

</script>

<template>
    <div v-if="primaryStore.connectionType == ConnectionTypes.Mongo">
        <MongoConnection />
    </div>
    <div v-else-if="primaryStore.connectionType == ConnectionTypes.Postgres">
        <PostgresConnection />
    </div>
    <div v-else-if="primaryStore.connectionType == ConnectionTypes.Redis">
        <RedisConnection />
    </div>
    <div v-else>
        <p>Choose a connection type</p>
        <button type="button" @click="primaryStore.setCurrentConnectionSelection(ConnectionTypes.Mongo)">MongoDB</button>
        <button type="button" @click="primaryStore.setCurrentConnectionSelection(ConnectionTypes.Postgres)">Postgres</button>
        <button type="button" @click="primaryStore.setCurrentConnectionSelection(ConnectionTypes.Redis)">Redis</button>
    </div>
</template>