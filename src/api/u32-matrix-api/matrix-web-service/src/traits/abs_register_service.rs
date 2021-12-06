use crate::service::RegisterService;
use std::pin::Pin;
use std::future::Future;
use matrix_http_client::MatrixClient;
use std::sync::Arc;

pub trait AbsRegisterService {
    fn register_user(self) -> Pin<Box<dyn Future<Output=Result<(),()>>>>;
}