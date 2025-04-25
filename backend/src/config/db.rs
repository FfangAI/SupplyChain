use sqlx::postgres::{PgPool, PgPoolOptions};
use log::info;

use super::Config;

pub async fn init_db_pool() -> PgPool {
    let config = Config::from_env();
    
    info!("Connecting to PostgreSQL database...");
    
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database connection pool")
}