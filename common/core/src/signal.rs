/// Returns when a SIGTERM, SIGINT or Ctrl+C signal is
/// received or shuts down immediately in case of SIGQUIT.
#[cfg(unix)]
pub async fn shutdown() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            tracing::info!("SIGINT received, shutting down gracefully");
        }
        _ = sigterm.recv() => {
            tracing::info!("SIGTERM received, shutting down gracefully");
        }
        _ = sigquit.recv() => {
            tracing::info!("SIGQUIT received, shutting down immediately");
            std::process::exit(0);
        }
    }
}

#[cfg(not(unix))]
pub async fn shutdown() {
    use tokio::signal::windows;

    tokio::signal::ctrl_c().await;
    tracing::info!("CTRL-C received, shutting down gracefully");
}
