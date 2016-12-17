extern crate peel;
use peel::prelude::*;

extern crate nom;
use nom::{ErrorKind, Needed, Err};

#[test]
fn parse_http_success() {
    let parser = HttpParser;
    let res = parser.parse(b"GET /\r\nHost: myhost.com\r\n\r\n", None, None, None).unwrap().1;
    println!("{}", res.0);
    assert_eq!(res.0,
               Layer::Http(Some(HttpPacket { request_method: HttpRequestMethod::Get })));

    let res = parser.parse(b"POST /\r\nHost: myhost.com\r\n\r\n", None, None, None).unwrap().1;
    assert_eq!(res.0,
               Layer::Http(Some(HttpPacket { request_method: HttpRequestMethod::Post })));
}

#[test]
fn parse_http_failure_wrong_method() {
    let parser = HttpParser;
    assert_eq!(parser.parse(b"GET", None, None, None),
               IResult::Incomplete(Needed::Size(4)));

    let input = b"GOT ";
    assert_eq!(parser.parse(input, None, None, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}
