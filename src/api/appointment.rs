use chrono::{Local, NaiveDate, NaiveTime};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use surf::{http::Method, RequestBuilder};
use tabled::Tabled;
use uuid::Uuid;

use crate::api::utils::{opti16_display, serde_date};
use crate::client::Client;

#[derive(Debug, Deserialize, Tabled)]
pub struct Session {
    #[header("ID", order = 0)]
    center_id: i32,
    #[header("Center", order = 1)]
    name: String,
    #[header(hidden)]
    name_l: Option<String>,
    #[header(hidden)]
    address: Option<String>,
    #[header(hidden)]
    address_l: Option<String>,
    #[header(hidden)]
    state_name: String,
    #[header(hidden)]
    state_name_l: Option<String>,
    #[header(hidden)]
    district_name: String,
    #[header(hidden)]
    district_name_l: Option<String>,
    #[header(hidden)]
    block_name: String,
    #[header(hidden)]
    block_name_l: Option<String>,
    #[header(hidden)]
    pincode: i32,
    #[header(hidden)]
    lat: f32,
    #[header(hidden)]
    long: f32,
    #[header(hidden)]
    from: NaiveTime,
    #[header(hidden)]
    to: NaiveTime,
    #[header(hidden)]
    fee_type: FeeType,
    #[header(hidden)]
    fee: String,
    #[header(hidden)]
    session_id: Uuid,
    #[serde(with = "serde_date")]
    #[header(hidden)]
    date: NaiveDate,
    #[header("Capacity", order = 4)]
    available_capacity: i16,
    #[header("Dose 1")]
    #[field(display_with = "opti16_display")]
    available_capacity_dose1: Option<i16>,
    #[header("Dose 2")]
    #[field(display_with = "opti16_display")]
    available_capacity_dose2: Option<i16>,
    #[header("Age")]
    min_age_limit: i8,
    #[header("Vaccine")]
    vaccine: String,
    #[header(hidden)]
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

    pub async fn get_by_pin(pincode: &str, date: Option<NaiveDate>) -> Result<Vec<Session>> {
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

    pub async fn get_by_district(
        district_id: i16,
        date: Option<NaiveDate>,
    ) -> Result<Vec<Session>> {
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

    async fn run(request: RequestBuilder, query: SessionsQuery) -> Result<Vec<Session>> {
        let Self { sessions } = request
            .query(&query)
            .map_err(|e| eyre!(e))?
            .recv_json::<Self>()
            .await
            .map_err(|e| eyre!(e))?;

        let mut sessions = sessions
            .into_iter()
            .filter(|s| s.available_capacity > 0)
            .collect::<Vec<Session>>();

        sessions.sort_by(|a, b| b.available_capacity.cmp(&a.available_capacity));
        Ok(sessions)
    }
}
