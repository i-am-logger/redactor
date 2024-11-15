#[cfg(test)]
mod tests {
    use crate::core::{
        ai,
        types::{NERModel, RedactError, RedactOptions},
    };
    use futures::StreamExt;
    use hf_hub::api::tokio::Api;
    use tokio::sync::OnceCell;

    const DEFAULT_NER_MODEL: &str = "dslim/bert-base-NER";

    static API: OnceCell<Api> = OnceCell::const_new();

    async fn get_api() -> &'static Api {
        API.get_or_init(|| async {
            let options = RedactOptions::NER(NERModel {
                model_name: DEFAULT_NER_MODEL.to_string(),
                entity_types: vec!["PER".to_string()],
            });
            ai::update_redact_settings(&options)
                .await
                .expect("Failed to initialize API")
        })
        .await
    }

    async fn run_redaction(text: &str, options: &RedactOptions) -> Result<String, RedactError> {
        let api = get_api().await;
        let mut result_stream = ai::redact(api, options, text.to_string()).await?;
        let mut redacted_text = String::new();

        while let Some(chunk) = result_stream.next().await {
            redacted_text.push_str(&chunk);
        }
        Ok(redacted_text)
    }

    fn create_ner_options(entity_types: Vec<&str>) -> RedactOptions {
        RedactOptions::NER(NERModel {
            model_name: DEFAULT_NER_MODEL.to_string(),
            entity_types: entity_types.into_iter().map(String::from).collect(),
        })
    }

    #[tokio::test]
    async fn test_redact_person() -> Result<(), RedactError> {
        let text = "John Doe met Jane Smith.";
        let options = create_ner_options(vec!["PER"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "████████ met ██████████.");

        Ok(())
    }

    #[tokio::test]
    async fn test_redact_location() -> Result<(), RedactError> {
        dotenv::dotenv().ok();

        let text = "John Doe went to Paris.";
        let options = create_ner_options(vec!["LOC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "John Doe went to █████.");

        Ok(())
    }

    #[tokio::test]
    async fn test_redact_date() -> Result<(), RedactError> {
        let text = "The meeting is on January 1, 2025.";
        let options = create_ner_options(vec!["MISC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "The meeting is on ████████ 1, 2025.");
        Ok(())
    }

    #[tokio::test]
    async fn test_redact_money() -> Result<(), RedactError> {
        let text = "The price is $100.";
        let options = create_ner_options(vec!["MISC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "The price is ███.");
        Ok(())
    }

    #[tokio::test]
    async fn test_redact_specific_words() -> Result<(), RedactError> {
        let text = "The confidential code is 12345.";
        let options = create_ner_options(vec!["MISC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "The ███████████ code is █████.");
        Ok(())
    }

    #[tokio::test]
    async fn test_redact_multiple_entity_types() -> Result<(), RedactError> {
        let text = "John Doe spent $100 in Paris on January 1st.";
        let options = create_ner_options(vec!["PER", "MISC", "LOC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "████████ spent ███ in █████ on ███████████.");
        Ok(())
    }

    #[tokio::test]
    async fn test_empty_text() -> Result<(), RedactError> {
        let text = "";
        let options = create_ner_options(vec!["PER"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(redacted_text, "");
        Ok(())
    }

    #[tokio::test]
    async fn test_text_without_entities() -> Result<(), RedactError> {
        let text = "The quick brown fox jumps over the lazy dog.";
        let options = create_ner_options(vec!["PER", "LOC"]);
        let redacted_text = run_redaction(text, &options).await?;
        assert_eq!(
            redacted_text,
            "The quick brown fox jumps over the lazy dog."
        );
        Ok(())
    }
}
