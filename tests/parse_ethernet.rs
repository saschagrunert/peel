extern crate peel;
use peel::prelude::*;

extern crate nom;
use nom::{IResult, ErrorKind, Needed};

static ETH_HEADER: &'static [u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 8, 0];

#[test]
fn eth_parser_variant() {
    let parser = EthernetParser;
    println!("{:?}", parser.variant());
}

#[test]
fn parse_eth_success() {
    let parser = EthernetParser;
    let res = parser.parse(ETH_HEADER, None, None, None).unwrap();
    println!("{}", res.1);
    match res {
        (_, Layer::Ethernet(eth)) => {
            assert_eq!(EthernetPacket {
                           dst: MacAddress(1, 2, 3, 4, 5, 6),
                           src: MacAddress(7, 8, 9, 10, 11, 12),
                           ethertype: EtherType::Ipv4,
                       },
                       eth)
        }
        _ => {}
    }
}

#[test]
fn parse_eth_success_ethertypes() {
    let parser = EthernetParser;
    let mut input = Vec::from(ETH_HEADER); // IPv4
    parser.parse(&input, None, None, None).unwrap();

    input[12] = 0x86; // IPv6
    input[13] = 0xdd;
    parser.parse(&input, None, None, None).unwrap();
}

#[test]
fn parse_eth_failure_wrong_ethertype() {
    let parser = EthernetParser;
    let mut input = Vec::from(ETH_HEADER);
    input[13] = 0x55;
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res, IResult::Error(ErrorKind::MapOpt));
}

#[test]
fn parse_eth_failure_too_small() {
    let parser = EthernetParser;
    let mut input = Vec::from(ETH_HEADER);
    input.pop();
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res, IResult::Incomplete(Needed::Size(14)));
}
