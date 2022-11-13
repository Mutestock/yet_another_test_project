import { defineStore } from "pinia";
import { ConnectionTypes } from "../enums";

export const useCounterStore = defineStore('counter', {
    state: () => {
        return { currentNewConnectionSelection: ConnectionTypes.None }
    },
})