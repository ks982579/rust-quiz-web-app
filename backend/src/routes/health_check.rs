//! backend/src/routes/health_check.rs
//! Small endpoint whose only purpose is to ensure application is alive.
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

/// Endpoint function for GET /health-check.
/// it only sends back 200 OK and HTML response (TODO: change in future)
/// Leaving in for first release - could be useful in future for checking stability.
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
    <title>HEALTH CHECK</title>
    <!-- <link href="css/style.css" rel="stylesheet"> -->
  </head>
  <body>
    <h1>Hello, world!</h1>
  </body>
</html>
            "#
        ))
}
