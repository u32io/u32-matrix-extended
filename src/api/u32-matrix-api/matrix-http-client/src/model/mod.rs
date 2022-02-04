pub use error_code::ErrorCode;
// Models
pub use auth_flow::AuthFlow;
pub use auth_flow::AuthFlowCollection;
pub use error_response::ErrorResponse;
pub use event_response::EventResponse;
pub use login_request::LoginIdentifier;
pub use login_request::LoginRequest;
pub use login_response::LoginResponse;
pub use message_request::MessageRequest;
pub use register_request::RegisterRequest;

mod auth_flow;
mod error_code;
mod error_response;
mod event_response;
mod login_request;
mod login_response;
mod message_request;
mod register_request;

pub enum ErrorKind {
    InvalidScheme,
    InvalidAuthenticationType,
}
