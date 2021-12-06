use actix_web::{post, Responder, web};
use matrix_web_dto::RegisterDTO;
use actix_web::web::Json;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_register);
}

#[post("/v1/register")]
async fn post_register(dto: Json<RegisterDTO>) -> impl Responder {
    "ok"
}