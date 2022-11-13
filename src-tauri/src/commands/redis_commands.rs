use crate::connection::redis_connection::RedisConnector;

#[tauri::command]
pub async fn ping_redis(
    username: &str,
    password: &str,
    host: &str,
    port: &str,
) -> Result<bool, ()> {
    let pwd_opt = match password == "" {
        true => Some(password.to_owned()),
        false => None,
    };

    Ok(RedisConnector::new(
        username.to_owned(),
        pwd_opt,
        host.to_owned(),
        port.parse::<u16>()
            .expect("Port could be parsed to u16"),
    )
    .create_connection()
    .await
    .expect("connection succesful")
    .ping()
    .await
    .expect("Ping successful"))
}
