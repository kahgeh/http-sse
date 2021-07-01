use std::time::{SystemTime};
use actix_web::{get, Responder, App, HttpServer, web, HttpResponse};
use serde::{Serialize};
use std::sync::*;
use std::borrow::Borrow;
use time::{format_description,OffsetDateTime};

const date_iso_format:&str="[year]-[month]-[day] [hour]:[minute]:[second]";

struct AppInfo {
    app_name: String,
    git_commit_id: String,
    started : String,
    // current_time :
}

struct AppConfig {
    app_info : Arc<AppInfo>,
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



#[get("/ping")]
async fn ping() -> impl Responder {
    format!("pong\n")
}

#[get("/app-info")]
async fn app_info(app_config: web::Data<AppConfig>) -> impl Responder {
    let AppInfo {app_name,git_commit_id, started}  = app_config.borrow().app_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name: String::from(app_name),
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),date_iso_format)
    })
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .data(AppConfig {
                app_info: Arc::new(AppInfo {
                    app_name: String::from("http-sse-server"),
                    git_commit_id: String::from("local-dev"),
                    started: systemtime_strftime(SystemTime::now(), date_iso_format),
                })
            })
            .service(ping)
            .service(app_info)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}