import { listen } from "@tauri-apps/api/event";
import { backendLog } from "../utils/logging";
import router from "../router";

// Need to register the event listeners.
// Would need to write the events or otherwise inject the listeners into the main file
// We don't want that
export function eventInit() {
    let eventName: string = ""
    listen<string>("tauri://menu", (event) => {
        eventName = event.payload
        backendLog(`Bump Log from menu with the payload ${eventName}`, "info")
            .then(x => x);
    })
        .then(x => x)
        .catch((err) => backendLog(`Log failed on event name ${eventName}. TS Output: ${err}`, "error"));
}