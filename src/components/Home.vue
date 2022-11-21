
<script setup lang="ts">
import { backendLog } from '../utils/logging';


async function bumpLog(){
    await backendLog("Log bumped on home page", "info")
}


</script>
 

<template>
    <div>
        <p>
            This is home
        </p>

        <button type="button" @click="bumpLog()">Bump the log</button>
    </div>
</template>