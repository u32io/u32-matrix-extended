use serde::{Deserialize, Serialize};
use crate::constants::AuthenticationType;

/// `AuthFlow` represents the [login flow](https://matrix.org/docs/spec/client_server/r0.6.1#get-matrix-client-r0-login)
/// with is both sent to and received by Matrix Synapse.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFlow {
    #[serde(rename = "type")]
    authentication_type: AuthenticationType,
}

impl From<AuthenticationType> for AuthFlow {
    fn from(auth: AuthenticationType) -> Self {
        Self {
            authentication_type: auth
        }
    }
}

impl AuthFlow {
    /// Creates an `AuthFlow` with an `authentication_type` of `AuthenticationType::Dummy`. Typically
    /// used for creating [`RegisterRequest`](struct@crate::model::RegisterRequest) types.
    pub fn dummy() -> Self {
        Self::from(AuthenticationType::Dummy)
    }

    pub fn authentication_type(&self) -> &AuthenticationType {
        &self.authentication_type
    }
}

#[derive(Deserialize)]
pub struct AuthFlowCollection {
    pub flows: Vec<AuthFlow>,
}

#[cfg(test)]
mod test {
    use super::AuthenticationType;
    use super::AuthFlow;

    #[test]
    fn flow_deserializes_from_json() {
        let json = "
        {
            \"type\": \"m.login.password\"
        }";

        let flow = serde_json::from_str(&json);
        assert!(flow.is_ok());

        let flow: AuthFlow = flow.unwrap();
        assert_eq!(AuthenticationType::Password, flow.authentication_type);
    }
}
