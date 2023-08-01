use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/convert/{format}")]
pub async fn convert_type(
    format: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let format = format.into_inner();

    match format.as_str() {
        "png" => ImageResponse {
            data: payload.image,
            format: image::ImageFormat::Png,
        },
        "jpeg" => ImageResponse {
            data: payload.image,
            format: image::ImageFormat::Jpeg,
        },
        "webp" => ImageResponse {
            data: payload.image,
            format: image::ImageFormat::WebP,
        },
        _ => ImageResponse {
            data: payload.image,
            format: image::ImageFormat::Png,
        },
    }
    .try_into()
}
