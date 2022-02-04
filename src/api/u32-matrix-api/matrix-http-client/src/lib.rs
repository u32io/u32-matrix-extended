mod api_uri_builder;
mod client_config;
mod matrix_client;
mod text_message;

pub mod abstraction;
pub mod constants;
pub mod error;
pub mod model;

pub use abstraction::AbsMatrixClient;
pub use api_uri_builder::ApiUriBuilder;
pub use client_config::ClientConfig;
pub use error::MatrixClientError;
pub use matrix_client::MatrixClient;
