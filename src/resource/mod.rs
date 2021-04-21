/// Bindings to the [Capabilities API].
///
/// [Capabilities API]: https://developers.meethue.com/develop/hue-api/10-capabilities-api/
pub mod capabilities;
/// Bindings to the [Configuration API].
///
/// [Configuration API]: https://developers.meethue.com/develop/hue-api/7-configuration-api
pub mod config;
/// Bindings to the [Groups API].
///
/// [Groups API]: https://developers.meethue.com/develop/hue-api/groupds-api
pub mod group;
/// Bindings to the [Lights API].
///
/// [Lights API]: https://developers.meethue.com/develop/hue-api/lights-api
pub mod light;
/// Bindings to the [Resourcelinks API].
///
/// [Resourcelinks API]: https://developers.meethue.com/develop/hue-api/9-resourcelinks-api
pub mod resourcelink;
/// Bindings to the [Rules API].
///
/// [Rules API]: https://developers.meethue.com/develop/hue-api/6-rules-api
pub mod rule;
/// Bindings to the [Scenes API].
///
/// [Scenes API]: https://developers.meethue.com/develop/hue-api/4-scenes
pub mod scene;
/// Bindings to the [Schedules API].
///
/// [Schedules API]: https://developers.meethue.com/develop/hue-api/3-schedules-api
pub mod schedule;
/// Bindings to the [Sensors API].
///
/// [Sensors API]: https://developers.meethue.com/develop/hue-api/5-sensors-api
pub mod sensor;

pub use capabilities::Capabilities;
pub use config::Config;
pub use group::Group;
pub use light::Light;
pub use resourcelink::Resourcelink;
pub use rule::Rule;
pub use scene::Scene;
pub use schedule::Schedule;
pub use sensor::Sensor;

use crate::{response::Modified, Bridge, Error, Response};
use chrono::NaiveDateTime;
use serde::{de, de::Error as _, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

/// Alert effect of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Alert {
    /// Performs one breathe cycle.
    Select,
    /// Performs breathe cycles for 15 seconds or until the alert attribute is set to `None`.
    LSelect,
    /// Disables any alert.
    None,
}

/// Dynamic effect of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    /// Cycles through all hues with the current brightness and saturation.
    Colorloop,
    /// Disables any effect.
    None,
}

/// Color mode of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum ColorMode {
    /// Uses a color temperatue to set the color of a light.
    #[serde(rename = "ct")]
    ColorTemperature,
    /// Uses hue and saturation to set the color of a light.
    #[serde(rename = "hs")]
    HueAndSaturation,
    /// Uses x and y coordinates in the color space to set the color of a light.
    #[serde(rename = "xy")]
    ColorSpaceCoordinates,
}

/// Struct for new resources that were scanned by the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Scan {
    /// When the bridge last scanned for new resources.
    pub last_scan: LastScan,
    /// New resources that were discovered.
    pub resources: Vec<ScanResource>,
}

impl<'de> Deserialize<'de> for Scan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field {
            LastScan,
            ResourceId(String),
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value: String = Deserialize::deserialize(deserializer)?;
                Ok(match value.as_ref() {
                    "lastscan" => Field::LastScan,
                    v => Field::ResourceId(v.to_owned()),
                })
            }
        }

        struct ScanVisitor;

        impl<'de> de::Visitor<'de> for ScanVisitor {
            type Value = Scan;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("struct Scan")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Scan, V::Error> {
                #[derive(Deserialize)]
                struct ResourceInfo {
                    name: String,
                }
                let mut resources = Vec::new();
                let mut last_scan = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LastScan => {
                            last_scan = serde_json::from_value(map.next_value()?)
                                .map_err(V::Error::custom)?
                        }
                        Field::ResourceId(v) => {
                            let info: ResourceInfo = map.next_value()?;
                            let resource = ScanResource {
                                id: v,
                                name: info.name,
                            };
                            resources.push(resource);
                        }
                    }
                }
                let last_scan = last_scan.ok_or_else(|| de::Error::missing_field("lastscan"))?;
                Ok(Scan {
                    resources,
                    last_scan,
                })
            }
        }

        const FIELDS: &[&str] = &["lastscan", "resources"];
        deserializer.deserialize_struct("Scan", FIELDS, ScanVisitor)
    }
}

/// Status of the last scan for a new resource type.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum LastScan {
    /// Date and time of the last scan.
    DateTime(NaiveDateTime),
    /// The bridge is currently scanning.
    Active,
    /// The bridge did not scan since it was powered on.
    None,
}

impl<'de> Deserialize<'de> for LastScan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(match value.as_ref() {
            "active" => LastScan::Active,
            "none" => LastScan::None,
            v => LastScan::DateTime(
                NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S").map_err(D::Error::custom)?,
            ),
        })
    }
}

/// Information about a resource that is returned from a scan.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ScanResource {
    /// Identifier of the resource.
    pub id: String,
    /// Name of the resource.
    pub name: String,
}

/// Enum for adjusting an attribute of a modifier or creator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Adjust<T> {
    /// Overrides the current value.
    Override(T),
    /// Adds the value to the current value.
    Increment(T),
    /// Subtracts the value to the current value.
    Decrement(T),
}

/// Represents a HTTP method.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RequestMethod {
    Put,
    Post,
    Get,
    Delete,
}

/// Marker trait for resources.
pub trait Resource {}

/// Trait for creating a resource.
pub trait Creator: Serialize {
    /// Returns the suffix of the API URL.
    fn url_suffix() -> String;

    /// Sends the request to create the resource.
    fn execute(&self, bridge: &Bridge) -> crate::Result<String> {
        #[derive(Deserialize)]
        struct CreationInfo {
            id: String,
        }
        let mut response: Vec<Response<CreationInfo>> = bridge.api_request(
            Self::url_suffix(),
            RequestMethod::Post,
            Some(serde_json::to_value(self)?),
        )?;
        match response.pop() {
            Some(v) => Ok(v.into_result()?.id),
            None => Err(Error::GetCreatedId),
        }
    }
}

/// Trait for modifying a resource.
pub trait Modifier: Serialize {
    /// The type of the identifier.
    ///
    /// Set to `()` if only one resource of the same type exists.
    type Id;

    /// Returns the suffix of the API URL.
    fn url_suffix(id: Self::Id) -> String;

    /// Sends the request to modify the resource.
    fn execute(&self, bridge: &Bridge, id: Self::Id) -> crate::Result<Vec<Response<Modified>>> {
        bridge.api_request(
            Self::url_suffix(id),
            RequestMethod::Put,
            Some(serde_json::to_value(self)?),
        )
    }
}

/// Trait for scanning new resources.
pub trait Scanner: Serialize {
    /// Returns the suffix of the API URL.
    fn url_suffix() -> String;

    /// Sends the request to scan for new resources.
    fn execute(&self, bridge: &Bridge) -> crate::Result<()> {
        let responses: Vec<Response<JsonValue>> = bridge.api_request(
            Self::url_suffix(),
            RequestMethod::Post,
            Some(serde_json::to_value(self)?),
        )?;
        for response in responses {
            response.into_result()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use serde_json::json;

    #[test]
    fn deserialize_last_scan() {
        let json = json!("none");
        let value: LastScan = serde_json::from_value(json).unwrap();
        assert_eq!(value, LastScan::None);

        let json = json!("active");
        let value: LastScan = serde_json::from_value(json).unwrap();
        assert_eq!(value, LastScan::Active);

        let json = json!("2020-01-01T00:10:00");
        let value: LastScan = serde_json::from_value(json).unwrap();
        let date = NaiveDate::from_ymd(2020, 1, 1);
        let time = NaiveTime::from_hms(0, 10, 0);
        assert_eq!(value, LastScan::DateTime(NaiveDateTime::new(date, time)))
    }

    #[test]
    fn deserialize_scan() {
        let json = json!({
            "lastscan": "active",
            "1": {"name": "light one"},
            "2": {"name": "light two"}
        });
        let value: Scan = serde_json::from_value(json).unwrap();
        let scan = Scan {
            last_scan: LastScan::Active,
            resources: vec![
                ScanResource {
                    id: "1".to_owned(),
                    name: "light one".to_owned(),
                },
                ScanResource {
                    id: "2".to_owned(),
                    name: "light two".to_owned(),
                },
            ],
        };
        assert_eq!(value, scan);
    }
}
