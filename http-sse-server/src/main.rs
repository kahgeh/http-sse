mod settings;
mod app_ops;
mod logging;

use std::time::{SystemTime};
use actix_web::{get,Responder, App, HttpServer, web, HttpResponse};
use serde::{Serialize};
use std::sync::*;
use std::borrow::Borrow;
use tracing::{info};

use crate::settings::Settings;
use crate::app_ops::*;
use crate::logging::{HttpAppRootSpanBuilder, LoggingBuilder, LogSettings};
use tracing_actix_web::{TracingLogger};

const APP_NAME: &str="http-sse-server";
struct AppState {
    app_name: String,
    settings: Settings,
    runtime_info : Arc<RuntimeInfo>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            app_name: String::from(APP_NAME),
            settings: Settings::new().expect("fail to load settings"),
            runtime_info: Arc::new(RuntimeInfo::new())
        }
    }

    fn get_url_prefix(&self)->String {
        self.settings.url_prefix.clone()
    }

    fn get_port(&self)->u16 {
        self.settings.port
    }
}

impl From<&actix_web::web::Data<AppState>> for LogSettings {
    fn from(app_state: &actix_web::web::Data<AppState>) -> Self {
        LogSettings::new(
            app_state.app_name.as_str(),
            app_state.settings.log_level.as_str()
        )
    }
}

#[derive(Serialize)]
struct GetAppInfoResponse {
    app_name: String,
    git_commit_id: String,
    started : String,
    current_time : String,
}

#[get("ping")]
async fn ping() -> impl Responder {
    format!("application running\n")
}

#[get("app-info")]
async fn app_info(app_config: web::Data<AppState>) -> impl Responder {
    let app_name = app_config.borrow().app_name.clone();
    let RuntimeInfo {git_commit_id, started}  = app_config.borrow().runtime_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name,
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
    })
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let app_config= web::Data::new(AppState::new());

    LoggingBuilder::new(app_config.borrow().into())
        .init_default();

    info!("Application started");

    let url_prefix = app_config.get_url_prefix();
    let address = format!("0.0.0.0:{}", app_config.get_port());

    HttpServer::new(move ||{
        App::new()
            .wrap(TracingLogger::<HttpAppRootSpanBuilder>::new())
            .app_data(app_config.clone())
            .service(
                web::scope(url_prefix.as_str())
                    .service(ping)
                    .service(app_info))
        })
        .bind(address)?
        .run()
        .await
}