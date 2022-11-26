use std::sync::Mutex;

use tauri::State;

use crate::connection::{
    common::{DatabaseConnection, GenericConnection}, mongo_connection::MongoConnector, pg_connection::PgConnector,
    redis_connection::RedisConnector,
};

pub struct ActiveConnectionsStorage {
    pub all_active_connections: Mutex<Vec<Box<dyn DatabaseConnection>>>,
    pub all_mongo_connections: Mutex<Vec<MongoConnector>>,
    pub all_redis_connections: Mutex<Vec<RedisConnector>>,
    pub all_postgres_connections: Mutex<Vec<PgConnector>>,
}

impl Default for ActiveConnectionsStorage {
    fn default() -> Self {
        Self {
            all_active_connections: Mutex::new(vec![]),
            all_mongo_connections: Mutex::new(vec![]),
            all_redis_connections: Mutex::new(vec![]),
            all_postgres_connections: Mutex::new(vec![]),
        }
    }
}

#[tauri::command]
pub async fn get_all_active_connections(
    active_connection_storage: State<'_, ActiveConnectionsStorage>,
) -> Result<Vec<GenericConnection>, tauri::InvokeError> {

    let generic_connections = active_connection_storage
        .all_active_connections
        .lock()
        .unwrap()
        .iter()
        .map(|x|x.to_generic_connection())
        .collect();


    Ok(generic_connections)
}
