use chrono::{Local, NaiveDate, NaiveTime};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use surf::{http::Method, RequestBuilder};
use uuid::Uuid;

use crate::api::utils::serde_date;
use crate::client::Client;

#[derive(Debug, Deserialize)]
pub struct Session {
    center_id: i32,
    name: String,
    name_l: Option<String>,
    address: Option<String>,
    address_l: Option<String>,
    state_name: String,
    state_name_l: Option<String>,
    district_name: String,
    district_name_l: Option<String>,
    block_name: String,
    block_name_l: Option<String>,
    pincode: i32,
    lat: f32,
    long: f32,
    from: NaiveTime,
    to: NaiveTime,
    fee_type: FeeType,
    fee: String,
    session_id: Uuid,
    #[serde(with = "serde_date")]
    date: NaiveDate,
    available_capacity: i16,
    available_capacity_dose1: Option<i16>,
    available_capacity_dose2: Option<i16>,
    min_age_limit: i8,
    vaccine: String,
    slots: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub enum FeeType {
    Paid,
    Free,
}

#[derive(Debug, Deserialize)]
pub struct Sessions {
    sessions: Vec<Session>,
}

#[derive(Serialize)]
struct SessionsQuery {
    #[serde(with = "serde_date")]
    date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pincode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    district_id: Option<i16>,
}

impl SessionsQuery {
    fn new(date: Option<NaiveDate>, pincode: Option<String>, district_id: Option<i16>) -> Self {
        let date = date.unwrap_or_else(|| Local::today().naive_local());
        Self {
            date,
            pincode,
            district_id,
        }
    }
}

impl Client for Sessions {
    const ENDPOINT: &'static str = "/v2/appointment/sessions/public/";
}

impl Sessions {
    const BY_PIN: &'static str = "findByPin";
    const BY_DISTRICT: &'static str = "findByDistrict";

    pub async fn get_by_pin(pincode: &str, date: Option<NaiveDate>) -> Result<Self> {
        let url = match Self::url().join(Self::BY_PIN) {
            Ok(url) => url,
            _ => unreachable!(),
        };
        Self::run(
            Self::request(Method::Get, Some(url)),
            SessionsQuery::new(date, Some(pincode.to_string()), None),
        )
        .await
    }

    pub async fn get_by_district(district_id: i16, date: Option<NaiveDate>) -> Result<Self> {
        let url = match Self::url().join(Self::BY_DISTRICT) {
            Ok(url) => url,
            _ => unreachable!(),
        };
        Self::run(
            Self::request(Method::Get, Some(url)),
            SessionsQuery::new(date, None, Some(district_id)),
        )
        .await
    }

    async fn run(request: RequestBuilder, query: SessionsQuery) -> Result<Self> {
        request
            .query(&query)
            .map_err(|e| eyre!(e))?
            .recv_json::<Self>()
            .await
            .map_err(|e| eyre!(e))
    }
}
