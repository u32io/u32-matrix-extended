use std::convert::TryFrom;
use std::path::Path;
use actix_web::{web, App, HttpServer};
use actix_web::client::Client;
use matrix_web_api::controller;
use matrix_http_client::{ApiUriBuilder, MatrixClient, ClientConfig};
use matrix_web_security::Secret;
use std::fs;
use matrix_web_api::constants::MatrixWebApi;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let secret = Secret::default();
    fs::write( MatrixWebApi::SECRET_FILE_NAME, secret.as_str())?;

    let server = HttpServer::new(|| {
        let client_config = ClientConfig::try_from(Path::new(".client.json"))
            .unwrap();
        let api_uri_builder = ApiUriBuilder::new(client_config.authority.as_str(), client_config.client_api.as_str())
            .unwrap();
        let actix_client = Client::default();

        let matrix_client = MatrixClient::new(api_uri_builder, actix_client);

        App::new()
            .data(matrix_client)
            .service(web::scope("/matrix/message/v1").configure(controller::v1::init_message_controller)
        )
    });

    server.bind("127.0.0.1:7676")?.run().await
}
