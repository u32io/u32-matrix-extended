use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserDTO {
    pub user_name: String,
    pub password: String,
    pub re_password: String,
}
