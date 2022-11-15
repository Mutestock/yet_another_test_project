import { defineStore } from "pinia";
import { DatabaseOptions } from "../connectors";
import { ConnectionType } from "../enums";

export const usePrimaryStore = defineStore('primaryStore', {

    state: () => {
        let allCurrentConnections: Array<DatabaseOptions> = new Array<DatabaseOptions>();
        return {
            currentNewConnectionSelection: ConnectionType.None,
            allCurrentConnections: allCurrentConnections
        }
    },
    getters: {
        connectionType: (state) => state.currentNewConnectionSelection,
        // For sidenav. Shows which connection types can be selected for generation
        availableConnections: (state) =>
            Object.values(ConnectionType)
                .map(x => x !== ConnectionType.None && x !== state.currentNewConnectionSelection),
    },
    actions: {
        setCurrentConnectionSelection(connectionType: ConnectionType) {
            this.currentNewConnectionSelection = connectionType
        },
        resetCurrentConnectionSelection() {
            this.currentNewConnectionSelection = ConnectionType.None
        },
        pushToAllCurrentConnections(databaseConnection: DatabaseOptions) {
            this.allCurrentConnections.push(databaseConnection)
        },
        removeFromAllCurrentConnections(databaseConnection: DatabaseOptions) {
            let index = this.allCurrentConnections.indexOf(databaseConnection)
            if (index!==-1){
                this.allCurrentConnections.splice(index, 1)
            }
        }
    }
})