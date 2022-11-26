use tauri::State;

use crate::{
    connection::pg_connection::PgConnector, state::active_connections::ActiveConnectionsStorage,
};

#[tauri::command]
pub async fn ping_postgres(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
    database: &str,
) -> Result<bool, ()> {
    Ok(PgConnector::new(
        username.to_owned(),
        password.to_owned(),
        host.to_owned(),
        port.parse::<u16>().expect("Port could be parsed to u16"),
        database.to_owned(),
    )
    .ping()
    .await
    .expect("Ping successful"))
}

#[tauri::command]
pub async fn new_postgres_connection(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
    database: &str,
    active_connetions_storage: State<'_, ActiveConnectionsStorage>,
) -> Result<(), tauri::InvokeError> {
    let pg_connection = PgConnector::new(
        username.to_owned(),
        password.to_owned(),
        host.to_owned(),
        port.parse::<u16>().expect("Port could be parsed to u16"),
        database.to_owned(),
    );

    active_connetions_storage
        .all_active_connections
        .lock()
        .unwrap()
        .push(Box::new(pg_connection.clone()));
    active_connetions_storage
        .all_postgres_connections
        .lock()
        .unwrap()
        .push(pg_connection);

    Ok(())
}
