use std::sync::Mutex;
use tauri::State;

use crate::connection::{
    common::DatabaseConnection, mongo_connection::MongoConnector, pg_connection::PgConnector,
    redis_connection::RedisConnector,
};

pub struct ActiveConnectionStorage {
    all_active_connections: Mutex<Vec<Box<dyn DatabaseConnection>>>,
    all_mongo_connections: Mutex<Vec<MongoConnector>>,
    all_redis_connections: Mutex<Vec<RedisConnector>>,
    all_postgres_connections: Mutex<Vec<PgConnector>>,
}
