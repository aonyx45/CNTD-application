use actix_web::{get, App, HttpServer, Responder};
use rand::{distributions::Alphanumeric, Rng}; // 0.8

#[get("/")]
async fn greet() -> impl Responder {
    let APP_NAME: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .map(char::from)
    .collect();
    return format!("{}{}{}{}", "<h1>Hello from Python API : ", APP_NAME, "</h1>", "<p>Return <a href=\"/\">home</a></p>")
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
