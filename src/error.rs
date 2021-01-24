use crate::response::Error as ResponseError;
use chrono::ParseError as ChronoParseError;
use serde_json::Error as SerdeJsonError;
#[cfg(feature = "upnp-description")]
use serde_xml_rs::Error as SerdeXmlError;
use std::result::Result as StdResult;
use std::{io::Error as IoError, net::AddrParseError};
use thiserror::Error as ThisError;
use ureq::Error as UreqError;

/// Alias for `Result<T, huelib::Error>`.
pub type Result<T> = StdResult<T, Error>;

/// Errors that can occur while interacting with the Philips Hue API.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that can occur when the username cannot be obtained after registering a user.
    #[error("Failed to get username")]
    GetUsername,

    /// Error that can occur when the identifier of a newly created resource cannot be obtained.
    #[error("Failed to get identifier of created resource")]
    GetCreatedId,

    /// Error that can occur while converting a string to a date.
    #[error("Failed to parse date")]
    ParseDate(#[from] ChronoParseError),

    /// Error that can occur while converting a http response into a string.
    #[error("Failed to parse http response")]
    ParseHttpResponse(#[from] IoError),

    /// Error that can occur while converting a string to an IP address.
    #[error("Failed to parse ip address")]
    ParseIpAddr(#[from] AddrParseError),

    /// Error that can occur while parsing json content.
    #[error("Failed to parse json content")]
    ParseJson(#[from] SerdeJsonError),

    /// Error that can occur when sending HTTP requests.
    #[error("Failed to send HTTP request")]
    Request(#[from] Box<UreqError>),

    #[cfg(feature = "upnp-description")]
    /// Error that can occur when deserializing [`Description`].
    ///
    /// [`Description`]: crate::bridge::Description
    #[error("Failed to parse description")]
    ParseDescription(#[from] SerdeXmlError),

    /// Error that is returned by the Philips Hue API.
    #[error("Error returned from Philips Hue API")]
    Response(#[from] ResponseError),
}

impl From<UreqError> for Error {
    fn from(ureq_error: UreqError) -> Self {
        Self::Request(Box::new(ureq_error))
    }
}
