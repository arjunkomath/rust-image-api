use actix_web::{
    body::BoxBody,
    error,
    http::header::{CacheControl, CacheDirective, ContentType, ETag, EntityTag},
    FromRequest, HttpMessage, HttpRequest, HttpResponse,
};
use image::{DynamicImage, ImageFormat};
use reqwest::Client;
use serde::Deserialize;
use std::{
    future::{ready, Ready},
    io::Cursor,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ImageSource {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ImagePayload {
    pub image: DynamicImage,
}

impl ImagePayload {
    pub async fn from_url(client: &Client, url: &str) -> anyhow::Result<Self> {
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        let image = image::load_from_memory(&bytes)?;

        Ok(ImagePayload { image })
    }
}

impl FromRequest for ImagePayload {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let value = req.extensions().get::<ImagePayload>().cloned();

        let result = match value {
            Some(v) => Ok(v),
            None => Err(ApiError::BadRequest(
                "Missing or invalid 'url' query parameter".to_string(),
            )),
        };

        ready(result)
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    BadGateway(String),
    InternalError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "{msg}"),
            ApiError::BadGateway(msg) => write!(f, "{msg}"),
            ApiError::InternalError(msg) => write!(f, "{msg}"),
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
            .insert_header(("Content-Type", "application/json"))
            .body(format!("{{\"error\":\"{self}\"}}"))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::BadGateway(_) => actix_web::http::StatusCode::BAD_GATEWAY,
            ApiError::InternalError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub struct ImageResponse {
    pub data: DynamicImage,
    pub format: ImageFormat,
}

impl TryFrom<ImageResponse> for HttpResponse {
    type Error = ApiError;

    fn try_from(image_response: ImageResponse) -> Result<Self, ApiError> {
        match image_response.format {
            ImageFormat::Png => {
                let mut bytes = Vec::new();
                image_response
                    .data
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                    .map_err(|e| ApiError::InternalError(format!("Failed to encode image: {e}")))?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type(ContentType::png())
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(86400u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .body(bytes))
            }
            ImageFormat::Jpeg => {
                let mut bytes = Vec::new();
                image_response
                    .data
                    .write_to(
                        &mut Cursor::new(&mut bytes),
                        image::ImageOutputFormat::Jpeg(100),
                    )
                    .map_err(|e| ApiError::InternalError(format!("Failed to encode image: {e}")))?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type(ContentType::jpeg())
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(86400u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .body(bytes))
            }
            ImageFormat::WebP => {
                let mut bytes = Vec::new();
                image_response
                    .data
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP)
                    .map_err(|e| ApiError::InternalError(format!("Failed to encode image: {e}")))?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type("image/webp")
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(86400u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .body(bytes))
            }
            _ => Err(ApiError::BadRequest(format!(
                "Unsupported image format: {:?}",
                image_response.format
            ))),
        }
    }
}

pub fn auto_image_format(req: &HttpRequest) -> ImageFormat {
    let accept: Option<&str> = req.headers().get("accept").and_then(|h| h.to_str().ok());
    match accept {
        Some(accept) if accept.contains("image/webp") => ImageFormat::WebP,
        _ => ImageFormat::Png,
    }
}
