mod settings;

use std::time::{SystemTime};
use actix_web::{get,Responder, App, HttpServer, web, HttpResponse};
use serde::{Serialize};
use std::sync::*;
use std::env;
use std::borrow::Borrow;
use time::{format_description,OffsetDateTime};
use crate::settings::Settings;


const DATE_ISO_FORMAT:&str="[year]-[month]-[day] [hour]:[minute]:[second]";

struct AppInfo {
    app_name: String,
    git_commit_id: String,
    started : String,
}

impl AppInfo {
    fn new() -> AppInfo {
        let git_commit_id=match env::var("git_commit_sha") {
          Ok(sha) => sha,
          _ => String::from("local-dev")
        };
        AppInfo {
            app_name: String::from("http-sse-server"),
            git_commit_id,
            started: systemtime_strftime(SystemTime::now(), DATE_ISO_FORMAT),
        }
    }
}

struct AppState {
    settings: Settings,
    app_info : Arc<AppInfo>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            settings: Settings::new().expect("fail to load settings"),
            app_info: Arc::new(AppInfo::new())
        }
    }

    fn get_url_prefix(&self)->String {
        self.settings.url_prefix.clone()
    }

    fn get_port(&self)->u16 {
        self.settings.port
    }
}

#[derive(Serialize)]
struct GetAppInfoResponse {
    app_name: String,
    git_commit_id: String,
    started : String,
    current_time : String,
}

fn systemtime_strftime<T>(dt: T, format: &str) -> String
    where T: Into<OffsetDateTime>
{
    let format =  format_description::parse(format).unwrap();
    dt.into().format(&format).unwrap()
}

#[get("ping")]
async fn ping() -> impl Responder {
    format!("pong\n")
}

#[get("app-info")]
async fn app_info(app_config: web::Data<AppState>) -> impl Responder {
    let AppInfo {app_name,git_commit_id, started}  = app_config.borrow().app_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name: String::from(app_name),
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
    })
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let app_config= web::Data::new(AppState::new());
    let url_prefix = app_config.get_url_prefix();
    let address = format!("0.0.0.0:{}", app_config.get_port());

    HttpServer::new(move ||{
        App::new()
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