use matrix_web_security::Secret;
use actix_web::http::Uri;
use clap::{App, Arg};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;
use super::environment::EnvironmentName;
use crate::constants::MatrixWebApi;
use matrix_http_client::ClientConfig;

// TODO: Move ConstArg to a utility crate
pub struct ConstArg {
    pub name: &'static str,
    pub required: bool,
    pub(crate) long: &'static str,
    pub(crate) short: Option<&'static str>,
    pub(crate) value: Option<&'static str>,
}

impl ConstArg {
    pub(crate) fn get_short(&self) -> &'static str {
        self.short.expect(&format!("Failed to unwrap {}", self.name))
    }

    pub(crate) fn get_value(&self) -> &'static str {
        self.value.expect(&format!("Failed to unwrap {}", self.name))
    }
}

pub struct ConfigConstants;

impl ConfigConstants {
    pub const REDIRECT_URI: ConstArg = ConstArg {
        name: "REDIRECT_URI",
        long: "redirect-uri",
        short: Some("r"),
        value: Some("https://element.io"),
        required: false,
    };
    pub const SYNAPSE_URI: ConstArg = ConstArg {
        name: "SYNAPSE_URI",
        long: "synapse-uri",
        short: None,
        value: Some("https://matrix.org"),
        required: false,
    };
    pub const IP: ConstArg = ConstArg {
        name: "IP",
        long: "ip",
        short: None,
        value: Some("127.0.0.1"),
        required: false,
    };
    pub const PORT: ConstArg = ConstArg {
        name: "PORT",
        long: "port",
        short: Some("p"),
        value: Some("7676"),
        required: false,
    };
    pub const SECRET_KEY: ConstArg = ConstArg {
        name: "SECRET_KEY",
        long: "secret-key",
        short: Some("-k"),
        value: Some("invitation"),
        required: false,
    };
    pub const SECRET: ConstArg = ConstArg {
        name: "SECRET",
        long: "secret",
        short: None,
        value: None,
        required: false,
    };
    pub const BASE_URI: ConstArg = ConstArg {
        name: "base_uri",
        long: "uri-path",
        short: None,
        value: Some("/static"),
        required: false,
    };
    pub const STATIC_PATH: ConstArg = ConstArg {
        name: "STATIC",
        long: "static",
        short: None,
        value: Some("/static"),
        required: false,
    };
    pub const HOME_SERVER: ConstArg = ConstArg {
        name: "HOME_SERVER",
        long: "home-server",
        short: Some("s"),
        value: Some("matrix.org"),
        required: false,
    };
    pub const AUTHORITY: ConstArg = ConstArg {
        name: "AUTHORITY",
        long: "authority",
        short: Some("a"),
        value: Some("matrix"),
        required: false,
    };
    pub const CLIENT_API: ConstArg = ConstArg {
        name: "CLIENT_API",
        long: "client-api",
        short: None,
        value: Some("/_matrix/client/r0"),
        required: false,
    };
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub ip: String,
    pub port: String,
    pub static_path: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub secret_key: String,
    pub secret: Secret,
    pub base_uri: String,
    pub redirect: Uri,
    pub synapse: Uri,
    pub server: ServerConfig,
    pub client: ClientConfig,
    environment: EnvironmentName,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            secret_key: ConfigConstants::SECRET_KEY.get_value().to_string(),
            secret: Secret::default(),
            base_uri: ConfigConstants::BASE_URI.get_value().to_string(),
            redirect: Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS),
            synapse: Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS),
            server: ServerConfig {
                ip: ConfigConstants::IP.get_value().to_string(),
                port: ConfigConstants::PORT.get_value().to_string(),
                static_path: ConfigConstants::STATIC_PATH.get_value().to_string(),
            },
            client: ClientConfig {
                home_server: ConfigConstants::HOME_SERVER.get_value().to_string(),
                authority: ConfigConstants::AUTHORITY.get_value().to_string(),
                client_api: ConfigConstants::CLIENT_API.get_value().to_string(),
            },
            environment: EnvironmentName::new(),
        }
    }
}

impl Config {
    pub fn opts<F>(mut self, f: F) -> Self
        where
            F: Fn(&mut Self) -> (),
    {
        f(&mut self);
        self
    }

    pub fn get_environment(&self) -> &EnvironmentName {
        &self.environment
    }
}