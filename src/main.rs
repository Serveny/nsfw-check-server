use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web::Data, App, HttpResponse, HttpServer, Responder};
use image::{DynamicImage, ImageFormat};
use nsfw::{create_model, examine, Model};
use std::io::BufReader;

const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(create_model(MODEL).expect("Cant load model")))
            .service(check)
    })
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}

#[derive(Debug, MultipartForm)]
pub struct CheckRequest {
    #[multipart]
    pub image: TempFile,
}

#[post("/check")]
async fn check(form: MultipartForm<CheckRequest>, model: Data<Model>) -> impl Responder {
    // Read image
    let img = match read_img(&form.image) {
        Ok(img) => img,
        Err(err_msg) => return HttpResponse::BadRequest().body(err_msg),
    };

    let result = match examine(&model, &img.into()) {
        Ok(img) => img,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    println!("{result:#?}");
    HttpResponse::Ok().body(format!("{result:#?}"))
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
