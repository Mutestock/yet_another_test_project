import { listen } from "@tauri-apps/api/event";
import { backendLog } from "../utils/logging";
import router from "../router";
import { RouteRecordNormalized } from "vue-router";

// Need to register the event listeners.
// Would need to write the events or otherwise inject the listeners into the main file
// We don't want that
export async function eventInit(): Promise<void> {
    let eventName: string = ""
    listen<string>("tauri://menu", (event) => {
        eventName = event.payload
        handleEvents(eventName)
        backendLog(`Bump Log from menu with the payload ${eventName}`, "info")
            .then(x => x);
    })
        .then(x => x)
        .catch((err) => backendLog(`Log failed on event name ${eventName}. TS Output: ${err}`, "error"));
}


function handleEvents(eventName: string): void {
    switch (eventName) {
        case "new_connection": redirectByEventName("NewConnection");
        case "import": void(0)
        case "export": void(0);
        case "bump_log": void(0);
        case "database_to_application": void(0);
        default: void(0);
    }
}


function redirectByEventName(eventName: string): void {
    let pathRaw: RouteRecordNormalized | undefined = router
        .getRoutes()
        .find(x => x.name == eventName)
    if (pathRaw !== undefined) {
        router.push(pathRaw)
    }
}
