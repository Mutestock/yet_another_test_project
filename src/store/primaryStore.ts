import { defineStore } from "pinia";
import { ConnectionTypes } from "../enums";

export const usePrimaryStore = defineStore('primaryStore', {
    state: () => {
        return { currentNewConnectionSelection: ConnectionTypes.None }
    },
    getters: {
        connectionType: (state) => state.currentNewConnectionSelection
    },
    actions: {
        setCurrentConnectionSelection(connectionType: ConnectionTypes){
            this.currentNewConnectionSelection = connectionType
        },
        resetCurrentConnectionSelection() {
            this.currentNewConnectionSelection = ConnectionTypes.None
        }
    }
})