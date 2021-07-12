mod settings;
mod app_ops;
mod logging;
mod application;
mod contracts;
mod mappers;
mod routes;
mod sse_exchange;
mod peers;

use futures::future::{join_all};
use tokio::{select, signal::{ctrl_c}};
use tracing::{error, info, debug};
use crate::settings::{AppSettings};
use crate::logging::{LoggingBuilder};
use crate::application::{Application, ApplicationStartUpDisplayInfo, StartUpError};

#[actix_web::main]
async fn main()-> Result<(), StartUpError> {
    let app_settings = AppSettings::load();

    LoggingBuilder::new((&app_settings).into())
        .init_default();

    debug!("app settings loaded {:?}", app_settings);

    let (server, sse_exchange_task)= match Application::new((&app_settings).into())
        .start(app_settings.clone()){
        Ok((server, sse_exchange_task)) => (server, sse_exchange_task),
        Err(e)=> {
            error!("Fail to start services {:?}", e);
            return Err(e);
        }
    };

    let ApplicationStartUpDisplayInfo{ environment_name, is_debug} = (&app_settings).into();
    info!(Environment=&environment_name[..], IsDebug=&is_debug[..], "Application started");

    let services_task = join_all(vec![tokio::spawn(server),sse_exchange_task]) ;
    select! {
        _ = services_task => {
            info!("services stopped");
        }
        _ = ctrl_c() => {
            info!("application terminated because of cancellation signal ctrl+c");
        }
    };

    Ok(())

}