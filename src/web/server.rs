use std::sync::Arc;

use crate::web::{api::routes::init, types::AppState};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::info;
use tokio::sync::RwLock;

pub async fn start(bind_address: String) -> std::io::Result<()> {
    let data = Data::new(AppState {
        options: Arc::new(RwLock::new(None)),
        api: Arc::new(RwLock::new(None)),
    });

    let data = data.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(init)
    })
    .bind(&bind_address)?;
    let addrs = server.addrs();
    info!("Server started on: {}", addrs[0]);
    server.run().await
}
