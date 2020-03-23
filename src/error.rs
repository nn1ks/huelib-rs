use crate::response;
use std::{fmt, io, net, num};

/// All errors that can occur while interacting with the Philips Hue API.
#[derive(Debug)]
pub enum Error {
    /// Error that can occur when the username cannot be obtained after registering a user.
    GetUsername,
    /// Error that can occur when the identifier of a group cannot be obtained after creating a
    /// group.
    GetGroupId,
    /// Error that can occur when the identifier of a scene cannot be obtained after creating a
    /// scene.
    GetSceneId,
    /// Error that can occur while converting a string to a date.
    ParseDate(chrono::ParseError),
    /// Error that can occur while converting a http response into a string.
    ParseHttpResponse(io::Error),
    /// Error that can occur while converting a string to an IP address.
    ParseIpAddr(net::AddrParseError),
    /// Error that can occur while converting a string to an integer.
    ParseInt(num::ParseIntError),
    /// Error that can occur while parsing json content.
    ParseJson(serde_json::Error),
    /// Error that is returned by the Philips Hue API.
    Response(response::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GetUsername => write!(f, "Failed to get the username"),
            Self::GetGroupId => write!(f, "Failed to get the group identifier"),
            Self::GetSceneId => write!(f, "Failed to get the scene identifier"),
            Self::ParseDate(e) => write!(f, "Failed to parse date: {}", e),
            Self::ParseHttpResponse(e) => write!(f, "Failed to parse http response: {}", e),
            Self::ParseIpAddr(e) => write!(f, "Failed to parse ip address: {}", e),
            Self::ParseInt(e) => write!(f, "Failed to parse integer: {}", e),
            Self::ParseJson(e) => write!(f, "Failed to parse json content: {}", e),
            Self::Response(e) => write!(f, "Error returned from Philips Hue API: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Error::ParseDate(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::ParseHttpResponse(e)
    }
}
impl From<net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::ParseIpAddr(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseJson(e)
    }
}

impl From<response::Error> for Error {
    fn from(e: response::Error) -> Self {
        Error::Response(e)
    }
}
