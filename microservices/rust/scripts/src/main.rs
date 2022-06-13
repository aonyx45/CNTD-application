use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello")]
async fn greet() -> impl Responder {
    return "<h1>Hello there !</h1>"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
