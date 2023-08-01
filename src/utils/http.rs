use actix_web::{
    body::BoxBody,
    error,
    http::header::{CacheControl, CacheDirective, ContentType, ETag, EntityTag, IfNoneMatch},
    FromRequest, HttpMessage, HttpResponse,
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
    pub async fn from_url(url: &str) -> anyhow::Result<Self> {
        let client = Client::new();
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        let image = image::load_from_memory(&bytes)?;

        Ok(ImagePayload { image })
    }
}

impl FromRequest for ImagePayload {
    type Error = EmptyResponse;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let value = req.extensions().get::<ImagePayload>().cloned();

        let result = match value {
            Some(v) => Ok(v),
            None => Err(EmptyResponse {}),
        };

        ready(result)
    }
}

#[derive(Debug)]
pub struct EmptyResponse;

impl std::fmt::Display for EmptyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty response")
    }
}

impl From<anyhow::Error> for EmptyResponse {
    fn from(_: anyhow::Error) -> Self {
        EmptyResponse {}
    }
}

impl error::ResponseError for EmptyResponse {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Ok()
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
            .body(Vec::new())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }
}

pub struct ImageResponse {
    pub data: DynamicImage,
    pub format: ImageFormat,
}

impl TryFrom<ImageResponse> for HttpResponse {
    type Error = EmptyResponse;

    fn try_from(image_response: ImageResponse) -> Result<Self, EmptyResponse> {
        match image_response.format {
            ImageFormat::Png => {
                let mut bytes = Vec::new();
                image_response
                    .data
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                    .map_err(|_| EmptyResponse {})?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type(ContentType::png())
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .insert_header(IfNoneMatch::Items(vec![EntityTag::new(
                        false,
                        etag_value.to_owned(),
                    )]))
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
                    .map_err(|_| EmptyResponse {})?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type(ContentType::jpeg())
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .insert_header(IfNoneMatch::Items(vec![EntityTag::new(
                        false,
                        etag_value.to_owned(),
                    )]))
                    .body(bytes))
            }
            ImageFormat::WebP => {
                let mut bytes = Vec::new();
                image_response
                    .data
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP)
                    .map_err(|_| EmptyResponse {})?;

                let etag_value = format!("{:x}", md5::compute(&bytes));

                Ok(HttpResponse::Ok()
                    .content_type("image/webp")
                    .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
                    .insert_header(ETag(EntityTag::new_strong(etag_value.to_owned())))
                    .insert_header(IfNoneMatch::Items(vec![EntityTag::new(
                        false,
                        etag_value.to_owned(),
                    )]))
                    .body(bytes))
            }
            _ => {
                panic!("Unsupported image format: {:?}", image_response.format);
            }
        }
    }
}
