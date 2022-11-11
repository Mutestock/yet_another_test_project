use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub struct MongoConnector {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    client: Option<Client>,
}

const CONNECTION_EXPECT_MESSAGE: &str = "Connection settings have been defined";

impl MongoConnector {
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
            client: None,
        }
    }

    pub async fn create_connection(&mut self) -> Result<&mut Self, mongodb::error::Error> {
        let conn_str = format!(
            "mongodb://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        );
        let client_options = ClientOptions::parse(conn_str).await?;
        self.client = Some(Client::with_options(client_options)?);
        Ok(self)
    }

    pub async fn ping(&self) -> Result<bool, mongodb::error::Error> {
        let client = self.client.as_ref().expect(CONNECTION_EXPECT_MESSAGE);

        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await?;

        Ok(true)
    }

    pub async fn get_cluster_database_names(&self) -> Result<Vec<String>, mongodb::error::Error> {
        self.client
            .as_ref()
            .expect(CONNECTION_EXPECT_MESSAGE)
            .list_database_names(None, None)
            .await
    }

    pub async fn get_database_handle(
        &self,
        database_name: &str,
    ) -> Result<Database, mongodb::error::Error> {
        Ok(self
            .client
            .as_ref()
            .expect(CONNECTION_EXPECT_MESSAGE)
            .database(database_name))
    }

    pub async fn get_database_collection_names(
        &self,
        database_name: &str,
    ) -> Result<Vec<String>, mongodb::error::Error> {
        self.client
            .as_ref()
            .expect(CONNECTION_EXPECT_MESSAGE)
            .database(database_name)
            .list_collection_names(None)
            .await
    }
}
