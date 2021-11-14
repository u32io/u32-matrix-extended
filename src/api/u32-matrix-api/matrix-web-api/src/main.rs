use std::convert::TryFrom;
use std::path::Path;
use actix_web::{web, App, HttpServer};
use actix_web::client::Client;
use matrix_web_api::controller;
use matrix_http_client::{ApiUriBuilder, MatrixClient, ClientConfig};
use matrix_web_security::Secret;
use std::fs;
use matrix_web_api::constants::MatrixWebApi;
use matrix_web_api::settings::{Config, EnvironmentName, Cli, ConfConsts};
use log::{info};
use std::env::args;
use actix_web::http::Uri;
use std::str::FromStr;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Create a default configuration on application startup. Most of the values in this config will
    // be overridden by the CLI input.
    let mut config = Config::default();

    // Instantiate a logger based on the environment.
    if *config.get_environment() == EnvironmentName::PROD {
        log4rs::init_file(MatrixWebApi::LOGGING_PROD_FILE_NAME, Default::default())
            .expect(&format!("Unable to locate {}", MatrixWebApi::LOGGING_PROD_FILE_NAME))
    }
    else {
        log4rs::init_file(MatrixWebApi::LOGGING_FILE_NAME, Default::default())
            .expect(&format!("Unable to locate {}", MatrixWebApi::LOGGING_FILE_NAME))
    }
    info!("Starting {} version {}", MatrixWebApi::APP_NAME, MatrixWebApi::APP_VERSION);

    // Generate a new secret
    // TODO: In order to reach v1.0.0, we need to have bot which sends the secret to the chat
    let secret = Secret::default();
    fs::write( MatrixWebApi::SECRET_FILE_NAME, secret.as_str())?;
    info!("Generated a secret: {}", MatrixWebApi::SECRET_FILE_NAME);

    // Create a command line interface and collect the arguments
    let cli =  Cli::new(&secret);
    let args = cli.get_matches();

    // Map arguments to configuration values within the scope of `fn main`
    config = config.opts(|conf|{
        // assign server options
        args.value_of(ConfConsts::IP.name).map(|x|conf.server.ip = x.to_string());
        args.value_of(ConfConsts::PORT.name).map(|x|conf.server.port = x.to_string());
        args.value_of(ConfConsts::STATIC_PATH.name).map(|x|conf.server.static_path = x.to_string());
        // assign client options
        args.value_of(ConfConsts::HOME_SERVER.name).map(|x|conf.client.home_server = x.to_string());
        args.value_of(ConfConsts::AUTHORITY.name).map(|x|conf.client.authority = x.to_string());
        args.value_of(ConfConsts::CLIENT_API.name).map(|x|conf.client.client_api = x.to_string());

        args.value_of(ConfConsts::SECRET_KEY.name).map(|x|conf.secret_key = x.to_string());
        args.value_of(ConfConsts::SECRET.name).map(|x|conf.secret = Secret::from(x));
        args.value_of(ConfConsts::BASE_URI.name).map(|x|conf.base_uri = x.to_string());
        args.value_of(ConfConsts::REDIRECT_URI.name)
            .map(|x| conf.redirect = Uri::from_str(x)
                .unwrap_or(Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS)));
        args.value_of(ConfConsts::SYNAPSE_URI.name)
            .map(|x| conf.synapse = Uri::from_str(x)
                .unwrap_or(Uri::from_static(MatrixWebApi::DEFAULT_ADDRESS)));
    });
    info!("Config: {:?}", config);

    let server_cfg = config.server.clone();

    let server = HttpServer::new(move||{
        let api_uri_builder = ApiUriBuilder::new(config.client.authority.as_str(), config.client.client_api.as_str())
            .expect(&format!("Failed to construct an ApiUriBuilder"));
        let actix_client = Client::default();

        let matrix_client = MatrixClient::new(api_uri_builder, actix_client);

        App::new()
            .data(matrix_client)
            .service(web::scope("/message/v1")
                .configure(controller::v1::init_message_controller)
            .service(web::scope("/register/v1"))
                .configure(controller::v1::init_register_controller)
            .service(actix_files::Files::new("/static", config.server.static_path.as_str()))
        )
    });

    server.bind(&format!("{}:{}", server_cfg.ip, server_cfg.port))?
        .run()
        .await
}
