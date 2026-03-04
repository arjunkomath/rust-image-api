use crate::utils::http::{auto_image_format, ApiError, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/brighten/{value}")]
pub async fn handler(
    req: HttpRequest,
    payload: ImagePayload,
    value: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    ImageResponse {
        data: payload.image.brighten(value.into_inner()),
        format: auto_image_format(&req),
    }
    .try_into()
}
