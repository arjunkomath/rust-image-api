mod routes;
mod utils;

use actix_files as fs;
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

      GET /v1/flip/<orientation>?url=<image-url>
        flip image from `<image-url>` to `<orientation>` orientation
        orientation: horizontal, vertical

      GET /v1/grayscale?url=<image-url>
        convert image from `<image-url>` to grayscale

      GET /v1/brighten/<value>?url=<image-url>
        brighten image from `<image-url>` by `<value>`, negative values decrease the brightness and positive values increase it

      GET /v1/blur/<sigma>?url=<image-url>
        blur image from `<image-url>` with `<sigma>` sigma (this is a slow endpoint and could potentially timeout)
    ")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .body("success")
}

#[get("/test")]
async fn test() -> impl Responder {
    fs::NamedFile::open("templates/test.html")
        .unwrap_or_else(|_| panic!("Failed to open the HTML file"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .service(hello)
            .service(health)
            .service(test)
            .service(
                web::scope("/v1")
                    .wrap(utils::middleware::ImageParser)
                    .service(
                        web::scope("/resize")
                            .service(routes::resize::resize_by_width)
                            .service(routes::resize::resize_by_height),
                    )
                    .service(routes::convert::convert_type)
                    .service(routes::flip::flip_orientation)
                    .service(routes::blur::blur_image)
                    .service(routes::grayscale::grayscale)
                    .service(routes::brighten::brighten),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
