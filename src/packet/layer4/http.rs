//! Hypertext Transfer Protocol related packet processing
use prelude::*;

#[derive(Debug, Clone)]
/// The HTTP parser
pub struct HttpParser;

#[derive(Debug, Eq, PartialEq)]
/// Representation of a Hypertext Transfer Protocol packet
pub struct HttpPacket {
    /// The HTTP request method
    pub request_method: HttpRequestMethod,
}

#[derive(Debug, Eq, PartialEq)]
/// List of supported HTTP request methods
pub enum HttpRequestMethod {
    /// The GET method requests a representation of the specified resource.
    Get,

    /// The POST method requests that the server accept the entity enclosed in the request as a new
    /// subordinate of the web resource identified by the URI.
    Post,
}

impl HttpPacket {
    named!(parse_tcp_based<&[u8], Layer>,
        do_parse!(
            method: alt!(
                map!(tag_fast!("GET "), |_| HttpRequestMethod::Get) |
                map!(tag_fast!("POST "), |_| HttpRequestMethod::Post)
            ) >>

            (Layer::Http(Some(HttpPacket {
                request_method: method,
            })))
        )
    );

    fn parse_tls_based<'a>(input: &'a [u8], result: Option<&Vec<Layer>>) -> IResult<&'a [u8], Layer> {
        expr_opt!(input,
            match result {
                Some(vector) => match vector.last() {
                    Some(&Layer::Tls(_)) => {
                        if let Some(transport_layer) = vector.iter().rev().nth(1) {
                            match transport_layer {
                                &Layer::Tcp(ref data) if (data.source_port == 443 || data.dest_port == 443) => {
                                    Some(Layer::Http(None))
                                }
                                _ => None
                            }
                        } else {
                            None // No transport layer available
                        }
                    },
                    _ => None, // Previous result found, but not correct parent
                },
                _ => None,
            }
        )
    }
}

impl Parser for HttpParser {
    type Result = Layer;
    type Variant = ParserVariant;

    /// Parse an HTTP frame from an u8 slice.
    fn parse<'a>(&self,
                 input: &'a [u8],
                 _: Option<&ParserNode<Layer, ParserVariant>>,
                 _: Option<&ParserArena<Layer, ParserVariant>>,
                 result: Option<&Vec<Layer>>)
                 -> IResult<&'a [u8], Layer> {
        do_parse!(input,

            // Check the transport protocol from the parent parser (TCP or TLS)
            result: alt!(
                // TCP based plain text transfer
                cond_reduce!(match result {
                    Some(vector) => match vector.last() {
                        Some(&Layer::Tcp(_)) => true,
                        _ => false, // Previous result found, but not correct parent
                    },
                    None => true, // Parse also if no result is given, for testability
                }, HttpPacket::parse_tcp_based) |

                // TLS based encrypted traffic
                apply!(HttpPacket::parse_tls_based, result)
            ) >>

            (result)
        )
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::Http(self.clone())
    }
}
