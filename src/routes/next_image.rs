use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct ResizeOptions {
    w: Option<u32>,
    h: Option<u32>,
}

#[get("/next")]
pub async fn handler(
    req: HttpRequest,
    options: web::Query<ResizeOptions>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let resized_image = match (options.w, options.h) {
        (Some(width), Some(height)) => {
            payload
                .image
                .resize_exact(width, height, image::imageops::FilterType::Triangle)
        }
        (Some(width), None) => {
            payload
                .image
                .resize(width, u32::MAX, image::imageops::FilterType::Triangle)
        }
        (None, Some(height)) => {
            payload
                .image
                .resize(u32::MAX, height, image::imageops::FilterType::Triangle)
        }
        _ => payload.image,
    };

    ImageResponse {
        data: resized_image,
        format: auto_image_format(&req),
    }
    .try_into()
}
