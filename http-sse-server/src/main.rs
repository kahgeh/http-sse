mod settings;
mod app_ops;
mod logging;
mod application;

use std::time::{SystemTime};
use actix_web::{get,Responder, web, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::*;
use std::borrow::Borrow;
use tracing::{info};

use crate::settings::Settings;
use crate::app_ops::*;
use crate::logging::{ LoggingBuilder, LogSettings};
use crate::application::{HttpServerSettings, Application, ApplicationStartUpDisplayInfo};

const APP_NAME: &str="http-sse-server";

#[derive(Debug, Deserialize, Clone)]
pub struct AppState {
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
}

impl From<&AppState> for LogSettings {
    fn from(app_state: &AppState) -> Self {
        LogSettings::new(
            app_state.app_name.as_str(),
            app_state.settings.log_level.as_str()
        )
    }
}

impl From<&AppState> for HttpServerSettings {
    fn from(app_state: &AppState) -> Self {
        HttpServerSettings::new(
            app_state.settings.url_prefix.as_str(),
            app_state.settings.port,
            )
    }
}

impl From<&AppState> for ApplicationStartUpDisplayInfo {
    fn from(app_state: &AppState) -> Self {
        ApplicationStartUpDisplayInfo::new(
            app_state.settings.environment.as_str(),
            app_state.settings.debug,
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
async fn app_info(app_settings: web::Data<AppState>) -> impl Responder {
    let app_name = app_settings.borrow().app_name.clone();
    let RuntimeInfo {git_commit_id, started}  = app_settings.borrow().runtime_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name,
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
    })
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let app_settings = AppState::new();

    LoggingBuilder::new((&app_settings).into())
        .init_default();

    let server = Application::new((&app_settings).into())
        .start(app_settings.clone())?;

    let ApplicationStartUpDisplayInfo{ environment_name, is_debug} = (&app_settings).into();
    info!(Environment=&environment_name[..], IsDebug=&is_debug[..], "Application started");
    server.await
}