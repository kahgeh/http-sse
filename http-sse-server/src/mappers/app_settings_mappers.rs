use std::time::SystemTime;
use std::borrow::Borrow;
use actix_web::web::Data;

use crate::settings::AppSettings;
use crate::logging::LogSettings;
use crate::application::{HttpServerSettings, ApplicationStartUpDisplayInfo};
use crate::contracts::GetAppInfoResponse;
use crate::app_ops::{RuntimeInfo, systemtime_strftime, DATE_ISO_FORMAT};
use crate::peers::{K8sDiscovery, DevBoxDiscovery};

impl From<&AppSettings> for LogSettings {
    fn from(app_state: &AppSettings) -> Self {
        LogSettings::new(
            app_state.app_name.as_str(),
            app_state.settings.log_level.as_str()
        )
    }
}

impl From<&AppSettings> for HttpServerSettings {
    fn from(app_state: &AppSettings) -> Self {
        HttpServerSettings::new(
            app_state.settings.url_prefix.as_str(),
            app_state.settings.port,
        )
    }
}

impl From<&AppSettings> for ApplicationStartUpDisplayInfo {
    fn from(app_state: &AppSettings) -> Self {
        ApplicationStartUpDisplayInfo::new(
            app_state.settings.environment.as_str(),
            app_state.settings.debug,
        )
    }
}

impl From<Data<AppSettings>> for GetAppInfoResponse {
    fn from(app_settings: Data<AppSettings>) -> Self {
        let app_name = app_settings.app_name.clone();
        let RuntimeInfo {git_commit_id, started, ip:_}  = app_settings.runtime_info.borrow();

        GetAppInfoResponse{
            app_name,
            git_commit_id: String::from(git_commit_id),
            started: String::from(started),
            current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
        }
    }
}

impl From<&AppSettings> for K8sDiscovery {
    fn from(app_settings: &AppSettings) -> Self {
        let namespace = app_settings.settings.k8s_namespace.clone();
        let service_name = app_settings.app_name.clone();
        K8sDiscovery {
            namespace,
            service_name,
        }
    }
}

impl From<&AppSettings> for DevBoxDiscovery {
    fn from(app_settings: &AppSettings) -> Self {
        let port = app_settings.settings.port;
        let ip = app_settings.runtime_info.ip.clone();
        DevBoxDiscovery {
            ip,
            port,
        }
    }
}