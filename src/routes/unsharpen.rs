use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

#[get("/unsharpen/{sigma}/{threshold}")]
pub async fn handler(
    req: HttpRequest,
    params: web::Path<(f32, i32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let (sigma, threshold) = params.into_inner();

    ImageResponse {
        data: payload.image.unsharpen(sigma, threshold),
        format: auto_image_format(&req),
    }
    .try_into()
}
