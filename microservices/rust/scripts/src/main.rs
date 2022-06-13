use actix_web::{get, App, HttpServer, HttpResponse};
use rand::{distributions::Alphanumeric, Rng}; // 0.8

#[get("/")]
async fn greet() -> HttpResponse {
    let mut APP_NAME: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .map(char::from)
    .collect();
    APP_NAME =  format!("{}{}{}{}", "<h1>Hello from Rust API : ", APP_NAME, "</h1>", "<p>Return <a href=\"/\">home</a></p>");
    return HttpResponse::Ok().header("Content-Type", "text/html").body(APP_NAME);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
