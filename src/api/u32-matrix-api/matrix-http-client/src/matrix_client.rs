use super::constants::RoomEventType;
use super::model::{
    ErrorResponse, EventResponse, AuthFlowCollection, LoginRequest, LoginResponse, MessageRequest,
};
use super::ApiUriBuilder;
use actix_web::client::{Client, ClientResponse};
use actix_web::http::{StatusCode, HeaderMap, HeaderValue};
use urlencoding::Encoded;
use std::future::Future;
use std::pin::Pin;
use crate::AbsMatrixClient;
use crate::error::{MatrixClientError, HttpResponseError};
use crate::model::RegisterRequest;
use actix_web::dev::{PayloadStream, Payload};
use actix_http::encoding::Decoder;
use serde::{Serialize, Deserialize};
use actix_http::http::Uri;
use serde::de::DeserializeOwned;
use actix_web::http::header::CONTENT_TYPE;

/// A template for building `GET` requests and mapping their `Err` to `MatrixClientErr`
macro_rules! http_get {
    ($http_client:expr, $uri:expr) => {
        $http_client
            .get($uri)
            .send()
            .await
            .map_err(|e| MatrixClientError::SendRequestError(e))
    };
}
/// A template for building `POST` requests and mapping their `Err` to `MatrixClientErr`.
/// This macro expects a body that serializes to json.
macro_rules! http_post {
    ($http_client:expr, $uri:expr, $json:expr) => {
        $http_client
            .post($uri)
            .send_json($json)
            .await
            .map_err(|e| MatrixClientError::SendRequestError(e))
    };
}
/// A template for extracting json from `HttpResponse` and mapping both the `PayloadErr` and
/// `JsonDeserializationError` to `MatrixClientErr`
macro_rules! get_json {
    ($t:ty, $response:expr) => {{
        let bytes = $response
            .body()
            .await
            .map_err(|e| MatrixClientError::PayloadErr(e))?;

        let json: Result<$t, MatrixClientError> = serde_json::from_slice(&*bytes)
            .map_err(|e| MatrixClientError::JsonDeserializationError(e));
        json
    }};
}
/// I don't even know how to explain this one. It basically does everything.
macro_rules! try_convert_200 {
    ($http_response:expr, $model:ty) => {
        match $http_response.status() {
            StatusCode::OK => Ok(get_json!($model, $http_response)?),
            _ => Err(MatrixClientError::HttpResponseError(HttpResponseError {
                status: $http_response.status(),
                body: get_json!(ErrorResponse, $http_response)?,
            })),
        }
    };
}

pub struct MatrixClient {
    internal: InternalMatrixClient,
}

impl MatrixClient {
    pub fn new(api_uri: ApiUriBuilder, http_client: Client) -> Self {
        Self {
            internal: InternalMatrixClient {
                api_uri,
                http_client,
            }
        }
    }
}

impl AbsMatrixClient for MatrixClient {
    /// `GET` the authentication scheme of the matrix-synapse API
    fn get_login<'req>(&'req self) -> Pin<Box<dyn Future<Output=Result<AuthFlowCollection,MatrixClientError>> + 'req>> {
        Box::pin(self.internal.get_login())
    }
    /// `POST` the credentials of a user and expect a `200` response with an access token
    fn post_login<'req>(&'req self, req: &'req LoginRequest) -> Pin<Box<dyn Future<Output=Result<LoginResponse, MatrixClientError>> + 'req>> {
        Box::pin(self.internal.post_login(req))
    }
    /// `POST` a basic message and expect and expect a response that contains an event id
    /// ```bash
    /// curl -XPOST -d '{"msgtype":"m.text", "body":"hello"}' \
    ///     "https://API/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
    ///
    /// { "event_id": "EVENT ID" }
    /// ```
    fn post_message<'req>(
        &'req self,
        msg: &'req MessageRequest,
        room_id: Encoded<&'req str>,
        access_token: &'req str
    ) -> Pin<Box<dyn Future<Output=Result<EventResponse, MatrixClientError>> + 'req>> {
        Box::pin(self.internal.post_message(msg, room_id, access_token))
    }
    /// `POST` a registration payload and expect a `200` response with an access token
    /// ```bash
    /// curl -XPOST \
    ///     -d '{"username":"example", "password":"wordpass", "auth": {"type":"m.login.dummy"}}' \
    ///     "https://localhost:8448/_matrix/client/r0/register"
    ///
    /// {
    ///     "access_token": "QGV4YW1wbGU6bG9jYWxob3N0.AqdSzFmFYrLrTmteXc",
    ///     "home_server": "localhost",
    ///     "user_id": "@example:localhost"
    /// }
    /// ```
    fn post_register<'req>(&'req self, req: &'req RegisterRequest) -> Pin<Box<dyn Future<Output=Result<LoginResponse, MatrixClientError>> + 'req>> {
        Box::pin(self.internal.post_register(req))
    }
}

struct InternalMatrixClient {
    api_uri: ApiUriBuilder,
    http_client: Client,
}

impl InternalMatrixClient {
    /// `GET` the authentication scheme of the matrix-synapse API
    async fn get_login(&self) -> Result<AuthFlowCollection, MatrixClientError> {
        let mut response = http_get!(self.http_client, self.api_uri.login())?;
        try_convert_200!(response, AuthFlowCollection)
    }
    /// `POST` the credentials of a user and expect a `200` response with an access token
    async fn post_login(&self, req: &LoginRequest) -> Result<LoginResponse, MatrixClientError> {
        let mut response = http_post!(self.http_client, self.api_uri.login(), req)?;
        try_convert_200!(response, LoginResponse)
    }
    /// `POST` a basic message and expect and expect a response that contains an event id
    /// ```bash
    /// curl -XPOST -d '{"msgtype":"m.text", "body":"hello"}' \
    ///     "https://API/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
    ///
    /// { "event_id": "EVENT ID" }
    /// ```
    async fn post_message(
        &self,
        msg: &MessageRequest,
        room_id: Encoded<&str>,
        access_token: &str,
    ) -> Result<EventResponse, MatrixClientError> {
        let mut response = http_post!(
            self.http_client,
            self.api_uri
                .send(room_id, RoomEventType::Message, access_token),
            msg
        )?;
        try_convert_200!(response, EventResponse)
    }

    async fn post_register<'req>(&'req self, req: &'req RegisterRequest) -> Result<LoginResponse, MatrixClientError> {
        let mut response = self.http_post(self.api_uri.register(), req).await?;
        match response.status() {
            StatusCode::OK | StatusCode::ACCEPTED => Self::get_json(response).await,
            _ => {
                let error = HttpResponseError {
                    status: response.status(),
                    body: Self::get_json(response).await?
                };

                Err(MatrixClientError::HttpResponseError(error))
            }
        }
    }

    async fn http_get(&self, uri: &str) -> Result<ClientResponse<Decoder<Payload<PayloadStream>>>, MatrixClientError> {
        self.http_client
            .get(uri)
            .send()
            .await
            .map_err(|e|MatrixClientError::SendRequestError(e))
    }

    async fn http_post<T: Serialize>(&self, uri: Uri, model: &T) -> Result<ClientResponse<Decoder<Payload<PayloadStream>>>, MatrixClientError> {
        self.http_client
            .post(uri)
            .send_json(model)
            .await
            .map_err(|e|MatrixClientError::SendRequestError(e))
    }

    // TODO: Rename this method and possibly move it to a utility class
    async fn get_json<T: DeserializeOwned>(mut response: ClientResponse<Decoder<Payload<PayloadStream>>>) -> Result<T, MatrixClientError> {
        use actix_web::http::{HeaderMap, HeaderName, HeaderValue};

        let headers: &HeaderMap = response.headers();

        let content_type = headers.get(CONTENT_TYPE)
            .ok_or(MatrixClientError::ContentTypeMissingError)?;

        if !content_type.eq(&HeaderValue::from_static("application/json")) {
            return Err(MatrixClientError::ContentTypeInvalidError(content_type.to_str()
                .unwrap_or("Failed up unwrap Content-Type value")
                .to_string()))
        }
        
        let bytes = response.body()
            .await
            .map_err(|e| MatrixClientError::PayloadErr(e))?;

        println!("{}", String::from_utf8_lossy(bytes.as_ref()));

        serde_json::from_slice(&*bytes)
            .map_err(|e|MatrixClientError::JsonDeserializationError(e))
    }
}