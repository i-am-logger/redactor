use common::config;
use env_logger;
use log::info;

mod common;
mod core;
mod utils;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();
    info!("Starting application...");
    let config = config::get_bind_address();
    let result = web::server::start(config).await;
    match &result {
        Ok(_) => info!("Goodbye..."),
        Err(e) => info!("Application exited with error: {}", e),
    }
    result
}
