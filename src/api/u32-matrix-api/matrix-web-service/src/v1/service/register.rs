use crate::v1::AbsRegisterService;
use matrix_http_client::model::RegisterRequest;
use matrix_http_client::{AbsMatrixClient, MatrixClient, MatrixClientError};
use matrix_web_dto::v1::user::RegisterUserDTO;
use matrix_web_model::v1::{RegisteredUser, User};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RegisterService {
    inner: InternalRegisterService,
}

impl AbsRegisterService for RegisterService {
    fn register_user(
        &self,
        dto: RegisterUserDTO,
    ) -> Pin<Box<dyn Future<Output = Result<RegisteredUser, MatrixClientError>> + '_>> {
        Box::pin(self.inner.register_user(dto))
    }
}

impl RegisterService {
    pub fn new(matrix_client: Arc<MatrixClient>) -> Self {
        Self {
            inner: InternalRegisterService { matrix_client },
        }
    }
}

struct InternalRegisterService {
    matrix_client: Arc<MatrixClient>,
}

impl InternalRegisterService {
    pub async fn register_user(
        &self,
        dto: RegisterUserDTO,
    ) -> Result<RegisteredUser, MatrixClientError> {
        let req = User::from(dto).into();

        self.matrix_client
            .post_register(&req)
            .await
            .map(|res| RegisteredUser {
                username: req.username,
                user_id: res.user_id,
                access_token: res.access_token,
                home_server: res.home_server,
                device_id: res.device_id,
            })
    }
}
