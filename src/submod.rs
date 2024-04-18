#[tracing::instrument]
pub fn function() {
    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");
}
