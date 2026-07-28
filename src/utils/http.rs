use actix_web::{
    FromRequest, HttpRequest, HttpResponse,
    body::BoxBody,
    error,
    http::header::{CacheControl, CacheDirective, ETag, EntityTag},
    web,
};
use image::{DynamicImage, ImageFormat, ImageOutputFormat};
use reqwest::Client;
use serde::Deserialize;
use std::{future::Future, io::Cursor, pin::Pin};

#[derive(Deserialize)]
struct ImageSource {
    url: String,
}

pub struct ImagePayload {
    pub image: DynamicImage,
}

impl FromRequest for ImagePayload {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let source = web::Query::<ImageSource>::from_query(req.query_string());
        let client = req.app_data::<web::Data<Client>>().cloned();

        Box::pin(async move {
            let source = source.map_err(|_| {
                ApiError::BadRequest("Missing or invalid 'url' query parameter".to_string())
            })?;
            let client = client
                .ok_or_else(|| ApiError::InternalError("HTTP client not configured".to_string()))?;
            let response =
                client.get(&source.url).send().await.map_err(|error| {
                    ApiError::BadGateway(format!("Failed to fetch image: {error}"))
                })?;
            let bytes = response
                .bytes()
                .await
                .map_err(|error| ApiError::BadGateway(format!("Failed to fetch image: {error}")))?;
            let image = image::load_from_memory(&bytes)
                .map_err(|error| ApiError::BadGateway(format!("Failed to fetch image: {error}")))?;

            Ok(Self { image })
        })
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
        let (format, content_type) = match image_response.format {
            ImageFormat::Png => (ImageOutputFormat::Png, "image/png"),
            ImageFormat::Jpeg => (ImageOutputFormat::Jpeg(100), "image/jpeg"),
            ImageFormat::WebP => (ImageOutputFormat::WebP, "image/webp"),
            _ => Err(ApiError::BadRequest(format!(
                "Unsupported image format: {:?}",
                image_response.format
            )))?,
        };
        let mut bytes = Vec::new();
        image_response
            .data
            .write_to(&mut Cursor::new(&mut bytes), format)
            .map_err(|e| ApiError::InternalError(format!("Failed to encode image: {e}")))?;
        let etag = format!("{:x}", md5::compute(&bytes));

        Ok(HttpResponse::Ok()
            .content_type(content_type)
            .insert_header(CacheControl(vec![CacheDirective::MaxAge(86400u32)]))
            .insert_header(ETag(EntityTag::new_strong(etag)))
            .body(bytes))
    }
}

pub fn auto_image_format(req: &HttpRequest) -> ImageFormat {
    let accept: Option<&str> = req.headers().get("accept").and_then(|h| h.to_str().ok());
    match accept {
        Some(accept) if accept.contains("image/webp") => ImageFormat::WebP,
        _ => ImageFormat::Png,
    }
}
