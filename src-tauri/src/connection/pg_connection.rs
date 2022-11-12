use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct PgConnector {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
}

impl PgConnector {
    pub fn new(
        username: String,
        password: String,
        host: String,
        port: u16,
        database: String,
    ) -> Self {
        Self {
            username,
            password,
            host,
            port,
            database,
        }
    }

    pub async fn get_pool(&self) -> Result<PgPool, sqlx::error::Error> {
        PgPoolOptions::new().max_connections(5).connect(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            )
            .as_str()
            .as_ref(),
        ).await
    }
}
