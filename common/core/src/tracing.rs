use sentry::integrations::tracing::EventFilter;
use tower_http::classify::GrpcErrorsAsFailures;
use tower_http::classify::ServerErrorsAsFailures;
use tower_http::classify::SharedClassifier;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::Metadata;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};

/// Initializes the `tracing` logging framework.
///
/// Regular CLI output is influenced by the
/// [`RUST_LOG`](tracing_subscriber::filter::EnvFilter) environment variable.
///
/// This function also sets up the Sentry error reporting integration for the
/// `tracing` framework.
pub fn init() {
    let log_layer = tracing_subscriber::fmt::layer()
        .compact()
        .without_time()
        .with_filter(EnvFilter::from_default_env());

    let sentry_layer = sentry::integrations::tracing::layer()
        .event_filter(event_filter)
        .with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(log_layer)
        .with(sentry_layer)
        .init();
}

pub fn event_filter(metadata: &Metadata<'_>) -> EventFilter {
    match metadata.level() {
        &Level::ERROR => EventFilter::Exception,
        &Level::WARN | &Level::INFO => EventFilter::Breadcrumb,
        &Level::DEBUG | &Level::TRACE => EventFilter::Ignore,
    }
}

/// Initializes the `tracing` logging framework for usage in tests.
pub fn init_for_test() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let _ = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .without_time()
        .with_test_writer()
        .try_init();
}

/// [`TraceLayer`] with common configuration for gRPC servers.
pub fn trace_layer_grpc() -> TraceLayer<SharedClassifier<GrpcErrorsAsFailures>> {
    TraceLayer::new_for_grpc().make_span_with(DefaultMakeSpan::new().include_headers(true))
}

/// [`TraceLayer`] with common configuration for HTTP servers.
pub fn trace_layer_http() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true))
}
