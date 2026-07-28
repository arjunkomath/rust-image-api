use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, get, web};

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
