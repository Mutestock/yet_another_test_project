
use crate::connection::mongo_connection::MongoConnector;


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
    .expect("Ping failed"))
}
