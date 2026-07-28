use crate::utils::http::{ApiError, ImagePayload, ImageResponse, auto_image_format};
use actix_web::{HttpRequest, HttpResponse, Result, get, web};

#[get("/rotate/{deg}")]
pub async fn handler(
    req: HttpRequest,
    deg: web::Path<String>,
    payload: ImagePayload,
) -> Result<HttpResponse, ApiError> {
    let deg = deg.into_inner();

    let image = match deg.as_str() {
        "90" => payload.image.rotate90(),
        "180" => payload.image.rotate180(),
        "270" => payload.image.rotate270(),
        _ => payload.image,
    };

    ImageResponse {
        data: image,
        format: auto_image_format(&req),
    }
    .try_into()
}
