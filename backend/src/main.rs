use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
