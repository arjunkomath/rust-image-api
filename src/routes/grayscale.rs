use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/grayscale")]
pub async fn handler(
    req: HttpRequest,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    ImageResponse {
        data: payload.image.grayscale(),
        format: auto_image_format(&req),
    }
    .try_into()
}
