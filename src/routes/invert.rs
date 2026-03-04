use crate::utils::http::{auto_image_format, ApiError, ImagePayload, ImageResponse};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/invert")]
pub async fn handler(req: HttpRequest, payload: ImagePayload) -> Result<HttpResponse, ApiError> {
    let mut image = payload.image;
    image.invert();

    ImageResponse {
        data: image,
        format: auto_image_format(&req),
    }
    .try_into()
}
