mod routes;
mod utils;

use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    middleware, web, App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("
    USAGE

      GET /v1/resize/w/<max-width>?url=<image-url>
        resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio

      GET /v1/resize/h/<max-height>?url=<image-url>
        resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
      
      GET /v1/convert/<format>?url=<image-url>
        convert and serve the image from `<image-url>` to `<format>` format
        format: png, jpeg, webp
    ")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .service(hello)
            .service(health)
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/resize")
                            .service(routes::resize::resize_by_width)
                            .service(routes::resize::resize_by_height),
                    )
                    .service(routes::convert::convert_type),
            );

        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
