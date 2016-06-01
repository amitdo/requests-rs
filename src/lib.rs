#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod request;
pub mod response;

pub use request::{get, post, put, head, delete};
pub use response::Response;

pub type RequestsResult = hyper::Result<Response>;

#[cfg(test)]
mod test {
    use super::*;
    use hyper;

    #[derive(Debug, Deserialize)]
    struct Args;

    #[derive(Debug, Deserialize)]
    struct Headers {
        #[serde(rename="Host")]
        host: String,
        #[serde(rename="User-Agent")]
        user_agent: String,
    }

    #[derive(Debug, Deserialize)]
    struct HttpBinData {
        // args: Args,
        headers: Headers,
        origin: String,
        url: String,
    }

    #[test]
    fn simple_get() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        let data: HttpBinData = res.from_json().unwrap();
        assert_eq!(data.url, URL);
        assert_eq!(data.headers.host, "httpbin.org");
        assert_eq!(data.headers.user_agent, concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn simple_post() {
        const URL: &'static str = "http://httpbin.org/post";
        let res = post(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_put() {
        const URL: &'static str = "http://httpbin.org/put";
        let res = put(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_head() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = head(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn simple_delete() {
        const URL: &'static str = "http://httpbin.org/delete";
        let res = delete(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn user_agent() {
        let useragent = concat!("{\n  \"user-agent\": \"requests-rs/",
                                env!("CARGO_PKG_VERSION"),
                                "\"\n}\n");
        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = get(URL).unwrap();
        assert_eq!(res.url(), URL);
        assert_eq!(res.status_code(), hyper::Ok);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.text(), Some(useragent));
    }

    #[test]
    fn user_agent_json() {

        #[derive(Debug, Deserialize)]
        struct UserAgent {
            #[serde(rename="user-agent")]
            user_agent: String,
        }

        const URL: &'static str = "http://httpbin.org/user-agent";
        let res = get(URL).unwrap();
        assert!(res.is_json());

        let ua: UserAgent = res.from_json().unwrap();
        assert_eq!(ua.user_agent, concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
    }

    use std::collections::HashMap;

    #[derive(Deserialize, Debug)]
    struct GenericResponse {
        args: HashMap<String, String>,
        data: Option<String>,
        files: Option<HashMap<String, String>>,
        form: Option<HashMap<String, String>>,
        headers: HashMap<String, String>,
        json: Option<String>,
        origin: String,
        url: String,
    }

    #[test]
    fn generic_get() {
        const URL: &'static str = "http://httpbin.org/get";
        let res = get(URL).unwrap();
        assert!(res.is_json());

        println!("{}", res.text().unwrap());
        let out = res.from_json::<GenericResponse>().unwrap();
        println!("{:#?}", out);

        assert_eq!(out.json, None);
        assert_eq!(out.url, "http://httpbin.org/get");
    }
}
