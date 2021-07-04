use actix_web::dev::Server;
use actix_web::{ App, HttpServer, web};
use tracing_actix_web::TracingLogger;
use crate::logging::HttpAppRootSpanBuilder;
use crate::{ping, app_info};
use std::net::TcpListener;
use std::io::Error;
use actix_web::web::Data;
use crate::settings::AppSettings;

pub struct HttpServerSettings {
    url_prefix: String,
    port: u16,
}

impl HttpServerSettings {
    pub fn new(url_prefix: &str, port: u16) -> HttpServerSettings {
        HttpServerSettings {
            url_prefix: String::from(url_prefix),
            port,
        }
    }

    pub fn create_listener(&self) -> Result<TcpListener, Error>{
        let address = format!("0.0.0.0:{}", self.port);
        TcpListener::bind(&address)
    }
}

pub struct ApplicationStartUpDisplayInfo {
    pub environment_name: String,
    pub is_debug: String,
}

impl ApplicationStartUpDisplayInfo {
    pub fn new(environment_name: &str, is_debug: bool )->ApplicationStartUpDisplayInfo{
        ApplicationStartUpDisplayInfo{
            environment_name: String::from(environment_name),
            is_debug : match is_debug { true => String::from("true"), _ => String::from("false") }
        }
    }
}

pub struct Application {
    settings: HttpServerSettings,
}

impl Application {
    pub fn new(settings: HttpServerSettings) ->Application {
        Application {
            settings
        }
    }

    pub fn start(&self, app_settings:AppSettings) -> Result<Server, std::io::Error>{
        let listener = self.settings.create_listener()?;
        let url_prefix = self.settings.url_prefix.clone();
        let server=HttpServer::new( move ||{
            App::new()
                .app_data(Data::new(app_settings.clone()))
                .wrap(TracingLogger::<HttpAppRootSpanBuilder>::new())
                .service(
                    web::scope(url_prefix.as_str())
                        .service(ping)
                        .service(app_info))
        })
            .listen(listener)?
            .run();

        Ok(server)
    }
}

