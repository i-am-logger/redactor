use std::path::PathBuf;

use hf_hub::api::tokio::ApiRepo;
use log::{info, warn};

use super::types::{DownloadModelError, ModelConfig};

pub async fn download_model_config(model: &ApiRepo) -> Result<ModelConfig, DownloadModelError> {
    let path: PathBuf = model.get("config.json").await?;

    info!("Downloaded config to: {:?}", path);

    let config_content = std::fs::read_to_string(&path)?;
    let config: ModelConfig = serde_json::from_str(&config_content)?;

    Ok(config)
}

pub async fn download_model(model: &ApiRepo) -> Result<(), DownloadModelError> {
    let tokenizer_files = [
        "model.safetensors",
        "vocab.txt",
        "tokenizer_config.json",
        "special_tokens_map.json",
    ];

    for file in tokenizer_files.iter() {
        match model.get(file).await {
            Ok(path) => info!("Downloaded '{}' to: {:?}", file, path),
            Err(e) => warn!("Failed to download optional file '{}': {}", file, e),
        }
    }

    let model_path = model.get("model.safetensors").await?;
    info!("Downloaded 'model.safetensors' to: {:?}", model_path);
    Ok(())
}
