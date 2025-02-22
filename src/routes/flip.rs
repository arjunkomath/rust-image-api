use crate::utils::http::{auto_image_format, EmptyResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

#[get("/flip/{orientation}")]
pub async fn handler(
    req: HttpRequest,
    orientation: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, EmptyResponse> {
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
