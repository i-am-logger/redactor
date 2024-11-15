// use candle_core::{safetensors, DType, Device, Tensor};
// use candle_nn::VarBuilder;
// use candle_transformers::models::bert::{BertModel, Config};
use futures::{
    stream::{self, Stream},
    StreamExt,
};
use hf_hub::api::tokio::{Api, ApiBuilder};
use log::info;
use std::{env, path::PathBuf, time::Duration};
use tokio::time::sleep;

use super::{
    huggingface_hub::utils,
    types::{RedactError, RedactOptions, UpdateRedactError},
};

fn get_cache_dir() -> PathBuf {
    match env::var("CACHE_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            eprintln!("Warning: CACHE_DIR not set, using default cache directory");
            PathBuf::from(format!("~/.cache/huggingface",))
        }
    }
}

pub async fn update_redact_settings(options: &RedactOptions) -> Result<Api, UpdateRedactError> {
    match options {
        RedactOptions::LLM(_) => return Err(UpdateRedactError::NotSupportingLLMModel),
        RedactOptions::NER(ner) => {
            let token = std::env::var("HUGGINGFACE_HUB_TOKEN").ok();
            let api = ApiBuilder::new()
                .with_cache_dir(get_cache_dir())
                .with_token(token)
                .build()?;

            let model = api.model(ner.model_name.to_string());
            let config = utils::download_model_config(&model).await?;
            config.validate_entity_types(&ner.entity_types)?;

            utils::download_model(&model).await?;
            info!("Model '{}' download completed successfully", ner.model_name);
            Ok(api)
        }
    }
}

// fn is_entity_of_interest(pred: u32, entity_types: &[String]) -> bool {
//     let entity_type = match pred {
//         1 => Some("PER"), // B-PER (Beginning of Person)
//         2 => Some("PER"), // I-PER (Inside of Person)
//         3 => Some("ORG"), // B-ORG (Beginning of Organization)
//         4 => Some("ORG"), // I-ORG (Inside of Organization)
//         5 => Some("LOC"), // B-LOC (Beginning of Location)
//         6 => Some("LOC"), // I-LOC (Inside of Location)
//         _ => None,        // O (Outside/Not an entity)
//     };

//     entity_type.map_or(false, |et| entity_types.iter().any(|t| t == et))
// }

pub async fn redact(
    api: &Api,
    options: &RedactOptions,
    text: String,
) -> Result<impl Stream<Item = String>, RedactError> {
    match options {
        RedactOptions::LLM(_) => return Err(RedactError::NotSupportingLLMModel),
        RedactOptions::NER(_ner) => {
            // let device = Device::cuda_if_available(0)?;
            // let model = api.model(ner.model_name.to_string());
            // let config_file = model.get("config.json").await?;
            // let vocab_file = model.get("vocab.txt").await?;
            // let model_file = model.get("model.safetensors").await?;

            // // let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
            // let tokenizer = tokenizers::Tokenizer::from_file(vocab_file)?;
            // let config: Config = serde_json::from_slice(&std::fs::read(config_file)?)?;

            // let tensors = safetensors::load(&model_file, &device)?;
            // let vb = VarBuilder::from_tensors(tensors, DType::F32, &device);
            // let bert_model = BertModel::load(vb, &config)?;

            // let encoding = tokenizer.encode(text.as_str(), true)?;
            // let input_ids = Tensor::new(encoding.get_ids(), &device)?;
            // let attention_mask = Tensor::new(encoding.get_attention_mask(), &device)?;

            // let hidden_states = bert_model.forward(&input_ids, &attention_mask, None)?;

            // let predictions = hidden_states.argmax(2)?;
            // let pred_ids = predictions.to_vec1::<u32>()?;

            // let mut result = text.clone();
            // for (idx, &pred) in pred_ids.iter().enumerate() {
            //     if is_entity_of_interest(pred, &ner.entity_types) {
            //         let token = encoding.get_tokens()[idx].clone();
            //         result = result.replace(token.as_str(), &"█".repeat(token.len()));
            //     }
            // }

            // Ok(stream::once(async move { result }).boxed())

            // just to test streaming
            Ok(stream::iter(vec![text; 20])
                .then(|t| async move {
                    sleep(Duration::from_millis(250)).await;
                    t
                })
                .boxed())
        }
    }
}
