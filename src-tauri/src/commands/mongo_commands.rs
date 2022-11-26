use tauri::State;

use crate::{
    connection::mongo_connection::MongoConnector,
    state::active_connections::ActiveConnectionsStorage,
};

#[tauri::command]
pub async fn ping_mongo(
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
    .expect("Ping succeeded"))
}

#[tauri::command]
pub async fn new_mongo_connection(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
    database: &str,
    active_connection_storage: State<'_, ActiveConnectionsStorage>,
) -> Result<(), tauri::InvokeError> {
    let mongo_connection = MongoConnector::new(
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
    .clone();

    active_connection_storage
        .all_active_connections
        .lock()
        .unwrap()
        .push(Box::new(mongo_connection.clone()));
    active_connection_storage
        .all_mongo_connections
        .lock()
        .unwrap()
        .push(mongo_connection);

    Ok(())
}
