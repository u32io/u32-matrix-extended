use serde::{Serialize};
use crate::model::AuthFlow;

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub auth: AuthFlow,
}

#[cfg(test)]
mod test {
    use super::RegisterRequest;
    use super::AuthFlow;
    use crate::constants::AuthenticationType;
    use chrono::prelude::*;
    use openssl::base64::encode_block;
    use openssl::sha::sha1;

    #[test]
    fn model_serializes_to_json() {
        let utc_now = Utc::now().timestamp().to_string();

        let registration = RegisterRequest {
            username: format!("test_bot_{}", utc_now),
            password: encode_block(&sha1(utc_now.as_bytes())),
            auth: AuthFlow {
                authentication_type: AuthenticationType::Password
            }
        };

        let json = serde_json::to_string(&registration);

        assert!(json.is_ok());
    }
}