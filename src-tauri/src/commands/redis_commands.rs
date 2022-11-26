use tauri::State;

use crate::connection::redis_connection::RedisConnector;
use crate::state::active_connections::ActiveConnectionsStorage;

#[tauri::command]
pub async fn ping_redis(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
) -> Result<bool, tauri::InvokeError> {
    let pwd_opt = match password == "" {
        true => Some(password.to_owned()),
        false => None,
    };

    Ok(RedisConnector::new(
        username.to_owned(),
        pwd_opt,
        host.to_owned(),
        port.parse::<u16>().expect("Port could be parsed to u16"),
    )
    .create_connection()
    .await
    .expect("connection successful")
    .ping()
    .await
    .expect("Ping successful"))
}

#[tauri::command]
pub async fn new_redis_connection(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
    active_connections_storage: State<'_, ActiveConnectionsStorage>,
) -> Result<(), tauri::InvokeError> {
    let pwd_opt = match password == "" {
        true => Some(password.to_owned()),
        false => None,
    };

    let redis_connection = RedisConnector::new(
        username.to_owned(),
        pwd_opt,
        host.to_owned(),
        port.parse::<u16>().expect("Port could be parsed to u16"),
    )
    .create_connection()
    .await
    .expect("connection successful")
    .clone();

    active_connections_storage
        .all_active_connections
        .lock()
        .unwrap()
        .push(Box::new(redis_connection.clone()));

    active_connections_storage
        .all_redis_connections
        .lock()
        .unwrap()
        .push(redis_connection);
    Ok(())
}
