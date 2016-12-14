//! Example parsers for the dynamic data structure
pub mod layer1;
pub mod layer2;
pub mod layer3;
pub mod layer4;

use prelude::*;
use std::{fmt, str};

/// Provides sensible imports for packet parsers
pub mod prelude {
    pub use std::net::{Ipv4Addr, Ipv6Addr};
    pub use nom::{be_u8, be_i8, be_u16, be_u32, be_u64, IResult};

    pub use super::{Layer, ParserVariant, get_packet_peel};

    /// A general shorthand for the packet parsing tree
    pub type PacketPeel = ::Peel<Layer, ParserVariant>;

    /// Link
    pub use super::layer1::*;
    pub use super::layer1::ethernet::*;

    /// Internet
    pub use super::layer2::*;
    pub use super::layer2::ipv4::*;
    pub use super::layer2::ipv6::*;

    // Transport
    pub use super::layer3::*;
    pub use super::layer3::tcp::*;
    pub use super::layer3::tls::*;
    pub use super::layer3::udp::*;

    // Application
    pub use super::layer4::ntp::*;
}

#[derive(Debug)]
/// The return value for the variant retrieval of the Parser trait
pub enum ParserVariant {
    /// Ethernet protocol parser
    Ethernet(EthernetParser),

    /// Internet Protocol version 4 parser
    Ipv4(Ipv4Parser),

    /// Internet Protocol version 6 parser
    Ipv6(Ipv6Parser),

    /// Transmission Control Protocol parser
    Tcp(TcpParser),

    /// Transport Layer Security parser
    Tls(TlsParser),

    /// User Datagram Protocol parser
    Udp(UdpParser),

    /// Network Time Protocol parser
    Ntp(NtpParser),
}

impl fmt::Display for ParserVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserVariant::Ethernet(_) => write!(f, "Ethernet"),
            ParserVariant::Ipv4(_) => write!(f, "IPv4"),
            ParserVariant::Ipv6(_) => write!(f, "IPv6"),
            ParserVariant::Tcp(_) => write!(f, "TCP"),
            ParserVariant::Tls(_) => write!(f, "TLS"),
            ParserVariant::Udp(_) => write!(f, "UDP"),
            ParserVariant::Ntp(_) => write!(f, "NTP"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Return values for the actual parsers
pub enum Layer {
    /// Ethernet protocol for layer 1
    Ethernet(EthernetPacket),

    /// Internet Protocol version 4 packet variant
    Ipv4(Ipv4Packet),

    /// Internet Protocol version 6 packet variant
    Ipv6(Ipv6Packet),

    /// Transmission Control Protocol packet variant
    Tcp(TcpPacket),

    /// Transport Layer Security packet variant
    Tls(TlsPacket),

    /// User Datagram Protocol packet variant
    Udp(UdpPacket),

    /// Network Time Protocol packet variant
    Ntp(NtpPacket),
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Layer::Ethernet(_) => write!(f, "Ethernet"),
            Layer::Ipv4(_) => write!(f, "IPv4"),
            Layer::Ipv6(_) => write!(f, "IPv6"),
            Layer::Tcp(_) => write!(f, "TCP"),
            Layer::Tls(_) => write!(f, "TLS"),
            Layer::Udp(_) => write!(f, "UDP"),
            Layer::Ntp(_) => write!(f, "NTP"),
        }
    }
}

/// Returns the default `Peel` structure for all available parser variants
pub fn get_packet_peel() -> PacketPeel {
    // Create a tree
    let mut p = Peel::new();

    // Create the parsers
    let eth = p.new_parser(EthernetParser);
    let ipv4 = p.new_parser(Ipv4Parser);
    let ipv6 = p.new_parser(Ipv6Parser);

    let tcp_ipv4 = p.new_parser(TcpParser);
    let tcp_ipv6 = p.new_parser(TcpParser);

    let udp_ipv4 = p.new_parser(UdpParser);
    let udp_ipv6 = p.new_parser(UdpParser);

    let tls_ipv4 = p.new_parser(TlsParser);
    let tls_ipv6 = p.new_parser(TlsParser);

    let ntp_ipv4 = p.new_parser(NtpParser);
    let ntp_ipv6 = p.new_parser(NtpParser);

    // Connect the parsers
    p.link(eth, ipv4);
    p.link(eth, ipv6);

    p.link(ipv4, tcp_ipv4);
    p.link(ipv4, udp_ipv4);

    p.link(tcp_ipv4, tls_ipv4);
    p.link(tcp_ipv6, tls_ipv6);

    p.link(udp_ipv4, ntp_ipv4);
    p.link(udp_ipv6, ntp_ipv6);

    p.link(ipv6, tcp_ipv6);
    p.link(ipv6, udp_ipv6);

    p
}
