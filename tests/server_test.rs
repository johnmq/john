extern crate john;
extern crate http;
extern crate url;
extern crate serialize;

use john::{ClearCommand, PushCommand, PeekResult, PeekCommand};

use http::client::RequestWriter;
use http::status;
use url::Url;

use serialize::json;

fn get(url: String) -> (status::Status, String) {
    let parsed_url = Url::parse(url.as_slice()).ok().expect("Invalid url");
    let request: RequestWriter = RequestWriter::new(http::method::Get, parsed_url).unwrap();

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_) => panic!("No response")
    };

    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(_) => panic!("No body")
    };

    let parsed_body = std::str::from_utf8(body.as_slice()).expect("Response is not UTF-8");

    (response.status, parsed_body.to_string())
}

fn post(url: String, body: String) -> (status::Status, String) {
    let parsed_url = Url::parse(url.as_slice()).ok().expect("Invalid url");
    let mut request: RequestWriter = RequestWriter::new(http::method::Post, parsed_url).unwrap();
    let data = body.as_slice().as_bytes();

    request.headers.content_length = Some(data.len());
    request.write(data);

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_) => panic!("No response")
    };

    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(_) => panic!("No body")
    };

    let parsed_body = std::str::from_utf8(body.as_slice()).expect("Response is not UTF-8");

    (response.status, parsed_body.to_string())
}

fn test_url(path: &str) -> String {
    format!("http://localhost:3100{}", path)
}

#[test]
fn hello_route() {
    match get(test_url("/hello/world")) {
        (status::Ok, greeting) => assert_eq!("Hello, world!", greeting.as_slice()),
        _ => panic!("Status should be status::Ok")
    }
}

#[test]
fn peek_on_empty_river_without_offset() {
    ClearCommand::new().execute("server_side_river");
    match get(test_url("/peek/server_side_river")) {
        (status::NotFound, _) => {},
        _ => panic!("Status should be status::NotFound")
    }
}

#[test]
fn peek_on_full_river_without_offset() {
    ClearCommand::new().execute("server_side_river_2");
    PushCommand::new().execute("server_side_river_2", "a message");
    PushCommand::new().execute("server_side_river_2", "a message 2");
    PushCommand::new().execute("server_side_river_2", "a message 3");

    match get(test_url("/peek/server_side_river_2")) {
        (status::Ok, json) => match json::decode::< PeekResult >(json.as_slice()) {
            Ok(PeekResult { message, offset }) => {
                assert_eq!("a message 3", message.as_slice());
                assert_eq!(4, offset);
            },
            _ => panic!("Unable to parse response into PeekResult")
        },
        _ => panic!("Status should be status::Ok")
    }
}

#[test]
fn peek_on_full_river_with_some_offset() {
    ClearCommand::new().execute("server_side_river_3");
    PushCommand::new().execute("server_side_river_3", "a message");
    PushCommand::new().execute("server_side_river_3", "a message 2");
    PushCommand::new().execute("server_side_river_3", "a message 3");
    PushCommand::new().execute("server_side_river_3", "a message 4");

    match get(test_url("/peek/server_side_river_3/2")) {
        (status::Ok, json) => match json::decode::< PeekResult >(json.as_slice()) {
            Ok(PeekResult { message, offset }) => {
                assert_eq!("a message 2", message.as_slice());
                assert_eq!(3, offset);
            },
            _ => panic!("Unable to parse response into PeekResult")
        },
        _ => panic!("Status should be status::Ok")
    }
}

#[test]
fn push_on_full_river_with_some_offset() {
    ClearCommand::new().execute("server_side_river_4");
    PushCommand::new().execute("server_side_river_4", "a message");
    PushCommand::new().execute("server_side_river_4", "a message 2");
    PushCommand::new().execute("server_side_river_4", "a message 3");
    PushCommand::new().execute("server_side_river_4", "a message 4");

    match post(test_url("/push/server_side_river_4"), "super message".to_string()) {
        (status::Created, _) => {},
        _ => panic!("Status should be status::Created")
    }

    match PeekCommand::new().execute("server_side_river_4", None) {
        Some(PeekResult { message, offset }) => {
            assert_eq!("super message", message.as_slice());
            assert_eq!(6, offset);
        },
        _ => panic!("New message should have been created")
    }
}
