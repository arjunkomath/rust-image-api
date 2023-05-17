use crate::utils::http::{ImageHelper, ImageSource};
use actix_web::{get, web, Responder};

#[get("/grayscale")]
pub async fn grayscale(query: web::Query<ImageSource>) -> impl Responder {
    if let Ok(image) = crate::utils::http::get_image_from_url(&query.url).await {
        let image = image.grayscale();

        if let Ok(result) = ImageHelper::new(image).png_response() {
            return result;
        }
    }

    crate::utils::http::empty_response()
}
