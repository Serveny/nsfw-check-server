use actix_multipart::form::tempfile::TempFile;
use actix_web::HttpResponse;
use image::{load_from_memory, load_from_memory_with_format, DynamicImage, ImageFormat};
use nsfw::{
    examine,
    model::{Classification, Metric},
    Model,
};
use reqwest::{header::CONTENT_TYPE, Client};
use std::io::BufReader;

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
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let format = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|format| format.to_str().ok())
        .and_then(|format| ImageFormat::from_mime_type(format));
    let bytes = response
        .bytes()
        .await
        .map_err(|err| err.to_string())?
        .to_vec();

    match format {
        Some(format) => load_from_memory_with_format(&bytes, format),
        None => load_from_memory(&bytes),
    }
    .map_err(|err| err.to_string())
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
