pub use jsonwebtoken::Validation;

pub mod global;

#[macro_export]
macro_rules! project_info {
    ($($arg:tt)+) => {{
        let span = tracing::span!(
            tracing::Level::INFO,
            module_path!(),
            file = file!(),
            line = line!(),
        );
        let _enter = span.enter();
        tracing::info!(
            target: "[soybean-admin-rust]",
            $($arg)+
        );
    }}
}

#[macro_export]
macro_rules! project_error {
    ($($arg:tt)+) => {{
        let span = tracing::span!(
            tracing::Level::ERROR,
            module_path!(),
            file = file!(),
            line = line!(),
        );
        let _enter = span.enter();
        tracing::error!(
            target: "[soybean-admin-rust]",
            $($arg)+
        );
    }}
}
