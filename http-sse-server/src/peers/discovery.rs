use derive_more::{Display, Error};

pub struct PeerEndpoint {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Display, Error)]
pub enum DiscoveryError {
    Connect {message:String},
    General {message:String}
}

pub trait Discoverable{
    fn get_service_endpoints(&self)->PeerEndpoint;
}
