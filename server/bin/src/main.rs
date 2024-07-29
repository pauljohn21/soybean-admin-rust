use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    server_initialize::initialize_config("server/resources/application.yaml").await;
    server_initialize::initialize_log_tracing().await;
    server_initialize::init_primary_connection().await;
    server_initialize::initialize_keys_and_validation().await;
    server_initialize::initialize_event_channel().await;

    // build our application with a route
    let app = server_initialize::initialize_admin_router().await;

    // run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
