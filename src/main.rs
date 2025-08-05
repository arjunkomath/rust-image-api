mod routes;
mod utils;

use actix_files as fs;
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    middleware, web, App, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .body("
    USAGE

      GET /v1/resize/w/<max-width>?url=<image-url>
        resize image `<max-width>` pixels wide, without changing the aspect ratio

      GET /v1/resize/h/<max-height>?url=<image-url>
        resize image to `<max-height>` pixels tall, without changing the aspect ratio

      GET /v1/crop/<x>/<y>/<width>/<height>?url=<image-url>
        crop image to `<width>`x`<height>` pixels, starting from position (`<x>`, `<y>`)
      
      GET /v1/convert/<format>?url=<image-url>
        convert image to `<format>` format
        format: png, jpeg, webp

      GET /v1/flip/<orientation>?url=<image-url>
        flip image  to `<orientation>` orientation
        orientation: horizontal, vertical

      GET /v1/grayscale?url=<image-url>
        convert image to grayscale

      GET /v1/invert?url=<image-url>
        invert image

      GET /v1/brighten/<value>?url=<image-url>
        brighten image by `<value>`, negative values decrease the brightness and positive values increase it

      GET /v1/blur/<sigma>?url=<image-url>
        blur image with `<sigma>` sigma (this is a slow endpoint and could potentially timeout)

      GET /v1/rotate/<deg>?url=<image-url>
        rotate image by `<deg>` degrees, degree can be 90, 180, 270

      GET /v1/unsharpen/<sigma>/<threshold>?url=<image-url>
        unsharpen image, sigma is the amount to blur the image by, threshold is a control of how much to sharpen

      GET /v1/next?url=<image-url>&w=<width>&h=<height>
        resize image to `<width>`x`<height>` pixels, without changing the aspect ratio
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
    let file = fs::NamedFile::open("templates/test.html")
        .unwrap_or_else(|_| panic!("Failed to open the HTML file"));

    file.customize()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);

    println!("Starting image server on port {port}");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .wrap(middleware::Compress::default())
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
                    .service(routes::convert::handler)
                    .service(routes::crop::handler)
                    .service(routes::flip::handler)
                    .service(routes::blur::handler)
                    .service(routes::grayscale::handler)
                    .service(routes::invert::handler)
                    .service(routes::brighten::handler)
                    .service(routes::unsharpen::handler)
                    .service(routes::rotate::handler)
                    .service(routes::next_image::handler),
            )
    })
    .bind(("::", port))?
    .run()
    .await?;

    Ok(())
}
