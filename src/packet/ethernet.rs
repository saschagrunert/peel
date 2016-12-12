//! Ethernet related packet processing
use packet::prelude::*;

#[derive(Debug, Clone)]
/// The Ethernet parser
pub struct EthernetParser;

#[derive(Debug, Eq, PartialEq)]
/// Representation of the Ethernet structure
pub struct EthernetPacket {
    /// Destination mac address
    pub dst: MacAddress,

    /// Source mac address
    pub src: MacAddress,

    /// EtherType of the packet
    pub ethertype: EtherType,
}

#[derive(Debug, Eq, PartialEq)]
/// Representation of a mac network address, usually in the format "ff:ff:ff:ff:ff:ff"
pub struct MacAddress(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Eq, PartialEq)]
/// Supported `EtherType`
pub enum EtherType {
    /// Internet Protocol Version 4
    IPv4,

    /// Internet Protocol Version 6
    IPv6,
}

impl EtherType {
    /// Convert a u16 to an `EtherType`. Returns None if the type is not supported or generally
    /// invalid.
    pub fn from_u16(input: u16) -> Option<EtherType> {
        match input {
            0x0800 => Some(EtherType::IPv4),
            0x86DD => Some(EtherType::IPv6),
            _ => None,
        }
    }
}

impl Parser for EthernetParser {
    type Result = ParserResult;
    type Variant = ParserVariant;

    fn parse<'a, 'b>(&'a self,
                     input: &'b [u8],
                     _: &ParserNode<ParserResult, ParserVariant>,
                     _: &ParserArena<ParserResult, ParserVariant>)
                     -> IResult<&'b [u8], ParserResult> {
        do_parse!(input,
            d: take!(6) >>
            s: take!(6) >>
            e: map_opt!(be_u16, EtherType::from_u16) >>
            (ParserResult::Ethernet(EthernetPacket {
                dst: MacAddress(d[0], d[1], d[2], d[3], d[4], d[5]),
                src: MacAddress(s[0], s[1], s[2], s[3], s[4], s[5]),
                ethertype: e,
            }))
        )
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::Ethernet(self.clone())
    }
}
