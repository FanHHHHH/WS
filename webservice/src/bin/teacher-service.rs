use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../dbaccess/mod.rs"]
mod db_access;
#[path = "../error.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // 从.env文件加载环境变量
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // 初始化AppState
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm Ok".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(courses_routes)
    };

    HttpServer::new(app).bind("localhost:3000")?.run().await
}
