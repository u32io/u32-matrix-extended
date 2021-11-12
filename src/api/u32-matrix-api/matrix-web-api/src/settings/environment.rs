// TODO: Move this file to its own external crate (we'll need this env standard for many other projects)
use std::env;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EnvironmentName {
    LOCAL,
    DEV,
    TEST,
    PROD,
    UNKNOWN,
}

impl From<&str> for EnvironmentName {
    fn from(val: &str) -> Self {
        match val {
            "LOCAL" | "local" => EnvironmentName::LOCAL,
            "DEV" | "dev" => EnvironmentName::DEV,
            "TEST" | "test" => EnvironmentName::TEST,
            "PROD" | "prod" => EnvironmentName::PROD,
            _ => EnvironmentName::UNKNOWN,
        }
    }
}

impl EnvironmentName {
    const GLOBAL_VARIABLE_NAME: &'static str = "RUNTIME_ENVIRONMENT";

    pub fn new() -> EnvironmentName {
        match env::var(Self::GLOBAL_VARIABLE_NAME) {
            Ok(val) => Self::from(val.as_str()),
            Err(_) => EnvironmentName::UNKNOWN,
        }
    }
}
