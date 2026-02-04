use actix_web::{web, App, HttpServer, middleware};
use actix_files as fs;
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod db;
mod auth;
mod utils;

use db::Database;

pub struct AppState {
    pub db: Database,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://blog.db".to_string());
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    log::info!("Starting blog engine server...");
    
    let db = Database::new(&database_url).await.expect("Failed to connect to database");
    db.init().await.expect("Failed to initialize database");
    
    log::info!("Database initialized successfully");

    let app_state = web::Data::new(AppState {
        db,
        jwt_secret,
    });

    log::info!("Server starting at http://{}:{}", host, port);

    HttpServer::new(move || {
        let tera = tera::Tera::new("templates/**/*.html").expect("Failed to initialize Tera");
        
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .configure(handlers::config)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
