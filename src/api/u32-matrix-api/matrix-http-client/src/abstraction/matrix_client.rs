use crate::model::{
    AuthFlowCollection, EventResponse, LoginRequest, LoginResponse, MessageRequest, RegisterRequest,
};
use crate::MatrixClientError;
use std::future::Future;
use std::pin::Pin;
use urlencoding::Encoded;

pub trait AbsMatrixClient {
    fn get_login<'req>(
        &'req self,
    ) -> Pin<Box<dyn Future<Output = Result<AuthFlowCollection, MatrixClientError>> + 'req>>;

    fn post_login<'req>(
        &'req self,
        req: &'req LoginRequest,
    ) -> Pin<Box<dyn Future<Output = Result<LoginResponse, MatrixClientError>> + 'req>>;

    fn post_message<'req>(
        &'req self,
        msg: &'req MessageRequest,
        room_id: Encoded<&'req str>,
        access_token: &'req str,
    ) -> Pin<Box<dyn Future<Output = Result<EventResponse, MatrixClientError>> + 'req>>;

    fn post_register<'req>(
        &'req self,
        req: &'req RegisterRequest,
    ) -> Pin<Box<dyn Future<Output = Result<LoginResponse, MatrixClientError>> + 'req>>;
}
