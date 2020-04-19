use crate::response;
use std::{io, net, num};

/// All errors that can occur while interacting with the Philips Hue API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error that can occur when the username cannot be obtained after registering a user.
    #[error("Failed to get username")]
    GetUsername,
    /// Error that can occur when the identifier of a newly created resource cannot be obtained.
    #[error("Failed to get identifier of created resource")]
    GetCreatedId,
    #[error("Failed to parse date: {0}")]
    ParseDate(#[from] chrono::ParseError),
    /// Error that can occur while converting a http response into a string.
    #[error("Failed to parse http response: {0}")]
    ParseHttpResponse(#[from] io::Error),
    /// Error that can occur while converting a string to an IP address.
    #[error("Failed to parse ip address: {0}")]
    ParseIpAddr(#[from] net::AddrParseError),
    /// Error that can occur while converting a string to an integer.
    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] num::ParseIntError),
    /// Error that can occur while parsing json content.
    #[error("Failed to parse json content: {0}")]
    ParseJson(#[from] serde_json::Error),
    /// Error that is returned by the Philips Hue API.
    #[error("Error returned from Philips Hue API: {0}")]
    Response(#[from] response::Error),
}
