use actix_web::{web::Data, App, HttpServer};
use api::{check_upload, check_url, is_allowed_url};
use nsfw::create_model;

mod api;
mod utils;

const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(create_model(MODEL).expect("Cant load model")))
            .service(check_upload)
            .service(check_url)
            .service(is_allowed_url)
    })
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}
