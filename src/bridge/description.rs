use mime::Mime;
use serde::{de::Error, Deserialize, Deserializer};
use std::{net::IpAddr, str::FromStr};
use url::Url;
use uuid::Uuid;

/// Returns the description of the bridge with the given IP address.
///
/// This method internally calls [`Description::get`].
#[cfg_attr(docsrs, doc(cfg(feature = "upnp-description")))]
pub fn description(ip_address: IpAddr) -> crate::Result<Description> {
    Description::get(ip_address)
}

/// Description of a bridge.
#[cfg_attr(docsrs, doc(cfg(feature = "upnp-description")))]
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub spec_version: DescriptionSpecVersion,
    #[serde(rename = "URLBase")]
    pub url_base: Url,
    pub device: DescriptionDevice,
}

impl Description {
    /// Returns the description of the bridge with the given IP address.
    ///
    /// This method sends a HTTP GET request to `http://<bridge_ip_address>/description.xml` to get
    /// the descriptor file.
    pub fn get(ip_address: IpAddr) -> crate::Result<Self> {
        let url = format!("http://{}/description.xml", ip_address);
        let http_response = ureq::get(&url).call()?;
        Ok(serde_xml_rs::from_reader(http_response.into_reader())?)
    }
}

/// Spec version type of a description.
#[cfg_attr(docsrs, doc(cfg(feature = "upnp-description")))]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct DescriptionSpecVersion {
    /// The major version.
    pub major: usize,
    /// The minor version.
    pub minor: usize,
}

/// Device type of a description.
#[cfg_attr(docsrs, doc(cfg(feature = "upnp-description")))]
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionDevice {
    pub device_type: String,
    pub friendly_name: String,
    pub manufacturer: String,
    #[serde(rename = "manufacturerURL")]
    pub manufacturer_url: Url,
    pub model_description: String,
    pub model_name: String,
    pub model_number: String,
    #[serde(rename = "modelURL")]
    pub model_url: Url,
    pub serial_number: String,
    #[serde(rename = "UDN", deserialize_with = "deserialize_uuid")]
    pub udn: Uuid,
    #[serde(rename = "presentationURL")]
    pub presentation_url: String,
    pub icon_list: Vec<DescriptionIcon>,
}

fn deserialize_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let value = value.trim_start_matches("uuid:");
    Uuid::from_str(value).map_err(D::Error::custom)
}

/// Icon type of a description.
#[cfg_attr(docsrs, doc(cfg(feature = "upnp-description")))]
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(from = "deserialize::DescriptionIconWrapper")]
pub struct DescriptionIcon {
    pub mimetype: Mime,
    pub height: usize,
    pub width: usize,
    pub depth: usize,
    pub url: String,
}

impl From<deserialize::DescriptionIconWrapper> for DescriptionIcon {
    fn from(value: deserialize::DescriptionIconWrapper) -> Self {
        Self {
            mimetype: value.icon.mimetype,
            height: value.icon.height,
            width: value.icon.width,
            depth: value.icon.depth,
            url: value.icon.url,
        }
    }
}

mod deserialize {
    use super::*;

    #[derive(Deserialize)]
    pub(super) struct DescriptionIconWrapper {
        pub(super) icon: DescriptionIcon,
    }

    #[derive(Deserialize)]
    pub(super) struct DescriptionIcon {
        #[serde(deserialize_with = "deserialize_mime")]
        pub(super) mimetype: Mime,
        pub(super) height: usize,
        pub(super) width: usize,
        pub(super) depth: usize,
        pub(super) url: String,
    }

    fn deserialize_mime<'de, D>(deserializer: D) -> Result<Mime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Mime::from_str(&value).map_err(D::Error::custom)
    }
}
