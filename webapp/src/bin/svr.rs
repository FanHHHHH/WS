#[path = "../mod.rs"]
mod wa; // web application

use wa::{errors, handlers, models, routers};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_url = env::var("HOST_PORT").expect("HOST_URL not set");
    println!("Listening on: {}", &host_url);

    HttpServer::new(move || {
        // 注册tera静态文件信息
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_url)?
    .run()
    .await
}
