pub mod database_initialization;
pub mod role_permissions;

use std::env;
use tokio_postgres::NoTls;

pub struct DatabaseConnection {
    pub client: tokio_postgres::Client,
}

static mut CONNECTION: Option<DatabaseConnection> = None;

pub async fn get_connection<'a>() -> &'a DatabaseConnection {
    unsafe {
        if CONNECTION.is_none() {
            CONNECTION = Some(DatabaseConnection::create().await);
        }
        if CONNECTION.as_ref().unwrap().client.is_closed() {
            CONNECTION = Some(DatabaseConnection::create().await);
        }
        CONNECTION.as_ref().unwrap()
    }
}

impl DatabaseConnection {
    async fn create() -> DatabaseConnection {
        info!("Connecting to database");
        let (client, conn) = tokio_postgres::connect(
            env::var("DB_CONNECTION_STRING")
                .expect("DB Connection String was not set!")
                .as_str(),
            NoTls,
        )
        .await
        .expect("Failed to connect to database");

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                error!("Database connection error: {}", e);
            }
        });

        DatabaseConnection { client }
    }
}
