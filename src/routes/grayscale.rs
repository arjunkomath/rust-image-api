use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpResponse};
use image::ImageFormat;

#[get("/grayscale")]
pub async fn grayscale(payload: ImagePayload) -> Result<HttpResponse, EmptyResponse> {
    ImageResponse {
        data: payload.image.grayscale(),
        format: ImageFormat::Png,
    }
    .try_into()
}
