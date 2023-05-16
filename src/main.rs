mod routes;
mod utils;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("
    USAGE

      GET /image/w/<max-width>?url=<image-url>
          resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

      GET /image/h/<max-height>?url=<image-url>
          resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
    ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(routes::resize::resize_by_width)
            .service(routes::resize::resize_by_height)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
