use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/app").route("/", web::get().to(index))))
        .bind("localhost:8090")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("HealthCheck: Server is up")
}
