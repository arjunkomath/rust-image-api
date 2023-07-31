use crate::utils::http::{EmptyResponse, ImageResponse, ImageSource};
use actix_web::{get, web, HttpResponse, Result};

#[get("/flip/{orientation}")]
pub async fn flip_orientation(
    orientation: web::Path<String>,
    query: web::Query<ImageSource>,
) -> Result<HttpResponse, EmptyResponse> {
    let orientation = orientation.into_inner();

    let image = crate::utils::http::get_image_from_url(&query.url).await?;

    let image = match orientation.as_str() {
        "horizontal" => image.fliph(),
        "vertical" => image.flipv(),
        _ => image,
    };

    ImageResponse {
        data: image,
        format: image::ImageFormat::Png,
    }
    .try_into()
}
