use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, get};

#[get("/grayscale")]
pub async fn handler(req: HttpRequest, payload: ImagePayload) -> Result<HttpResponse, ApiError> {
    ImageResponse {
        data: payload.image.grayscale(),
        format: auto_image_format(&req),
    }
    .try_into()
}
