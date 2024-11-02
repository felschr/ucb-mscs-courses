use std::time::Duration;

use futures::Future;
use tonic::async_trait;

pub use crate::health::common::HealthReporter;

pub mod common;
pub mod grpc;

#[async_trait]
pub trait HealthCheck {
    async fn is_alive(&self) -> bool;
}

async fn check_health<Fut>(health: &impl HealthCheck, on_result: &mut impl FnMut(bool) -> Fut)
where
    Fut: Future<Output = ()>,
{
    let serving = health.is_alive().await;
    (*on_result)(serving).await;
}

/// Checks the health using object implementing `HealthCheck` trait
/// every second and updates gRPC health services accordingly.
pub async fn check_health_loop<Fut>(
    health: &impl HealthCheck,
    mut on_result: impl FnMut(bool) -> Fut,
) where
    Fut: Future<Output = ()>,
{
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        check_health(health, &mut on_result).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_health() {
        struct State(bool);

        #[async_trait]
        impl HealthCheck for State {
            async fn is_alive(&self) -> bool {
                self.0
            }
        }

        let mut state = State(false);
        check_health(&state, &mut |serving| async move { assert!(!serving) }).await;

        state.0 = true;
        check_health(&state, &mut |serving| async move { assert!(serving) }).await;
    }
}
