use chrono::{Local, NaiveDate, NaiveTime};
use cli_table::Table;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use surf::{http::Method, RequestBuilder};
use uuid::Uuid;

use crate::api::utils::{opti16_display, serde_date};
use crate::client::Client;

#[derive(Debug, Deserialize, Table)]
pub struct Session {
    #[table(title = "ID", order = 0)]
    center_id: i32,
    #[table(title = "Center", order = 1)]
    name: String,
    #[table(skip)]
    name_l: Option<String>,
    #[table(skip)]
    address: Option<String>,
    #[table(skip)]
    address_l: Option<String>,
    #[table(skip)]
    state_name: String,
    #[table(skip)]
    state_name_l: Option<String>,
    #[table(skip)]
    district_name: String,
    #[table(skip)]
    district_name_l: Option<String>,
    #[table(skip)]
    block_name: String,
    #[table(skip)]
    block_name_l: Option<String>,
    #[table(skip)]
    pincode: i32,
    #[table(skip)]
    lat: f32,
    #[table(skip)]
    long: f32,
    #[table(skip)]
    from: NaiveTime,
    #[table(skip)]
    to: NaiveTime,
    #[table(skip)]
    fee_type: FeeType,
    #[table(skip)]
    fee: String,
    #[table(skip)]
    session_id: Uuid,
    #[serde(with = "serde_date")]
    #[table(skip)]
    date: NaiveDate,
    #[table(title = "Capacity", order = 4)]
    available_capacity: i16,
    #[table(title = "Dose 1", display_fn = "opti16_display")]
    available_capacity_dose1: Option<i16>,
    #[table(title = "Dose 2", display_fn = "opti16_display")]
    available_capacity_dose2: Option<i16>,
    #[table(title = "Age")]
    min_age_limit: i8,
    #[table(title = "Vaccine")]
    vaccine: String,
    #[table(skip)]
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
