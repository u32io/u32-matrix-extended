use serde::{Deserialize, Serialize};
use crate::constants::AuthenticationType;

// TODO: Rename `Flow` to something like `AuthenticationFlow`
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFlow {
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
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
