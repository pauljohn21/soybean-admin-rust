use server_global::global;

use crate::project_info;

pub async fn initialize_event_channel() {
    global::initialize_event_channels().await;
    project_info!("Event channels initialized");

    tokio::spawn(async {
        server_service::admin::handle_login_jwt().await;
    });
    project_info!("Login JWT handler spawned");

    if let Some(rx) = global::get_string_event_receiver().await {
        tokio::spawn(async move {
            server_service::admin::start_event_listener(rx).await;
        });
        project_info!("String event listener spawned");
    } else {
        project_info!("No string event receiver available");
    }
}
