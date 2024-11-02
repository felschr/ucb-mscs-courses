use std::{ops::Deref, sync::Arc};

use tokio::sync::RwLock;
use tonic::server::NamedService;
use tonic_health::{
    pb::health_server::{Health, HealthServer},
    ServingStatus,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Serving,
    NotServing,
    Unknown,
}

impl From<bool> for HealthStatus {
    fn from(value: bool) -> Self {
        match value {
            false => HealthStatus::NotServing,
            true => HealthStatus::Serving,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HealthReporterInner {
    status: HealthStatus,
    grpc_health_reporter: tonic_health::server::HealthReporter,
}

impl HealthReporterInner {
    pub fn set_status(&mut self, status: HealthStatus) {
        self.status = status;
    }
}

#[derive(Clone, Debug)]
pub struct HealthReporter(Arc<RwLock<HealthReporterInner>>);

impl HealthReporter {
    pub fn new() -> (HealthReporter, HealthServer<impl Health>) {
        let (grpc_health_reporter, grpc_health_server) = tonic_health::server::health_reporter();
        let reporter = HealthReporter(Arc::new(RwLock::new(HealthReporterInner {
            status: HealthStatus::Unknown,
            grpc_health_reporter,
        })));
        (reporter.clone(), grpc_health_server)
    }

    pub async fn get_status(&self) -> HealthStatus {
        self.read().await.status
    }

    /// Set main serving status for HTTP & gRPC health service.
    /// ATTENTION: status for specific gRPC services need to be set via `set_grpc_service_serving`
    pub async fn set_serving(&mut self, serving: bool) {
        let mut inner = self.0.write().await;
        inner.set_status(serving.into());

        inner
            .grpc_health_reporter
            .set_service_status(
                "",
                match serving {
                    true => ServingStatus::Serving,
                    false => ServingStatus::NotServing,
                },
            )
            .await;
    }

    /// Sets gRPC serving status for specific service
    pub async fn set_grpc_service_serving<T: NamedService>(&mut self, serving: bool) {
        let mut inner = self.0.write().await;
        if serving {
            inner.grpc_health_reporter.set_serving::<T>().await;
        } else {
            inner.grpc_health_reporter.set_not_serving::<T>().await;
        };
    }
}

impl Deref for HealthReporter {
    type Target = Arc<RwLock<HealthReporterInner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_reporter_status() {
        let (mut reporter, _) = HealthReporter::new();
        assert_eq!(HealthStatus::Unknown, reporter.get_status().await);

        reporter.set_serving(true).await;
        assert_eq!(HealthStatus::Serving, reporter.get_status().await);

        reporter.set_serving(false).await;
        assert_eq!(HealthStatus::NotServing, reporter.get_status().await);
    }
}
