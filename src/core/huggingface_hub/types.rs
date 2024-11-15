use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadModelError {
    #[error("Failed to download file: {0}")]
    ApiError(#[from] hf_hub::api::tokio::ApiError),

    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ConfigParseError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum ModelConfigError {
    #[error("Model is not a token classifier")]
    NotTokenClassifier,

    #[error("Model has no label mappings")]
    NoLabelMappings,

    #[error("NoNER labels found in model")]
    NoNerLabels,

    #[error("Unsupported entity types: {0:?}")]
    UnsupportedEntityTypes(Vec<String>),
}

// #[derive(Deserialize, Debug)]
// struct TaskParams {
//     task: String,
// }
#[derive(Deserialize, Debug)]
pub struct ModelConfig {
    architectures: Vec<String>,
    #[serde(default)]
    id2label: HashMap<String, String>,
    // #[serde(rename = "task_specific_params")]
    // #[serde(default)]
    // task_params: HashMap<String, TaskParams>,
}

impl ModelConfig {
    fn is_token_classifier(&self) -> bool {
        self.architectures
            .iter()
            .any(|a| a.contains("ForTokenClassification"))
    }
    fn get_ner_labels(&self) -> Result<Vec<String>, ModelConfigError> {
        if !self.is_token_classifier() {
            return Err(ModelConfigError::NotTokenClassifier);
        }
        if self.id2label.is_empty() {
            return Err(ModelConfigError::NoLabelMappings);
        }

        let entity_types: Vec<String> = self
            .id2label
            .values()
            .filter(|label| label.starts_with("B-"))
            .map(|label| label.strip_prefix("B-").unwrap().to_string())
            .collect();

        if entity_types.is_empty() {
            return Err(ModelConfigError::NoNerLabels);
        }

        Ok(entity_types)
    }

    pub fn validate_entity_types(
        &self,
        requested_types: &[String],
    ) -> Result<(), ModelConfigError> {
        let supported_types = self.get_ner_labels()?;

        let unsupported: Vec<String> = requested_types
            .iter()
            .filter(|req_type| !supported_types.contains(*req_type))
            .cloned()
            .collect();

        if unsupported.is_empty() == false {
            return Err(ModelConfigError::UnsupportedEntityTypes(unsupported));
        }

        Ok(())
    }
}
