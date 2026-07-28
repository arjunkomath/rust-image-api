use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

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
