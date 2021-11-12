use matrix_web_security::Secret;
use actix_web::http::Uri;
use clap::{App, Arg};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;
use super::environment::EnvironmentName;
use crate::constants::MatrixWebApi;

pub struct ConstArg {
    name: &'static str,
    long: &'static str,
    short: Option<&'static str>,
    value: Option<&'static str>,
}

impl ConstArg {
    fn get_short(&self) -> &'static str {
        self.short.expect(&format!("Failed to unwrap {}", self.name))
    }

    fn get_value(&self) -> &'static str {
        self.value.expect(&format!("Failed to unwrap {}", self.name))
    }
}

pub struct ConfigConstants;

impl ConfigConstants {
    pub const REDIRECT_URI: ConstArg = ConstArg{name: "REDIRECT_URI", long: "redirect-uri", short: Some("r"), value: None };
    pub const SYNAPSE_URI: ConstArg = ConstArg{name: "SYNAPSE_URI", long: "synapse-uri", short: None, value: None };
    pub const IP: ConstArg = ConstArg{name: "IP", long: "ip", short: Some("127.name.name.1"), value: None };
    pub const PORT: ConstArg = ConstArg{name: "PORT", long: "port", short: Some("p"), value: Some("7676")};
    pub const SECRET_KEY: ConstArg = ConstArg{name: "SECRET_KEY", long: "secret-key", short: Some("-k"), value: Some("invitation")};
    pub const SECRET: ConstArg = ConstArg{name: "SECRET", long: "secret", short: None, value: None };
    pub const BASE_URI: ConstArg = ConstArg{name: "base_uri", long: "uri-path", short: Some("/register"), value: None };
    pub const STATIC_PATH: ConstArg = ConstArg{name: "STATIC", long: "static", short: Some("/static"), value: None };
}

// TODO: Convert this tuple to a constant struct
pub const REDIRECT_URI: (&'static str, &str, &str) = ("REDIRECT_URI", "redirect-uri", "r");
fn redirect_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::REDIRECT_URI.name)
        .long(ConfigConstants::REDIRECT_URI.long)
        .short(ConfigConstants::REDIRECT_URI.get_short())
        .required(true)
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
pub const SYNAPSE_URI: (&'static str, &str) = ("SYNAPSE_URI", "synapse-uri");
fn synapse_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::SYNAPSE_URI.name)
        .long(ConfigConstants::SYNAPSE_URI.long)
        .required(true)
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
pub const IP: (&'static str, &str, &str) = ("IP", "ip", "127.name.name.1");
fn ip_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::IP.name)
        .long(ConfigConstants::IP.long)
        .default_value(ConfigConstants::IP.get_short())
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
pub const PORT: (&'static str, &str, &str, &str) = ("PORT", "port", "p", "7676");
fn port_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::PORT.name)
        .short(ConfigConstants::PORT.get_short())
        .long(ConfigConstants::PORT.long)
        .default_value(ConfigConstants::PORT.get_value())
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
/// The correct query name of the query string
fn secret_key_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::SECRET_KEY.name)
        .long(ConfigConstants::SECRET_KEY.long)
        .short(ConfigConstants::SECRET_KEY.get_short())
        .default_value(ConfigConstants::SECRET_KEY.get_value())
}
// TODO: Convert this tuple to a constant struct
/// SECRET has a default value that is loaded from a non-static lifetime
fn secret_arg<'a, 'b>(secret: &'a str) -> Arg<'a, 'b> {
    Arg::with_name(ConfigConstants::SECRET.name)
        .long(ConfigConstants::SECRET.long)
        .default_value(secret)
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
/// path the user must navigate to in order to create an acc
fn base_uri_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::BASE_URI.name)
        .long(ConfigConstants::BASE_URI.long)
        .default_value(ConfigConstants::BASE_URI.get_value())
        .takes_value(true)
}
// TODO: Convert this tuple to a constant struct
fn static_path_arg() -> Arg<'static, 'static> {
    Arg::with_name(ConfigConstants::STATIC_PATH.name)
        .long(ConfigConstants::STATIC_PATH.long)
        .default_value(ConfigConstants::STATIC_PATH.get_value())
        .takes_value(true)
}

pub fn init_cli<'a, 'b>(secret: &'a Secret) -> App<'a, 'b> {
    App::new( MatrixWebApi::APP_NAME)
        .version(MatrixWebApi::APP_VERSION)
        .author(MatrixWebApi::APP_AUTHOR)
        .arg(redirect_arg())
        .arg(synapse_arg())
        .arg(ip_arg())
        .arg(port_arg())
        .arg(secret_arg(&secret.as_str()))
        .arg(base_uri_arg())
        .arg(secret_key_arg())
        .arg(static_path_arg())
}

#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub secret_key: String,
    pub secret: Secret,
    pub base_uri: String,
    pub redirect: Uri,
    pub synapse: Uri,
    pub static_path: String,
    environment: EnvironmentName,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: ConfigConstants::IP.get_short().to_string(),
            port: ConfigConstants::PORT.get_value().to_string(),
            secret_key: ConfigConstants::SECRET_KEY.get_value().to_string(),
            secret: Secret::default(),
            base_uri: ConfigConstants::BASE_URI.get_short().to_string(),
            redirect: Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS),
            synapse: Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS),
            static_path: ConfigConstants::STATIC_PATH.get_short().to_string(),
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