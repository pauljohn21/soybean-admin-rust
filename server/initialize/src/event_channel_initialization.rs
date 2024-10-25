use server_global::global;

pub async fn initialize_event_channel() {
    global::initialize_event_channels().await;

    tokio::spawn(async {
        server_service::admin::handle_login_jwt().await;
    });

    if let Some(rx) = global::get_string_event_receiver().await {
        tokio::spawn(async move {
            server_service::admin::start_event_listener(rx).await;
        });
    }
}
