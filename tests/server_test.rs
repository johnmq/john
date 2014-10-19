extern crate john;
extern crate http;
extern crate url;

//use john::{ClearCommand, PushCommand, PeekCommand};

use http::client::RequestWriter;
use http::status;
use url::Url;

fn get(url: &str) -> (status::Status, String) {
    let parsed_url = Url::parse(url).ok().expect("Invalid url");
    let request: RequestWriter = RequestWriter::new(http::method::Get, parsed_url).unwrap();

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_) => fail!("No response")
    };

    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(_) => fail!("No body")
    };

    let parsed_body = std::str::from_utf8(body.as_slice()).expect("Response is not UTF-8");

    (response.status, parsed_body.to_string())
}

#[test]
fn hello_route() {
    match get("http://localhost:3100/hello/world") {
        (status::Ok, greeting) => assert_eq!("Hello, world!", greeting.as_slice()),
        _ => fail!("Status should be status::Ok")
    }
}
