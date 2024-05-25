use actix_web::{
    dev::Server, http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer,
};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Login</title>
    <!-- <link href="css/style.css" rel="stylesheet"> -->
  </head>
  <body>
    <!-- To send a POST request to this endpoint -->
    <h1>Hello, world!</h1>
  </body>
</html>
            "#
        ))
}

pub async fn anything(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Login</title>
    <!-- <link href="css/style.css" rel="stylesheet"> -->
  </head>
  <body>
    <!-- To send a POST request to this endpoint -->
    <h1>OK</h1>
  </body>
</html>
            "#
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(anything))
            .route("/health-check", web::get().to(health_check))
    })
    // .bind(("127.0.0.1", 8080))?
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
