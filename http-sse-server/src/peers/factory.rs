use derive_more::{ Display, Error};

use crate::peers::{K8sDiscovery, PeerEndpoint, DiscoveryError};
use crate::peers::Compute::{K8s, DevBox};
use crate::settings::{AppSettings};
use crate::peers::ParseToComputeError::NotValid;

pub enum Compute {
    K8s(K8sDiscovery),
    DevBox(DevBoxDiscovery),
}

pub struct DevBoxDiscovery {
    pub ip: String,
    pub port: u16
}

impl DevBoxDiscovery {
    pub fn get_service_endpoints(&self)->Vec<PeerEndpoint>{
        vec![PeerEndpoint{ip:self.ip.clone(), port:self.port}]
    }
}

#[derive(Debug, Error, Display)]
pub enum ParseToComputeError{
    NotValid{ message: String }
}

impl Compute {
    pub fn from_str(input:&str, app_settings: &AppSettings) -> Result<Compute, ParseToComputeError> {
        match input {
            "K8s" => Ok(K8s(app_settings.into())),
            "DevBox" => Ok(DevBox(app_settings.into())),
            _ => Err(NotValid{message: String::from(input)})
        }
    }

    // macro learning opportunity - how can we auto generate this
    // - only after making get_service_endpoints
    // async as well
    pub async fn get_service_endpoints(&self)->Result<Vec<PeerEndpoint>, DiscoveryError> {
        match self {
            K8s(k8s)=> Ok(k8s.get_service_endpoints().await?),
            DevBox(dev_box)=> Ok(dev_box.get_service_endpoints())
        }
    }
}

