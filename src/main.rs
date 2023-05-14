#[macro_use]
extern crate rocket;

use image::GenericImageView;
use reqwest::Client;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::Request;
use std::io::Cursor;

struct BytesResponse(Vec<u8>);

impl<'r> Responder<'r, 'static> for BytesResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let bytes = self.0;

        Response::build()
            .header(ContentType::PNG)
            .status(Status::Ok)
            .raw_header("ETag", format!("{:x}", md5::compute(&bytes)))
            .raw_header("Cache-Control", "public, max-age=31536000, immutable")
            .sized_body(bytes.len(), std::io::Cursor::new(bytes))
            .ok()
    }
}

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

#[get("/w/<width>?<url>")]
async fn resize_by_width(width: u32, url: &str) -> BytesResponse {
    let client = Client::new();
    let response = client.get(url).send().await;

    if let Ok(response) = response {
        if response.status().is_success() {
            let bytes = response.bytes().await;
            if let Ok(bytes) = bytes {
                // Load the image from bytes
                if let Ok(image) = image::load_from_memory(&bytes) {
                    // Get the original dimensions
                    let (original_width, original_height) = image.dimensions();

                    // Calculate the new height while maintaining the aspect ratio
                    let new_height =
                        (width as f32 * original_height as f32 / original_width as f32) as u32;

                    // Resize the image
                    let resized_image = image.resize_exact(
                        width,
                        new_height,
                        image::imageops::FilterType::Lanczos3,
                    );

                    // Encode the resized image back to bytes
                    let mut resized_bytes = Vec::new();
                    if let Err(_) = resized_image.write_to(
                        &mut Cursor::new(&mut resized_bytes),
                        image::ImageOutputFormat::Png,
                    ) {
                        // Handle the case when encoding fails
                        return BytesResponse(Vec::new());
                    }

                    return BytesResponse(resized_bytes);
                }
            }
        }
    }

    // Handle the case when the image download fails
    // For example, you could return a default image or an error message
    BytesResponse(Vec::new()) // Return an empty byte vector
}

#[get("/h/<height>?<url>")]
async fn resize_by_height(height: u32, url: &str) -> BytesResponse {
    let client = Client::new();
    let response = client.get(url).send().await;

    if let Ok(response) = response {
        if response.status().is_success() {
            let bytes = response.bytes().await;
            if let Ok(bytes) = bytes {
                // Load the image from bytes
                if let Ok(image) = image::load_from_memory(&bytes) {
                    // Get the original dimensions
                    let (original_width, original_height) = image.dimensions();

                    // Calculate the new height while maintaining the aspect ratio
                    let new_width =
                        (height as f32 * original_width as f32 / original_height as f32) as u32;

                    // Resize the image
                    let resized_image = image.resize_exact(
                        new_width,
                        height,
                        image::imageops::FilterType::Lanczos3,
                    );

                    // Encode the resized image back to bytes
                    let mut resized_bytes = Vec::new();
                    if let Err(_) = resized_image.write_to(
                        &mut Cursor::new(&mut resized_bytes),
                        image::ImageOutputFormat::Png,
                    ) {
                        // Handle the case when encoding fails
                        return BytesResponse(Vec::new());
                    }

                    return BytesResponse(resized_bytes);
                }
            }
        }
    }

    // Handle the case when the image download fails
    // For example, you could return a default image or an error message
    BytesResponse(Vec::new()) // Return an empty byte vector
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/image", routes![resize_by_width, resize_by_height])
}
