use actix_web::web::{Data, Json};
use actix_web::{post, web, Responder};
use matrix_http_client::MatrixClient;

pub fn init(cfg: &mut web::ServiceConfig) {
    //cfg.service(post_message);
}
//
// #[post("/send")]
// async fn post_message(
//     msg: Json<BasicMessage>,
//     matrix_client: Data<MatrixClient>,
// ) -> impl Responder {
//     "ok"
// }
