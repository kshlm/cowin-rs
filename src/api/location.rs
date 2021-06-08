use async_std::fs;
use chrono::{DateTime, Duration, Utc};
use eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use surf::http::Method;

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
        Self::request(Method::Get, None)
            .recv_json::<Self>()
            .await
            .map_err(|e| eyre!(e))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct District {
    district_id: i16,
    district_name: String,
    district_name_l: Option<String>,
    state_id: Option<i16>,
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
        Self::request(Method::Get, Some(url))
            .recv_json::<Self>()
            .await
            .map_err(|e| eyre!(e))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatesAndDistricts {
    states: Vec<State>,
    districts: Vec<District>,
    expires_at: DateTime<Utc>,
}

impl StatesAndDistricts {
    const CACHE_FILE: &'static str = "cowin-states-districts.json";

    pub fn new(states: Vec<State>, districts: Vec<District>, expires_at: DateTime<Utc>) -> Self {
        Self {
            states,
            districts,
            expires_at,
        }
    }

    pub async fn get() -> Result<Self> {
        match Self::load_cache().await {
            Ok(ret) => Ok(ret),
            _ => Self::refresh().await,
        }
    }

    pub async fn refresh() -> Result<Self> {
        let states = States::get().await?;
        let mut ttls = vec![states.ttl];

        let mut districts: Vec<District> = Vec::new();
        for state in &states.states {
            let mut ds = state.get_districts().await?;
            // ds.districts.iter_mut().map(|d| d.state_id = Some(state.state_id);
            for d in ds.districts.iter_mut() {
                d.state_id = Some(state.state_id);
            }
            districts.append(&mut ds.districts);
            ttls.push(ds.ttl);
        }

        let min_ttl: i64 = match ttls.iter().min() {
            Some(ttl) => ttl.to_owned().into(),
            _ => unreachable!(),
        };

        let sd = Self::new(
            states.states,
            districts,
            Utc::now() + Duration::hours(min_ttl),
        );

        if let Ok(data) = serde_json::to_vec_pretty(&sd) {
            let _ = fs::write(Self::CACHE_FILE, data).await;
        }

        Ok(sd)
    }

    async fn load_cache() -> Result<Self> {
        let cache_data = fs::read(Self::CACHE_FILE)
            .await
            .wrap_err_with(|| "failed to read cache file")?;
        let sd: Self = from_slice(&cache_data).wrap_err_with(|| "failed to parse cache file")?;

        if sd.expires_at > Utc::now() {
            Ok(sd)
        } else {
            Err(eyre!("Cache expired"))
        }
    }

    pub fn district_id(&self, state: &str, district: &str) -> Result<i16> {
        let state = self
            .states
            .iter()
            .find(|s| s.state_name == state)
            .ok_or_else(|| eyre!("state not found"))?;
        let district = self
            .districts
            .iter()
            .filter(|d| d.state_id == Some(state.state_id))
            .find(|d| d.district_name == district)
            .ok_or_else(|| eyre!("district not found"))?;
        Ok(district.district_id)
    }
}
