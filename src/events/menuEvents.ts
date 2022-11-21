import { UnlistenFn } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import router from "../router";


const detachNewConnectionEvent: UnlistenFn = await appWindow.listen<string>("new_connection", (_) => {
    let path_raw: string | undefined = router
        .getRoutes()
        .find(x => x.name === "NewConnection")?.name?.toString();
    if (path_raw === undefined) {
        console.log("Route redirection to new connection failed");
    }
    else {
        router.push(path_raw);
    }
});