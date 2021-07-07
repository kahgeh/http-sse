use std::collections::HashMap;
use derive_more::{Display, Error};
use actix_web::web::{Bytes};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{info,error};

use crate::sse_exchange::Command::{Connect, Shutdown, Publish};
use crate::sse_exchange::SseExchangeError::FailToEstablishConnection;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = message)]
    General {
        message: String,
    },
}

#[derive(Debug, Display, Error)]
pub enum SseExchangeError {
    FailToEstablishConnection,
}

pub struct Client {
    id: String,
    sender: Sender<Result<Bytes, CustomError>>,
}

impl Client {
    pub fn new(id: &str)->(Client, Receiver<Result<Bytes, CustomError>>){
        let (tx, rx) = channel::<Result<Bytes, CustomError>>(100);
        (Client { id:String::from(id), sender: tx }, rx)
    }
}

pub struct SseExchange {
    tx: Sender<Command>,
}

pub struct Event {
     client_id: String,
     payload: String,
}

impl Event {
    pub fn new(client_id :&str, payload:&str) -> Event {
        Event {
            client_id: String::from(client_id),
            payload: String::from(payload),
        }
    }
}

pub enum Command {
    Connect(Client),
    Publish(Event),
    Shutdown,
}

impl SseExchange {

    pub async fn connect(&self, client_id: &str) -> Result<ReceiverStream<Result<Bytes,CustomError>>, SseExchangeError> {
        let (client, rx )=Client::new(client_id);
        if self.tx.clone().send(Connect(client)).await.is_err() {
            error!(client_id=client_id,"error attempting to connect");
            return Err(FailToEstablishConnection)
        }
        info!("returning receiver stream");
        Ok(ReceiverStream::new(rx))
    }

    pub async fn publish(&self, event: Event) -> bool {
        self.tx.clone().send(Publish(event)).await.is_ok()
    }

    pub fn start() -> (JoinHandle<()>, SseExchange) {
        let (tx, mut rx) = channel::<Command>(100);
        let task=tokio::spawn(async move {
            let mut clients: HashMap<String, Sender<Result<Bytes, CustomError>>> = HashMap::new();
            info!("sse exchange started");
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Connect(client) => {
                        info!("sending 'connection established'...");
                        client.sender.send(Ok(Bytes::from("data: connection established\n\n"))).await.unwrap();
                        clients.insert(client.id, client.sender);
                    },
                    Command::Publish(event) => {
                        match clients.get(&event.client_id) {
                            Some(tx)=> tx.send(Ok(Bytes::from(format!("data: {}\n\n", event.payload)))).await.unwrap(),
                            None => info!(client_id=&event.client_id[..], "client no longer registered"),
                        }
                    },
                    Command::Shutdown => {
                        rx.close();
                    }
                }
            }
        });
        (task,SseExchange{
            tx
        })
    }

    pub async fn stop(&self) {
        info!("shutting down");
        if self.tx.clone().send(Shutdown).await.is_err() {
            error!("error shutting down");
        }
    }

}

