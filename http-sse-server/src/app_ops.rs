use time::{format_description,OffsetDateTime};
use std::time::{SystemTime};
use std::env;

pub const DATE_ISO_FORMAT:&str="[year]-[month]-[day] [hour]:[minute]:[second]";

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