use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/blur/{sigma}")]
pub async fn blur_image(
    sigma: web::Path<u32>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let sigma = sigma.into_inner();

    ImageResponse {
        data: payload.image.blur(sigma as f32),
        format: image::ImageFormat::Png,
    }
    .try_into()
}
