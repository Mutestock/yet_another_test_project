use redis::Client;

use crate::connection::common::{ConnectionType, DatabaseConnection};

pub struct RedisConnector {
    username: String,
    password: Option<String>,
    host: String,
    port: u16,
    client: Option<Client>,
    connection_type: ConnectionType,
}

impl RedisConnector {
    pub fn new(username: String, password: Option<String>, host: String, port: u16) -> Self {
        Self {
            username,
            password,
            host,
            port,
            client: None,
            connection_type: ConnectionType::Redis,
        }
    }

    pub async fn create_connection(&mut self) -> Result<&mut Self, redis::RedisError> {
        match self.client {
            Some(_) => (),
            None => {
                if self.password == None {
                    self.client = Some(redis::Client::open(format!(
                        "redis://{}@{}:{}",
                        self.username, self.host, self.port
                    ))?)
                } else {
                    self.client = Some(redis::Client::open(format!(
                        "redis://{}:{}@{}:{}",
                        self.username,
                        self.password.as_ref().unwrap(),
                        self.host,
                        self.port
                    ))?)
                }
            }
        }
        Ok(self)
    }

    pub async fn ping(&mut self) -> Result<bool, redis::RedisError> {
        let connection_established = match self.create_connection().await {
            Ok(_) => true,
            Err(_) => false,
        };
        Ok(connection_established)
    }
}

impl DatabaseConnection for RedisConnector {
    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_type(&self) -> ConnectionType {
        self.connection_type
    }
}
