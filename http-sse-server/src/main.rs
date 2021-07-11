mod settings;
mod app_ops;
mod logging;
mod application;
mod contracts;
mod mappers;
mod routes;
mod sse_exchange;
mod peers;

use tokio::{join};
use tracing::{info};
use crate::settings::{AppSettings};
use crate::logging::{LoggingBuilder};
use crate::application::{Application, ApplicationStartUpDisplayInfo};

#[actix_web::main]
async fn main()-> tokio::io::Result<()> {
    let app_settings = AppSettings::load();

    LoggingBuilder::new((&app_settings).into())
        .init_default();

    let (server, sse_exchange_task) = Application::new((&app_settings).into())
        .start(app_settings.clone()).unwrap();

    let ApplicationStartUpDisplayInfo{ environment_name, is_debug} = (&app_settings).into();
    info!(Environment=&environment_name[..], IsDebug=&is_debug[..], "Application started");

    let (server_result, _)=join!(server, sse_exchange_task);

    if server_result.is_err() {
        return Err(server_result.unwrap_err())
    }

    Ok(())


}