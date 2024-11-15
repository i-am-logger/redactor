use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::huggingface_hub::types::{DownloadModelError, ModelConfigError};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NERModel {
    pub model_name: String,
    pub entity_types: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LLMModel {
    pub model_name: String,
    pub prompt: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RedactOptions {
    #[serde(rename = "ner")]
    NER(NERModel),
    #[serde(rename = "llm")]
    LLM(LLMModel),
}

#[derive(Error, Debug)]
pub enum UpdateRedactError {
    #[error("LLM models are not supported")]
    NotSupportingLLMModel,

    #[error("HuggingFace api error: {0}")]
    HuggingFace(#[from] hf_hub::api::tokio::ApiError),

    #[error("Invalid model configuration: {0}")]
    ModelConfigError(#[from] ModelConfigError),

    #[error("Failed to download model: {0}")]
    DownloadModelError(#[from] DownloadModelError),
}

use std::error::Error as StdError;

#[derive(Error, Debug)]
pub enum RedactError {
    #[error("LLM models are not supported")]
    NotSupportingLLMModel,
    #[error("Options not initialized")]
    OptionsNotInitialized,
    #[error("API configuration not initialized")]
    ApiNotInitialized,
    #[error("HuggingFace api error: {0}")]
    HuggingFace(#[from] hf_hub::api::tokio::ApiError),
    #[error("JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // #[error("Candle error: {0}")]
    // Candle(#[from] candle_core::Error),
    #[error("Tokenizer error: {0}")]
    Tokenizer(#[from] Box<dyn StdError + Send + Sync>),
}
