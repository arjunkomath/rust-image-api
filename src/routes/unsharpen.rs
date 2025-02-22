use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

#[get("/unsharpen/{sigma}/{threshold}")]
pub async fn handler(
    req: HttpRequest,
    params: web::Path<(f32, i32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let (sigma, threshold) = params.into_inner();

    ImageResponse {
        data: payload.image.unsharpen(sigma, threshold),
        format: auto_image_format(&req),
    }
    .try_into()
}
