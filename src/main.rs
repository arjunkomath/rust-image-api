#[macro_use]
extern crate rocket;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      GET /image/w/<max-width>?url=<image-url>
          resize and serve the image from `<image-url>` to `<max-width>` pixels wide, without changing the aspect ratio
        
      GET /image/h/<max-height>?url=<image-url>
          resize and serve the image from `<image-url>` to `<max-height>` pixels tall, without changing the aspect ratio
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/image",
        routes![
            routes::resize::resize_by_width,
            routes::resize::resize_by_height
        ],
    )
}
