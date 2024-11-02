use tonic::server::NamedService;
use tonic_health::server::HealthReporter;

/// Combined version of [`HealthReporter::set_serving`] &
/// [`HealthReporter::set_not_serving`]
///
/// [`HealthReporter::set_serving`]: HealthReporter::set_serving
/// [`HealthReporter::set_not_serving`]: HealthReporter::set_not_serving
pub async fn set_serving<T: NamedService>(reporter: &mut HealthReporter, serving: bool) {
    if serving {
        reporter.set_serving::<T>().await;
    } else {
        reporter.set_not_serving::<T>().await;
    };
}
