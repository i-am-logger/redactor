use std::env::var;

use log::warn;

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "9000";
// const DEFAULT_CACHE_DIR: &str = "./.cache/";

pub fn get_bind_address() -> String {
    format!(
        "{}:{}",
        var("BIND_ADDRESS").unwrap_or_else(|_| {
            warn!("BIND_ADDRESS not set, using default: {}", DEFAULT_ADDRESS);
            DEFAULT_ADDRESS.to_string()
        }),
        var("BIND_PORT").unwrap_or_else(|_| {
            warn!("BIND_PORT not set, using default: {}", DEFAULT_PORT);
            DEFAULT_PORT.to_string()
        })
    )
}
