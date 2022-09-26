use actix_web::dev::Server;
use actix_web::{ App, HttpServer, web};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use std::io::Error;
use tokio::task::JoinHandle;
use actix_web::web::Data;
use tracing::{error};
use derive_more::{Display, Error};

use crate::settings::AppSettings;
use crate::app_ops::{ping, app_info};
use crate::routes::{receive_connect_request, receive_send_request, receive_send_broadcast};

use crate::sse_exchange::{SseExchange};
use crate::peers;
use crate::application::StartUpError::{FailToParseCompute, FailToStartHttpServer, FailToStartTcpListener};
use crate::logging::HttpAppRootSpanBuilder;

#[derive(Debug, Display, Error)]
pub enum StartUpError {
    FailToParseCompute,
    FailToStartTcpListener(std::io::Error),
    FailToStartHttpServer(std::io::Error),
}

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

    pub fn start(&self, app_settings:AppSettings) -> Result<(Server, JoinHandle<tokio::io::Result<()>>), StartUpError>{
        let listener = match self.settings.create_listener() {
            Ok(l)=>l,
            Err(e)=> return Err(FailToStartTcpListener(e)),
        };

        let url_prefix = self.settings.url_prefix.clone();
        let (sse_exchange_task, sse_exchange) = SseExchange::start();
        let sse_exchange= Data::new(sse_exchange);
        let compute_name = app_settings.clone().settings.compute;
        let compute = match peers::Compute::from_str(
            &compute_name,
            &app_settings) {
            Ok(service)=> service,
            Err(_) => {
                error!("error creating compute");
                return Err(FailToParseCompute);
            }
        };

        let discovery_service = Data::new(compute);
        let server=HttpServer::new( move ||{
            App::new()
                .app_data(Data::new(app_settings.clone()))
                .app_data(Data::new(app_settings.clone().settings))
                .app_data(sse_exchange.clone())
                .app_data(discovery_service.clone())
                .wrap(TracingLogger::<HttpAppRootSpanBuilder>::new())
                .service(
                    web::scope(url_prefix.as_str())
                        .service(ping)
                        .service(app_info)
                        .service(receive_connect_request)
                        .service(receive_send_request)
                        .service(receive_send_broadcast)
                )
            })
            .listen(listener).map_err(|e|FailToStartHttpServer(e))?;

        Ok((server.run(),sse_exchange_task))
    }
}

