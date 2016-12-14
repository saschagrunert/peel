extern crate peel;
use peel::prelude::*;

extern crate nom;
use nom::Needed;

static UDP_HEADER: &'static [u8] = &[0x00, 0x35, 0xc7, 0xde, 0x00, 0x92, 0xad, 0x1b];

#[test]
fn udp_parser_variant() {
    let parser = UdpParser;
    println!("{:?}", parser.variant());
}

#[test]
fn parse_udp_success() {
    let parser = UdpParser;
    let res = parser.parse(UDP_HEADER, None, None, None).unwrap();
    println!("{}", res.1);
    match res {
        (_, Layer::Udp(udp)) => {
            assert_eq!(UdpPacket {
                           source_port: 53,
                           dest_port: 51166,
                           length: 146,
                           checksum: 44315,
                       },
                       udp);
        }
        _ => {}
    }
}

#[test]
fn parse_udp_failure_too_small() {
    let parser = UdpParser;
    let mut input = Vec::from(UDP_HEADER);
    input.pop();
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res, IResult::Incomplete(Needed::Size(8)));
}
