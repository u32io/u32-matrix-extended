use crate::service::RegisterService;
use matrix_http_client::MatrixClient;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait AbsRegisterService {
    fn register_user(&self) -> Pin<Box<dyn Future<Output = Result<(), ()>>>>;
}
