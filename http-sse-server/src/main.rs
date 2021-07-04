mod settings;
mod app_ops;
mod logging;
mod application;
mod contracts;
mod mappers;
mod routes;

use tracing::{info};
use crate::settings::{AppSettings};
use crate::logging::{LoggingBuilder};
use crate::application::{Application, ApplicationStartUpDisplayInfo};

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let app_settings = AppSettings::load();

    LoggingBuilder::new((&app_settings).into())
        .init_default();

    let server = Application::new((&app_settings).into())
        .start(app_settings.clone())?;

    let ApplicationStartUpDisplayInfo{ environment_name, is_debug} = (&app_settings).into();
    info!(Environment=&environment_name[..], IsDebug=&is_debug[..], "Application started");
    server.await
}