use serde::{Deserialize};

#[derive(Deserialize)]
pub struct RegisterDTO {
    pub user_name: String,
    pub password: String,
    pub re_password: String,
}