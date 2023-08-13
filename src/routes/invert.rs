use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpResponse};
use image::ImageFormat;

#[get("/invert")]
pub async fn invert(payload: ImagePayload) -> Result<HttpResponse, EmptyResponse> {
    let mut image = payload.image.clone();
    image.invert();

    ImageResponse {
        data: image,
        format: ImageFormat::Png,
    }
    .try_into()
}
