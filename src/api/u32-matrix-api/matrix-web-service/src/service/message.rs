use matrix_http_client::MatrixClient;
use std::sync::Arc;

pub struct MessageService {
    matrix_client: Arc<MatrixClient>,
}

