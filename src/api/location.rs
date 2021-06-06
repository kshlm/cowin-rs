use serde::{Deserialize, Serialize};
use surf::{http::Method, Result};

use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    state_id: i16,
    state_name: String,
    state_name_l: Option<String>,
}

impl State {
    pub(crate) async fn get_districts(&self) -> Result<Districts> {
        Districts::get(self.state_id).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct States {
    states: Vec<State>,
    ttl: i16,
}

impl Client for States {
    const ENDPOINT: &'static str = "/v2/admin/location/states";
}

impl States {
    pub async fn get() -> Result<Self> {
        Self::request(Method::Get, None).recv_json::<Self>().await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct District {
    state_id: i16,
    district_id: i16,
    district_name: String,
    district_name_l: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Districts {
    districts: Vec<District>,
    ttl: i16,
}

impl Client for Districts {
    const ENDPOINT: &'static str = "/v2/admin/location/districts/";
}

impl Districts {
    pub async fn get(state_id: i16) -> Result<Self> {
        let url = match Self::url().join(state_id.to_string().as_str()) {
            Ok(url) => url,
            _ => unreachable!(),
        };
        Self::request(Method::Get, Some(url)).recv_json::<Self>().await
    }
}
