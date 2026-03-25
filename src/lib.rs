use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use actix_web::dev::Server;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}


#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn deploy() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(index).service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}