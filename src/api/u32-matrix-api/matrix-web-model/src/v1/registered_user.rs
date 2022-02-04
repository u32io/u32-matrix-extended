use super::User;
use matrix_http_client::model::LoginResponse;

pub struct RegisteredUser {
    pub username: String,
    pub user_id: String,
    pub access_token: String,
    pub home_server: String,
    pub device_id: Option<String>,
}
