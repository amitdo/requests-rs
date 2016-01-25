use hyper;

pub type HyperResponse = hyper::client::response::Response;

#[derive(Debug)]
pub struct Response {
    pub url: String,
    pub status_code: u16,
    pub reason: String,
    raw: HyperResponse,
}

impl Response {
    pub fn from_hyper(raw: HyperResponse) -> Self {
        let status_code;
        let reason;
        {
            let &hyper::http::RawStatus(sc, ref r) = raw.status_raw();
            status_code = sc;
            reason = r.to_string();
        }
        Response {
            url: raw.url.serialize(),
            status_code: status_code,
            reason: reason,
            raw: raw,
        }
    }
}
