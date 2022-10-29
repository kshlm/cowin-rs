use reqwest::{header, Client as rClient, Method, RequestBuilder, Url};

const USER_AGENT: &str = "cowin-rs";
const ACCEPT_LANGUAGE: &str = "en_US";
const BASE_URL: &str = "https://cdn-api.co-vin.in/api";

pub(crate) trait Client {
    const ENDPOINT: &'static str;

    fn url() -> Url {
        let url = &format!("{}{}", BASE_URL, Self::ENDPOINT);
        match Url::parse(url) {
            Ok(url) => url,
            _ => unreachable!(),
        }
    }

    fn request(method: Method, url: Option<Url>) -> RequestBuilder {
        rClient::new()
            .request(method, url.unwrap_or_else(Self::url))
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::ACCEPT, mime::JSON.as_str())
            .header(header::ACCEPT_LANGUAGE, ACCEPT_LANGUAGE)
    }
}
