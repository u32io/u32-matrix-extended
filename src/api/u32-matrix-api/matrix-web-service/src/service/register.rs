use matrix_http_client::MatrixClient;
use std::sync::Arc;
use crate::traits::AbsRegisterService;
use std::pin::Pin;
use std::future::Future;

pub struct RegisterService {
    matrix_client: Arc<MatrixClient>,
}

impl AbsRegisterService for RegisterService {
    fn register_user(self) -> Pin<Box<dyn Future<Output=Result<(), ()>>>> {
        todo!()
    }
}

impl RegisterService {
    pub fn new(matrix_client: Arc<MatrixClient>) -> Self {
        Self {
            matrix_client,
        }
    }
}