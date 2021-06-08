use surf::{
    http::{headers, mime::JSON, Method},
    RequestBuilder, Url,
};

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
        RequestBuilder::new(method, url.unwrap_or_else(Self::url))
            .header(headers::USER_AGENT, USER_AGENT)
            .header(headers::ACCEPT, JSON)
            .header(headers::ACCEPT_LANGUAGE, ACCEPT_LANGUAGE)
    }
}
