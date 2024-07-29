use server_global::global;

pub async fn initialize_event_channel() {
    global::initialize_dyn_global_event_channel().await;

    tokio::spawn(async {
        server_service::admin::handle_login_jwt().await;
    });
}
