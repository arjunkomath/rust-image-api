use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

#[get("/blur/{sigma}")]
pub async fn handler(
    req: HttpRequest,
    sigma: web::Path<u32>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let sigma = sigma.into_inner();

    ImageResponse {
        data: payload.image.blur(sigma as f32),
        format: auto_image_format(&req),
    }
    .try_into()
}
