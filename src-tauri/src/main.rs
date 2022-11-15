#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod state;

use commands::{
    mongo_commands::ping_mongo, pg_commands::ping_postgres, redis_commands::ping_redis,
};

use state::navigation::{
    get_currently_selected_new_connection, set_currently_selected_new_connection, NavigationStorage,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("cake");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(NavigationStorage::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            ping_mongo,
            ping_postgres,
            ping_redis,
            get_currently_selected_new_connection,
            set_currently_selected_new_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
