use actix_web::{
    get, post,
    web::{Path, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

#[get("")]
async fn get_files() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[post("")]
async fn upload_file(req: HttpRequest) -> impl Responder {
    // read content type, only accept a plain binary stream (no compression), maybe use middleware for this?

    HttpResponse::Ok().body("")
}

#[get("/{file_id}")]
async fn get_file_by_id(path: Path<u64>) -> impl Responder {
    let file_id = path.into_inner();

    HttpResponse::Ok().body("")
}

pub(crate) fn file_endpoint_config(cfg: &mut ServiceConfig) {
    cfg.service(get_files)
        .service(upload_file)
        .service(upload_file);
}