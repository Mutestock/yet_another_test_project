#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod state;
mod ui;

use commands::{
    mongo_commands::ping_mongo, pg_commands::ping_postgres, redis_commands::ping_redis,
};

use state::navigation::{
    get_currently_selected_new_connection, set_currently_selected_new_connection, NavigationStorage,
};
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use ui::menu::{create_menu, handle_menu_event};


fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(|event| handle_menu_event(event))
        .manage(NavigationStorage::default())
        .plugin(
            LoggerBuilder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            ping_mongo,
            ping_postgres,
            ping_redis,
            get_currently_selected_new_connection,
            set_currently_selected_new_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
