use std::sync::Arc;

use actix_web::{http::StatusCode, ResponseError};
use hf_hub::api::tokio::Api;
use tokio::sync::RwLock;

use crate::core::{
    huggingface_hub::types::ModelConfigError,
    types::{RedactError, RedactOptions, UpdateRedactError},
};

pub struct AppState {
    pub options: Arc<RwLock<Option<RedactOptions>>>,
    pub api: Arc<RwLock<Option<Api>>>,
}

impl ResponseError for ModelConfigError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl ResponseError for UpdateRedactError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl ResponseError for RedactError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
