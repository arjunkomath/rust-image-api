use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

#[get("/crop/{x}/{y}/{width}/{height}")]
pub async fn handler(
    req: HttpRequest,
    params: web::Path<(u32, u32, u32, u32)>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let (x, y, width, height) = params.into_inner();

    ImageResponse {
        data: payload.image.clone().crop(x, y, width, height),
        format: auto_image_format(&req),
    }
    .try_into()
}
