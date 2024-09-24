use std::error::Error;

use server_config::ServerConfig;
use server_global::global;

pub async fn get_server_address() -> Result<String, Box<dyn Error>> {
    let server_config = global::get_config::<ServerConfig>().await.unwrap();
    let addr = format!("{}:{}", server_config.host, server_config.port);
    Ok(addr)
}
