use std::time::{SystemTime};
use actix_web::{get,Responder, App, HttpServer, web, HttpResponse};
use serde::{Serialize};
use std::sync::*;
use std::env;
use std::borrow::Borrow;
use time::{format_description,OffsetDateTime};

const DATE_ISO_FORMAT:&str="[year]-[month]-[day] [hour]:[minute]:[second]";
const DEFAULT_PORT: u16 = 8080;

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

struct AppConfig {
    url_prefix: String,
    port: u16,
    app_info : Arc<AppInfo>,
}

impl AppConfig {
    fn new() -> AppConfig {
        let url_prefix =match env::var("url_prefix") {
            Ok(prefix) => prefix,
            _ => String::from("")
        };

        let port: u16 =match env::var("port") {
            Ok(port) => port.parse().unwrap(),
            _ => DEFAULT_PORT
        };

        AppConfig {
            port,
            url_prefix,
            app_info: Arc::new(AppInfo::new())
        }
    }

    fn get_url_prefix(&self)->String {
        self.url_prefix.clone()
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
async fn app_info(app_config: web::Data<AppConfig>) -> impl Responder {
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
    let app_config= web::Data::new(AppConfig::new());
    let url_prefix = app_config.get_url_prefix();
    let address = format!("0.0.0.0:{}", app_config.port);

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