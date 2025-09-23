use actix_cors::Cors;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{http::header, HttpResponse};
use image::{DynamicImage, ImageFormat, ImageReader};
use nsfw::{
    examine,
    model::{Classification, Metric},
    Model,
};
use reqwest::{header::CONTENT_TYPE, Client};
use std::io::{BufReader, Cursor};

pub fn cors_cfg() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
        ])
        .allow_any_origin()
        .supports_credentials()
        .max_age(3600)
}

pub fn read_img(temp_file: &TempFile) -> Result<DynamicImage, String> {
    let file = match std::fs::File::open(&temp_file.file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Cannot read file: {err:?}")),
    };
    let reader = BufReader::new(file);
    let format = temp_file
        .content_type
        .as_ref()
        .ok_or("Can't read image format")?;
    let format = ImageFormat::from_mime_type(format).ok_or("Unknown image format")?;
    let img = image::load(reader, format).map_err(|err| format!("Image corrupt: {err:?}"))?;

    Ok(img)
}

pub async fn fetch_image(url: &str) -> Result<DynamicImage, String> {
    let client = Client::builder()
        // Set user agent to prevent web scraping protections
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()
        .map_err(|err| err.to_string())?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    // Read format from content type header
    let format = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(ImageFormat::from_mime_type)
        // If no header, read format from url ending
        .or_else(|| ImageFormat::from_path(url).ok());

    let bytes = response.bytes().await.map_err(|err| err.to_string())?;

    let mut reader = ImageReader::new(Cursor::new(bytes));

    if let Some(format) = format {
        reader.set_format(format);
    }

    reader.decode().map_err(|err| err.to_string())
}

pub fn check_image(img: DynamicImage, model: &Model) -> HttpResponse {
    match examine(model, &img.into()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub fn is_allowed(classifications: Vec<Classification>) -> bool {
    let Some(max_score) = classifications
        .iter()
        .max_by(|a, b| a.score.total_cmp(&b.score))
    else {
        return true;
    };
    max_score.metric != Metric::Hentai && max_score.metric != Metric::Porn
}
