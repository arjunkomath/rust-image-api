use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, get};

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
