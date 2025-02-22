use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/invert")]
pub async fn handler(
    req: HttpRequest,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
    let mut image = payload.image.clone();
    image.invert();

    ImageResponse {
        data: image,
        format: auto_image_format(&req),
    }
    .try_into()
}
