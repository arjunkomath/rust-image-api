use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse};
use image::ImageFormat;

#[get("/brighten/{value}")]
pub async fn brighten(
    payload: ImagePayload,
    value: web::Path<i32>,
) -> Result<HttpResponse, EmptyResponse> {
    ImageResponse {
        data: payload.image.brighten(value.into_inner()),
        format: ImageFormat::Png,
    }
    .try_into()
}
