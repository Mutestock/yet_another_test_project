#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use connection::mongo_connection::MongoConnector;

mod connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn ping_mongo(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
    database: &str,
) -> Result<bool, ()> {
    Ok(MongoConnector::new(
        username.to_owned(),
        password.to_owned(),
        host.to_owned(),
        port.to_owned()
            .parse::<u16>()
            .expect("Port wasn't an integer"),
        database.to_owned(),
    )
    .create_connection()
    .await
    .expect("Connection succeeded")
    .ping()
    .await
    .expect("Ping failed"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ping_mongo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
