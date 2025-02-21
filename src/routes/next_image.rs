use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};
use image::GenericImageView;
use serde::Deserialize;

#[derive(Deserialize)]
struct ResizeOptions {
    w: Option<u32>,
    h: Option<u32>,
}

#[get("/next")]
pub async fn next_image(
    options: web::Query<ResizeOptions>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let width = options.w;
    let height = options.h;

    let (original_width, original_height) = payload.image.dimensions();

    let resized_image = match (width, height) {
        (Some(width), Some(height)) => {
            payload
                .image
                .resize_exact(width, height, image::imageops::FilterType::Triangle)
        }
        (Some(width), None) => {
            let new_height = (width as f32 * original_height as f32 / original_width as f32) as u32;
            payload
                .image
                .resize_exact(width, new_height, image::imageops::FilterType::Triangle)
        }
        (None, Some(height)) => {
            let new_width = (height as f32 * original_width as f32 / original_height as f32) as u32;
            payload
                .image
                .resize_exact(new_width, height, image::imageops::FilterType::Triangle)
        }
        _ => payload.image,
    };

    ImageResponse {
        data: resized_image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
