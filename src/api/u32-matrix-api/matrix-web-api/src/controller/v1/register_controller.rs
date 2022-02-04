use actix_web::http::header::CONTENT_TYPE;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse, Responder};
use matrix_web_dto::v1::user::RegisterUserDTO;
use matrix_web_service::v1::AbsRegisterService;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_register);
}

#[post("/v1/register")]
async fn post_register(
    service: Data<Box<dyn AbsRegisterService>>,
    dto: Json<RegisterUserDTO>,
) -> impl Responder {
    if dto.password != dto.re_password {
        return HttpResponse::BadRequest()
            .header(CONTENT_TYPE, "text/html")
            .body("Passwords do not match");
    }

    let result = service.register_user(dto.into_inner()).await;

    match result {
        Ok(user) => HttpResponse::Accepted().finish(),
        Err(error) => HttpResponse::BadGateway().finish(),
    }
}
