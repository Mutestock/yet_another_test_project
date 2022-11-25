import { listen } from "@tauri-apps/api/event";
import { backendLog } from "../utils/logging";
import { handleRedirect } from "../router";

// Need to register the event listeners.
// Would need to write the events or otherwise inject the listeners into the main file
// We don't want that
export async function eventInit(): Promise<void> {
    let eventName: string = ""
    listen<string>("tauri://menu", (event) => {
        eventName = event.payload
        handleRedirect(eventName)
        backendLog(`Front=>menu_event=>${eventName}`, "info")
    })
        .catch((err) => backendLog(`Log failed on event name ${eventName}. TS Output: ${err}`, "error"));
}
