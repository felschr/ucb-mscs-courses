use futures::FutureExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tonic::async_trait;
use tonic_web::GrpcWebLayer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use ucb_mscs_courses_core::debug::if_debug;
use ucb_mscs_courses_core::health::HealthReporter;
use ucb_mscs_courses_core::tracing::trace_layer_grpc;

use proto::courses_grpc_server::CoursesGrpcServer;
use ucb_mscs_courses_proto::course::v1 as proto;

use crate::api::courses::MyCoursesGrpc;

mod api;
mod config;

struct AppState {
    // repository: Repository,
}

#[async_trait]
impl ucb_mscs_courses_core::health::HealthCheck for AppState {
    async fn is_alive(&self) -> bool {
        select! {
            _ = tokio::time::sleep(Duration::from_secs(1)) => false,
            // v = self.repository.is_alive() => v,
        }
    }
}

/// set health status
pub async fn set_serving(reporter: &mut HealthReporter, serving: bool) {
    let mut rep = reporter.clone();
    let set_grpc_services_serving = || async move {
        rep.set_grpc_service_serving::<CoursesGrpcServer<MyCoursesGrpc>>(serving)
            .await;
    };

    tokio::join!(reporter.set_serving(serving), set_grpc_services_serving());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::init();

    let _sentry = ucb_mscs_courses_core::sentry::init(config.sentry);
    ucb_mscs_courses_core::tracing::init();
    let sentry_layer = ucb_mscs_courses_core::sentry::layer();

    // let repository = Repository::new(&config.db).await?;

    let state = Arc::new(AppState {});
    let state2 = state.clone();

    // health service
    let (health_reporter, grpc_health_server) = HealthReporter::new();
    tokio::spawn(async move {
        ucb_mscs_courses_core::health::check_health_loop(state2.as_ref(), |serving| {
            let mut reporter = health_reporter.clone();
            async move {
                set_serving(&mut reporter, serving).await;
            }
        })
        .await;
    });

    let shutdown_signal = ucb_mscs_courses_core::signal::shutdown().shared();

    let tonic_handle = tokio::spawn(async move {
        let reflection_service = if_debug(|| {
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
                .register_encoded_file_descriptor_set(
                    ucb_mscs_courses_proto::course::v1::FILE_DESCRIPTOR_SET,
                )
                .build_v1()
                .unwrap()
        });

        let courses = MyCoursesGrpc {
            // repository: state.repository.clone(),
        };

        let default_builder = ServiceBuilder::new();

        let svc_courses = default_builder.service(CoursesGrpcServer::new(courses));

        let addr = "0.0.0.0:8000".parse().unwrap();
        tracing::info!("gRPC server listening on {addr}");

        tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(sentry_layer)
            .layer(trace_layer_grpc())
            .layer(CorsLayer::new())
            .layer(GrpcWebLayer::new())
            .add_service(grpc_health_server)
            .add_service(svc_courses)
            .add_optional_service(reflection_service)
            .serve_with_shutdown(addr, shutdown_signal)
            .await
    });

    tonic_handle.await??;

    tracing::info!("Service has shut down gracefully.");
    Ok(())
}
