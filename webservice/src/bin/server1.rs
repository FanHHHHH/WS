use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// 配置 route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 配置handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Atix Web Service is running")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
