use actix_web::{get, Responder, HttpResponse, HttpServer, App};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from Rust and MongoDB.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
