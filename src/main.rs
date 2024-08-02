use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{Data, Query},
    App, HttpResponse, HttpServer, Responder,
};
use image::{load_from_memory, load_from_memory_with_format, DynamicImage, ImageFormat};
use nsfw::{create_model, examine, Model};
use reqwest::{header::CONTENT_TYPE, Client};
use serde::Deserialize;
use std::io::BufReader;

const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(create_model(MODEL).expect("Cant load model")))
            .service(check_upload)
            .service(check_url)
    })
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}

#[derive(Debug, MultipartForm)]
pub struct CheckUploadRequest {
    #[multipart]
    pub image: TempFile,
}

#[post("/check")]
async fn check_upload(
    form: MultipartForm<CheckUploadRequest>,
    model: Data<Model>,
) -> impl Responder {
    // Read image
    let img = match read_img(&form.image) {
        Ok(img) => img,
        Err(err_msg) => return HttpResponse::BadRequest().body(err_msg),
    };

    check_image(img, &model)
}

#[derive(Debug, Deserialize)]
pub struct CheckUrlRequest {
    pub url: String,
}

#[get("/check")]
async fn check_url(info: Query<CheckUrlRequest>, model: Data<Model>) -> impl Responder {
    // Read image
    let img = match fetch_image(&info.url).await {
        Ok(img) => img,
        Err(err_msg) => return HttpResponse::BadRequest().body(err_msg),
    };

    check_image(img, &model)
}

pub fn read_img(temp_file: &TempFile) -> Result<DynamicImage, &'static str> {
    let Ok(file) = std::fs::File::open(&temp_file.file) else {
        return Err("Cannot read file");
    };
    let reader = BufReader::new(file);
    let format = temp_file
        .content_type
        .as_ref()
        .ok_or("Can't read image format")?;
    let format = ImageFormat::from_mime_type(format).ok_or("Unknown image format")?;
    let img = image::load(reader, format).map_err(|_| "Image corrupt")?;

    Ok(img)
}

async fn fetch_image(url: &str) -> Result<DynamicImage, String> {
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

fn check_image(img: DynamicImage, model: &Model) -> HttpResponse {
    match examine(model, &img.into()) {
        Ok(res) => HttpResponse::Ok().body(format!("{res:#?}")),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }
}
