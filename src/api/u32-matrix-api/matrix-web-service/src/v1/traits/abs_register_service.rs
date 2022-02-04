use matrix_http_client::{MatrixClient, MatrixClientError};
use matrix_web_dto::v1::user::RegisterUserDTO;
use matrix_web_model::v1::RegisteredUser;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait AbsRegisterService {
    fn register_user(
        &self,
        dto: RegisterUserDTO,
    ) -> Pin<Box<dyn Future<Output = Result<RegisteredUser, MatrixClientError>> + '_>>;
}
