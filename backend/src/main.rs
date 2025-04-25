mod api;
mod config;
mod models;
mod services;
mod utils;

use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use config::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    
    // Initialize database connection pool
    let db_pool = db::init_db_pool().await;
    
    println!("🚀 Server running at http://{}", address);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(api::routes::configure)
    })
    .bind(address)?
    .run()
    .await
}
