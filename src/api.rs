use crate::utils::{check_image, fetch_image, is_allowed, read_img};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{Data, Query},
    HttpResponse, Responder,
};
use nsfw::{examine, Model};
use serde::Deserialize;

#[derive(Debug, MultipartForm)]
pub struct CheckUploadRequest {
    #[multipart]
    pub image: TempFile,
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

#[get("/is_allowed")]
pub async fn is_allowed_url(info: Query<CheckUrlRequest>, model: Data<Model>) -> impl Responder {
    let img = match fetch_image(&info.url).await {
        Ok(img) => img,
        Err(err_msg) => return HttpResponse::BadRequest().body(err_msg),
    };
    let classifications = match examine(&model, &img.into()) {
        Ok(res) => res,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    HttpResponse::Ok().json(is_allowed(classifications))
}

#[post("/is_allowed")]
pub async fn is_allowed_upload(
    form: MultipartForm<CheckUploadRequest>,
    model: Data<Model>,
) -> impl Responder {
    let img = match read_img(&form.image) {
        Ok(img) => img,
        Err(err_msg) => return HttpResponse::BadRequest().body(err_msg),
    };

    let classifications = match examine(&model, &img.into()) {
        Ok(res) => res,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    HttpResponse::Ok().json(is_allowed(classifications))
}
