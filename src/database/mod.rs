pub mod database_initialization;
pub mod role_permissions;

use std::env;
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Error, NoTls, Socket};

pub struct DatabaseConnection {
    pub client: tokio_postgres::Client,
}

static mut CONNECTION: Option<DatabaseConnection> = None;

pub async fn get_connection<'a>() -> &'a DatabaseConnection {
    unsafe {
        if CONNECTION.is_none() {
            CONNECTION = Some(DatabaseConnection::create().await);
        }
        CONNECTION.as_ref().unwrap()
    }
}

impl DatabaseConnection {
    async fn create() -> DatabaseConnection {
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
                eprintln!("connection error: {}", e);
            }
        });

        DatabaseConnection { client }
    }
}
