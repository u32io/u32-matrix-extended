use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub access_token: String,
    pub home_server: String,
    #[serde(default)]
    pub device_id: Option<String>,
}

#[cfg(test)]
mod test {
    use super::LoginResponse;

    #[test]
    fn json_with_no_device_id_deserializes() {
        let json = "{
            \"user_id\": \"@example:localhost\",
            \"access_token\": \"QGV4YW1wbGU6bG9jYWxob3N0.AqdSzFmFYrLrTmteXc\",
            \"home_server\": \"localhost\"
        }";

        let login = serde_json::from_str(json);
        assert!(login.is_ok());

        let login: LoginResponse = login.unwrap();
        assert!(login.device_id.is_none());
    }
}
