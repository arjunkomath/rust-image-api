use crate::utils::http::{auto_image_format, ApiError, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

#[get("/blur/{sigma}")]
pub async fn handler(
    req: HttpRequest,
    sigma: web::Path<f32>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    ImageResponse {
        data: payload.image.blur(sigma.into_inner()),
        format: auto_image_format(&req),
    }
    .try_into()
}
