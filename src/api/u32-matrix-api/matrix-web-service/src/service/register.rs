use crate::traits::AbsRegisterService;
use matrix_http_client::{AbsMatrixClient, MatrixClient};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RegisterService {
    matrix_client: Arc<MatrixClient>,
}

impl AbsRegisterService for RegisterService {
    fn register_user(&self) -> Pin<Box<dyn Future<Output = Result<(), ()>>>> {
        //self.matrix_client.post_register()
        todo!()
    }
}

impl RegisterService {
    pub fn new(matrix_client: Arc<MatrixClient>) -> Self {
        Self { matrix_client }
    }
}
