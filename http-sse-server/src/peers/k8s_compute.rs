use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::{Client, Error};
use k8s_openapi::api::core::v1::{Endpoints};
use crate::peers::{PeerEndpoint, DiscoveryError};
use crate::peers::DiscoveryError::Connect;
use tokio::time::sleep;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct K8sDiscovery {
    pub namespace: String,
    pub service_name: String,
    pub interval_between_watches_secs: Duration,
}

impl K8sDiscovery {
    pub async fn start(&self)->JoinHandle<()>{
        tokio::spawn(async {
            loop{

                watch_and_report().await;
                sleep(self.interval_between_watches_secs).await?
            }
        })
    }

    async fn watch_and_report(){
        let api: Api<Endpoints> = Api::namespaced(client, &namespace);
        let lp = ListParams::default()
            .fields(&format!("metadata.name={}", service_name))
            .timeout(60);

        let mut stream= api.watch(&lp, "0").await?.boxed();
        while let Some(status) = stream.try_next().await? {
            match status {
                WatchEvent::Added(o) => println!("Added {}", o.name()),
                WatchEvent::Modified(o) => {
                    println!("Modified: {} ", o.name());
                }
                WatchEvent::Deleted(o) => println!("Deleted {}", o.name()),
                WatchEvent::Error(e) => println!("Error {}", e.message),
                _ => {}
            }
        }
    }

    pub async fn get_service_endpoints(&self) -> Result<Vec<PeerEndpoint>, DiscoveryError> {
        let client = Client::try_default().await
            .map_err(|_|Connect{message:String::from("k8s client failed to connect")})?;

        let api: Api<Endpoints> = Api::namespaced(client, &self.namespace);
        let mut peer_endpoints: Vec<PeerEndpoint> = vec![];
        if let Ok(endpoints) = api.get(&self.service_name).await {
            for endpoint_subset in endpoints.subsets {
                let port = endpoint_subset.ports[0].port;
                for address in endpoint_subset.addresses {
                    peer_endpoints.push(PeerEndpoint {
                        ip: address.ip,
                        port: port as u16,
                    })
                }
            }
        }

        Ok(peer_endpoints)
    }
}



