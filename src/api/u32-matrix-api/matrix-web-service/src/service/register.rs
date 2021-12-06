use matrix_http_client::MatrixClient;
use std::sync::Arc;

pub struct RegisterService {
    matrix_client: Arc<MatrixClient>,
}

