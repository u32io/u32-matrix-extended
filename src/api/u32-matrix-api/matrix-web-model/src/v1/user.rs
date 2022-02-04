use matrix_http_client::model::RegisterRequest;
use matrix_web_dto::v1::user::RegisterUserDTO;

pub struct User {
    username: String,
    password: Option<String>,
}

impl From<RegisterUserDTO> for User {
    fn from(src: RegisterUserDTO) -> Self {
        Self {
            username: src.user_name,
            password: Some(src.password),
        }
    }
}

impl From<RegisterRequest> for User {
    fn from(src: RegisterRequest) -> Self {
        Self {
            username: src.username,
            password: None,
        }
    }
}

impl From<User> for RegisterRequest {
    fn from(src: User) -> Self {
        Self::new(src.username, src.password.unwrap())
    }
}
