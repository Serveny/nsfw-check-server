use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{check_upload, check_url, is_allowed_upload, is_allowed_url};
use log::LevelFilter;
use nsfw::create_model;

mod api;
mod utils;

const MODEL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter(Some("tract"), LevelFilter::Warn)
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(create_model(MODEL).expect("Cant load model")))
            .service(check_upload)
            .service(check_url)
            .service(is_allowed_url)
            .service(is_allowed_upload)
    })
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}
