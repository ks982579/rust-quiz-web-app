use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

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
    <h1>Hello, world!</h1>
  </body>
</html>
            "#
        ))
}
