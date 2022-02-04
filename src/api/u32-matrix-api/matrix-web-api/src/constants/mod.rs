pub struct MatrixWebApi;

impl MatrixWebApi {
    pub const APP_NAME: &'static str = "u32-matrix-api";
    pub const APP_VERSION: &'static str = "0.0.1";
    pub const APP_AUTHOR: &'static str = "James M. <jamesjmeyer210@gmail.com>";
    pub const DEFAULT_ADDRESS: &'static str = "https://localhost:7676";

    pub const LOGGING_FILE_NAME: &'static str = "logging.yaml";
    pub const LOGGING_PROD_FILE_NAME: &'static str = "logging.prod.yaml";

    pub const SECRET_FILE_NAME: &'static str = "secret.dat";
}
