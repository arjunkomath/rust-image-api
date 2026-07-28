mod routes;
mod utils;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::header::{CacheControl, CacheDirective},
    middleware, web,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .body(include_str!("../README.md"))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .body("success")
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/test.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080);

    println!("Starting image server on port {port}");

    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .wrap(middleware::Compress::default())
            .service(hello)
            .service(health)
            .service(test)
            .service(
                web::scope("/v1")
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
    .await
}
