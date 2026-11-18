
use rinf::{DartSignal, RustSignal};

use crate::models::InstalledDownloaderConfig;

#[derive(serde::Serialize, serde::Deserialize, DartSignal)]
pub(crate) struct InstallDownloaderConfigFromUrlRequest {
    pub url: String,

    #[serde(default)]
    pub rclone_config_password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, DartSignal)]
pub(crate) struct RetryDownloaderInitRequest {
    #[serde(default)]
    pub rclone_config_password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, DartSignal)]
pub(crate) struct RefreshDownloaderSourcesRequest {}

#[derive(serde::Serialize, serde::Deserialize, DartSignal)]
pub(crate) struct SelectDownloaderSourceRequest {
    pub config_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, DartSignal)]
pub(crate) struct RemoveDownloaderSourceRequest {
    pub config_id: String,
}

#[derive(Default, serde::Serialize, serde::Deserialize, RustSignal)]
pub(crate) struct DownloaderSourcesChanged {
    pub configs: Vec<InstalledDownloaderConfig>,
    pub active_config_id: Option<String>,
    pub refreshing: bool,
    pub error: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, RustSignal)]
pub(crate) struct DownloaderConfigInstallResult {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, RustSignal)]
pub(crate) struct DownloaderSourceRemovedResult {
    pub config_id: String,
    pub success: bool,
    pub error: Option<String>,
}
