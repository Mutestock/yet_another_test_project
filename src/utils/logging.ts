import { invoke } from "@tauri-apps/api/tauri";

export async function backendLog(message: string, severity: string): Promise<void> {
    await invoke("backend_log", {
        message: message,
        severity: severity
    })
}