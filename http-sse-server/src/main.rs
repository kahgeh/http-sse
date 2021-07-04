mod settings;
mod app_ops;
mod logging;
mod application;
mod contracts;
mod mappers;

use std::time::{SystemTime};
use actix_web::{get,Responder, web, HttpResponse};
use std::borrow::Borrow;
use tracing::{info};

use crate::settings::{AppSettings};
use crate::contracts::{GetAppInfoResponse};
use crate::app_ops::*;
use crate::logging::{LoggingBuilder};
use crate::application::{Application, ApplicationStartUpDisplayInfo};

#[get("ping")]
async fn ping() -> impl Responder {
    format!("application running\n")
}

#[get("app-info")]
async fn app_info(app_settings: web::Data<AppSettings>) -> impl Responder {
    let app_name = app_settings.app_name.clone();
    let RuntimeInfo {git_commit_id, started}  = app_settings.runtime_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name,
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
    })
}

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