#[cfg(any(feature = "google-cloud-secretmanager-logging-v1"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-secretmanager-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-secretmanager-v1beta2"))]
pub mod v1beta2;
