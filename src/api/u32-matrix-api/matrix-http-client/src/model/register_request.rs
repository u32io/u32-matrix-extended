use serde::{Serialize};
use crate::model::Flow;

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub auth: Flow,
}