import { defineStore } from "pinia";
import { MongoOptions } from "../connectors";

export const useMongoStore = defineStore("mongoStore", {
    state: () => {
        let mongoOptionsList: Array<MongoOptions> = new Array()
        return {
            storedOptions: mongoOptionsList
        }
    },
    actions: {
        addOption(mongoOptions: MongoOptions) {
            this.storedOptions.push(mongoOptions);
        }
    }
})