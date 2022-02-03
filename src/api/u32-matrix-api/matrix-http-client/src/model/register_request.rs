use serde::{Serialize};
use crate::model::AuthFlow;

/// `RegisterRequest` is a payload used for registering new users with Matrix Synapse.
/// It contains a `username`, `password`, and `AuthFlow`, which must always be have an
/// `authentication_type` of `AuthenticationType::Dummy`.
#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    password: String,
    auth: AuthFlow,
}

impl RegisterRequest {
    /// Instantiates a new `RegisterRequest` with the private field `auth` as `AuthFlow::dummy()`.
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            auth: AuthFlow::dummy()
        }
    }
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
            auth: AuthFlow::dummy()
        };

        let json = serde_json::to_string(&registration);

        assert!(json.is_ok());
    }
}