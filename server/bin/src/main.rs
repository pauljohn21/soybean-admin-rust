use std::net::SocketAddr;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config_path = if cfg!(debug_assertions) {
        "server/resources/application-test.yaml"
    } else {
        "server/resources/application.yaml"
    };

    server_initialize::initialize_log_tracing().await;
    server_initialize::initialize_config(config_path).await;
    server_initialize::init_primary_connection().await;
    server_initialize::initialize_keys_and_validation().await;
    server_initialize::initialize_event_channel().await;

    // build our application with a route
    let app = server_initialize::initialize_admin_router().await;

    let addr = match server_initialize::get_server_address().await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to get server address: {}", e);
            return;
        }
    };

    // run it
    let listener = TcpListener::bind(&addr).await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
