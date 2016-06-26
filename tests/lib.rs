extern crate hyper;
extern crate json;
extern crate requests;

use requests::{delete, get, head, post, put};

#[test]
fn simple_get() {
    const URL: &'static str = "http://httpbin.org/get";
    let res = get(URL).unwrap();
    assert_eq!(res.url(), URL);
    assert_eq!(res.status_code(), hyper::Ok);
    assert_eq!(res.reason(), "OK");
    let data = res.json().unwrap();
    println!("{:?}", data);
    assert_eq!(data["url"], URL);
    assert_eq!(data["headers"]["Host"], "httpbin.org");
    assert_eq!(data["headers"]["User-Agent"],
               concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
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

    const URL: &'static str = "http://httpbin.org/user-agent";
    let res = get(URL).unwrap();
    assert!(res.is_json());

    let ua = res.json().unwrap();
    assert_eq!(ua["user-agent"],
               concat!("requests-rs/", env!("CARGO_PKG_VERSION")));
}

#[test]
fn content() {
    const URL: &'static str = "http://httpbin.org/headers";
    let res = get(URL).unwrap();
    let content = vec![123, 10, 32, 32, 34, 104, 101, 97, 100, 101, 114, 115, 34, 58, 32, 123, 10,
                       32, 32, 32, 32, 34, 72, 111, 115, 116, 34, 58, 32, 34, 104, 116, 116, 112,
                       98, 105, 110, 46, 111, 114, 103, 34, 44, 32, 10, 32, 32, 32, 32, 34, 85,
                       115, 101, 114, 45, 65, 103, 101, 110, 116, 34, 58, 32, 34, 114, 101, 113,
                       117, 101, 115, 116, 115, 45, 114, 115, 47, 48, 46, 48, 46, 49, 55, 34, 10,
                       32, 32, 125, 10, 125, 10];
    assert_eq!(res.content(), &content);
}

macro_rules! status_code_test {
    ($($name:ident: $numeric:expr,)+) => {
        $(#[test]
        fn $name() {
            let res = get(&format!("http://httpbin.org/status/{}", $numeric)).unwrap();
            assert_eq!(res.status_code(), hyper::status::StatusCode::from_u16($numeric));
        })+
    }
}

status_code_test! {
    status_code_100: 100,
    status_code_101: 101,
    status_code_102: 102,
    status_code_200: 200,
    status_code_201: 201,
    status_code_202: 202,
    status_code_203: 203,
    status_code_400: 400,
    status_code_401: 401,
    status_code_402: 402,
    status_code_403: 403,
    status_code_404: 404,
    status_code_500: 500,
}
