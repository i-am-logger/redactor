use actix_web::{
    get, post, put,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};

use crate::{
    core::{
        ai,
        types::{RedactError, RedactOptions, UpdateRedactError},
    },
    utils::web::to_bytes_stream,
    web::types::AppState,
};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_options);
    cfg.service(set_options);
    cfg.service(redact_text);
}

#[get("/api/options")]
async fn get_options(data: Data<AppState>) -> impl Responder {
    let options_guard = data.options.read().await;
    match options_guard.as_ref() {
        None => HttpResponse::NotFound().body("Options not initialized."),
        Some(options) => HttpResponse::Ok().json(options),
    }
}

#[put("/api/options")]
async fn set_options(
    data: Data<AppState>,
    json_options: Json<RedactOptions>,
) -> Result<HttpResponse, UpdateRedactError> {
    let options = json_options.into_inner();
    let mut options_guard = data.options.write().await;
    let mut api_guard = data.api.write().await;
    let api = ai::update_redact_settings(&options).await?;
    *options_guard = Some(options);
    *api_guard = Some(api);
    Ok(HttpResponse::Ok().body("Options updated"))
}

#[post("/api/redact_text")]
async fn redact_text(data: Data<AppState>, text: String) -> Result<HttpResponse, RedactError> {
    let options_guard = data.options.read().await;
    let options = options_guard
        .as_ref()
        .ok_or(RedactError::OptionsNotInitialized)?;

    let api_guard = data.api.read().await;
    let api = api_guard.as_ref().ok_or(RedactError::ApiNotInitialized)?;

    let stream = ai::redact(api, options, text).await?;
    let bytes_stream = to_bytes_stream(stream);

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(bytes_stream))
}
