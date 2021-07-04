use time::{format_description,OffsetDateTime};
use std::time::{SystemTime};
use std::env;
use serde::{Deserialize};
use actix_web::{get, Responder, HttpResponse};
use crate::settings::AppSettings;
use actix_web::web::Data;
use std::borrow::Borrow;
use crate::contracts::GetAppInfoResponse;

pub const DATE_ISO_FORMAT:&str="[year]-[month]-[day] [hour]:[minute]:[second]";

#[derive(Debug, Deserialize, Clone)]
pub struct RuntimeInfo {
    pub git_commit_id: String,
    pub started : String,
}

impl RuntimeInfo {
    pub fn new() -> RuntimeInfo {
        let git_commit_id=match env::var("git_commit_sha") {
            Ok(sha) => sha,
            _ => String::from("local-dev")
        };
        RuntimeInfo {
            git_commit_id,
            started: systemtime_strftime(SystemTime::now(), DATE_ISO_FORMAT),
        }
    }
}

pub fn systemtime_strftime<T>(dt: T, format: &str) -> String
    where T: Into<OffsetDateTime>
{
    let format =  format_description::parse(format).unwrap();
    dt.into().format(&format).unwrap()
}

#[get("ping")]
pub async fn ping() -> impl Responder {
    format!("application running\n")
}

#[get("app-info")]
pub async fn app_info(app_settings: Data<AppSettings>) -> impl Responder {
    let app_name = app_settings.app_name.clone();
    let RuntimeInfo {git_commit_id, started}  = app_settings.runtime_info.borrow();
    HttpResponse::Ok().json(GetAppInfoResponse{
        app_name,
        git_commit_id: String::from(git_commit_id),
        started: String::from(started),
        current_time: systemtime_strftime(SystemTime::now(),DATE_ISO_FORMAT)
    })
}
