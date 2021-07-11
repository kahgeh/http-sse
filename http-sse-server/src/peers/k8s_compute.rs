use k8s_openapi::api::core::v1::Endpoints;
use kube::{
    api::{Api},
    Client,
};
use crate::peers::{PeerEndpoint, DiscoveryError};
use crate::peers::DiscoveryError::Connect;

pub struct K8sDiscovery {
    pub namespace: String,
    pub service_name: String,
}

impl K8sDiscovery {
    // todo: tap into watcher for more efficient updates
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



