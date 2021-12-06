use actix_web::{post, Responder, web};
use matrix_web_dto::RegisterDTO;
use matrix_web_service::traits::AbsRegisterService;
use actix_web::web::{Json, Data};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_register);
}

#[post("/v1/register")]
async fn post_register(service: Data<Box<dyn AbsRegisterService>>, dto: Json<RegisterDTO>) -> impl Responder {
    "ok"
}