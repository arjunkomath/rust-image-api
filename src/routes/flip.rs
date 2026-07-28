use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

#[get("/flip/{orientation}")]
pub async fn handler(
    req: HttpRequest,
    orientation: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let orientation = orientation.into_inner();

    let image = match orientation.as_str() {
        "horizontal" => payload.image.fliph(),
        "vertical" => payload.image.flipv(),
        _ => payload.image,
    };

    ImageResponse {
        data: image,
        format: auto_image_format(&req),
    }
    .try_into()
}
