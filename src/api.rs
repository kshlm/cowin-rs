use serde::{Deserialize, Serialize};
use surf::{http::Method, Result};

use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    state_id: i16,
    state_name: String,
    state_name_l: Option<String>,
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
