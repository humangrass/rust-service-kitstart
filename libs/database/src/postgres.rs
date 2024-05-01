use sqlx::{PgPool};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use log::{error, info};
use crate::config::DatabaseConfig;

pub async fn new_postgres_pool(config: DatabaseConfig) -> Result<PgPool, anyhow::Error> {
    let connect_options = PgConnectOptions::new()
        .username(&config.username)
        .password(&config.password)
        .host(&config.host)
        .port(config.port)
        .database(&config.database);

    let pool = match PgPoolOptions::new()
        .max_connections(config.max_open_cons)
        .min_connections(config.min_idle_cons)
        .acquire_timeout(config.connection_timeout)
        .max_lifetime(config.conn_max_lifetime)
        .idle_timeout(config.idle_timeout)
        .connect_with(connect_options)
        .await
    {
        Ok(pool) => {
            info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    Ok(pool)
}
