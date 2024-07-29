use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

pub async fn initialize_log_tracing() {
    LogTracer::init()
        .expect("[soybean-admin-rust] >>>>>> [server-initialize] Failed to set logger");

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false).with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sea_orm=info")),
    );

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).expect(
        "[soybean-admin-rust] >>>>>> [server-initialize] Failed to set up global subscriber",
    );

    tracing::info!(
        "[soybean-admin-rust] >>>>>> [server-initialize] Log Tracing initialized successfully"
    );
}
