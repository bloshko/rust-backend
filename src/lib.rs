use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

const HOST: &str = "127.0.0.1:8000";

async fn greet() -> impl Responder {
    "text"
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)).route("/health_check", web::get().to(health_check))
    }).bind(HOST)?.run();

    Ok(server)
}
