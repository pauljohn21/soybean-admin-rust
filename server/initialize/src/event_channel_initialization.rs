use server_global::global;

pub async fn initialize_event_channel() {
    use server_service::admin::{
        auth_login_listener, jwt_created_listener, sys_operation_log_listener,
    };

    global::register_event_listeners(
        Box::new(|rx| Box::pin(jwt_created_listener(rx))),
        &[
            (
                "auth_login".to_string(),
                Box::new(|rx| Box::pin(auth_login_listener(rx))),
            ),
            (
                "sys_operation_log".to_string(),
                Box::new(|rx| Box::pin(sys_operation_log_listener(rx))),
            ),
        ],
    )
    .await;
}
