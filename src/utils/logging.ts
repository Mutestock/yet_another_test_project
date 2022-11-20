import { trace, info, error, attachConsole } from "tauri-plugin-log-api";

const detach = await attachConsole()

trace("Trace");
info("Info");
error("Error");

detach()