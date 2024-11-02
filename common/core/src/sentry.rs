use sentry::integrations::tower as sentry_tower;
use sentry::integrations::tower::NewFromTopProvider;
use sentry::{ClientInitGuard, ClientOptions, Hub, SessionMode, TransactionContext};
use std::sync::Arc;
use tower::layer::util::Stack;

use crate::config;

type Config = config::Sentry;

/// Initializes the Sentry SDK from the environment variables.
pub fn init(config: Config) -> ClientInitGuard {
    println!("sentry config: {:?}", config);

    let Config {
        enable,
        debug,
        dsn,
        release,
        environment,
    } = config;

    let traces_sample_rate = 0.1;

    let traces_sampler = move |ctx: &TransactionContext| -> f32 {
        match ctx.name() {
            name if name.contains(" /health") => 0.0,
            name if name.contains(" /grpc.health.v1.Health") => 0.0,
            _ => traces_sample_rate,
        }
    };

    sentry::init(ClientOptions {
        debug,
        dsn: enable.then(|| dsn.expect("Sentry is enabled but no DSN is configured")),
        environment: Some(environment.into()),
        release: Some(release.into()),
        auto_session_tracking: true,
        session_mode: SessionMode::Request,
        traces_sampler: Some(Arc::new(traces_sampler)),
        ..Default::default()
    })
}

/// Create a new stack layer with configured [`SentryLayer`] & [`SentryHttpLayer`] layers.
///
/// [`SentryLayer`]: sentry_tower::SentryLayer
/// [`SentryHttpLayer`]: sentry_tower::SentryHttpLayer
pub fn layer<Request>() -> Stack<
    sentry_tower::SentryLayer<NewFromTopProvider, Arc<Hub>, Request>,
    sentry_tower::SentryHttpLayer,
> {
    tower::layer::util::Stack::new(
        sentry_tower::NewSentryLayer::new_from_top(),
        sentry_tower::SentryHttpLayer::with_transaction(),
    )
}
