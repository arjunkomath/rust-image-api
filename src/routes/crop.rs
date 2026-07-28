use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

#[get("/crop/{x}/{y}/{width}/{height}")]
pub async fn handler(
    req: HttpRequest,
    params: web::Path<(u32, u32, u32, u32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let (x, y, width, height) = params.into_inner();

    ImageResponse {
        data: payload.image.clone().crop(x, y, width, height),
        format: auto_image_format(&req),
    }
    .try_into()
}
