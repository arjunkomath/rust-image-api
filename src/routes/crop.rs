use crate::utils::http::{EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse, Result};

#[get("/crop/{x}/{y}/{width}/{height}")]
pub async fn crop_image(
    params: web::Path<(u32, u32, u32, u32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let (x, y, width, height) = params.into_inner();

    ImageResponse {
        data: payload.image.clone().crop(x, y, width, height),
        format: image::ImageFormat::Png,
    }
    .try_into()
}
