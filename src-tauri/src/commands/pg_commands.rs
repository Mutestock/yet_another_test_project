use crate::connection::pg_connection::PgConnector;

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
