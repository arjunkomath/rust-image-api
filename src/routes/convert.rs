use crate::utils::http::{ApiError, ImagePayload, ImageResponse};
use actix_web::{HttpResponse, Result, get, web};

#[get("/convert/{format}")]
pub async fn handler(
    format: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let format = format.into_inner();
    let format = match format.as_str() {
        "jpeg" => image::ImageFormat::Jpeg,
        "webp" => image::ImageFormat::WebP,
        _ => image::ImageFormat::Png,
    };

    ImageResponse {
        data: payload.image,
        format,
    }
    .try_into()
}
