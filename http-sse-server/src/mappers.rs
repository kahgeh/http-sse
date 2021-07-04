use crate::settings::AppSettings;
use crate::logging::LogSettings;
use crate::application::{HttpServerSettings, ApplicationStartUpDisplayInfo};

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
