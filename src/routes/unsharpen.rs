use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/unsharpen/{sigma}/{threshold}")]
pub async fn unsharpen(
    params: web::Path<(f32, i32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let (sigma, threshold) = params.into_inner();

    ImageResponse {
        data: payload.image.unsharpen(sigma, threshold),
        format: image::ImageFormat::Png,
    }
    .try_into()
}
