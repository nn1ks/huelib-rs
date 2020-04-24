#![warn(missing_docs)]

//! Rust bindings for the [Philips Hue API].
//!
//! ## About
//!
//! This library sends HTTP requests to the bridge using the [ureq] crate. The responses/requests
//! are deserialized/serialized using the [serde], [serde_json] and [serde_repr] crates.
//!
//! [Philips Hue API]: https://developers.meethue.com/develop/hue-api
//! [ureq]: https://github.com/algesten/ureq
//! [serde]: https://github.com/serde-rs/serde
//! [serde_json]: https://github.com/serde-rs/json
//! [serde_repr]: https://github.com/dtolnay/serde-repr
//!
//! ## Examples
//!
//! Modifies the state of a light on a specific bridge:
//!
//! ```rust,no_run
//! use huelib::{light, Bridge, Modifier, ModifierType};
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! // Create a bridge with IP address and username.
//! let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), "username");
//!
//! // Create a state modifier that increments the brightness by 40 and sets the saturation to 200.
//! let modifier = light::StateModifier::new()
//!     .brightness(ModifierType::Increment, 40)
//!     .saturation(ModifierType::Override, 200);
//!
//! // Set attributes of the light with index '1' from the modifier and print the responses.
//! match bridge.set_light_state("1", &modifier) {
//!     Ok(v) => v.iter().for_each(|response| println!("{}", response)),
//!     Err(e) => eprintln!("Failed to modify the light state: {}", e),
//! };
//! ```
//!
//! Creates a group and registers a user on a discovered bridge:
//!
//! ```rust,no_run
//! use huelib::{bridge, group, Bridge};
//!
//! // Get the IP address of the bridge that was first discovered in the local network.
//! let ip_address = bridge::discover()
//!     .expect("Failed to discover bridges")
//!     .pop()
//!     .expect("No bridges found in the local network");
//!
//! // Register a user on the discovered bridge.
//! let user = bridge::register_user(ip_address, "huelib-rs example", false)
//!     .expect("Failed to register user");
//!
//! // Create a bridge with IP address and username.
//! let bridge = Bridge::new(ip_address, user.name);
//!
//! // Create a group creator that sets the name to 'group1', adds the lights with the index '1'
//! // and '2' to the group and sets the class to 'Office'.
//! let creator = group::Creator::new("group", ["1", "2"].to_vec()).class(group::Class::Office);
//!
//! // Create the group and print the identifier of the new group.
//! match bridge.create_group(&creator) {
//!     Ok(v) => println!("Created group with id '{}'", v),
//!     Err(e) => eprintln!("Failed to create group: {}", e),
//! };
//! ```

/// Module for managing bridges.
pub mod bridge;
/// Bindings to the [Capabilities API].
///
/// [Capabilities API]: https://developers.meethue.com/develop/hue-api/10-capabilities-api/
pub mod capabilities;
/// Bindings to the [Configuration API].
///
/// [Configuration API]: https://developers.meethue.com/develop/hue-api/7-configuration-api
pub mod config;
/// Errors that can occur while interacting with the Philips Hue API.
pub mod error;
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
/// Responses returned from the Philips Hue API.
pub mod response;
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

pub use bridge::Bridge;
pub use capabilities::Capabilities;
pub use config::Config;
pub use error::Error;
pub use group::Group;
pub use light::Light;
pub use resourcelink::Resourcelink;
pub use response::Response;
pub use rule::Rule;
pub use scene::Scene;
pub use schedule::Schedule;
pub use sensor::Sensor;

mod util;

use serde::{de, de::Error as DeError, Deserialize, Serialize};
use std::fmt;

/// Alert effect of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    /// Cycles through all hues with the current brightness and saturation.
    Colorloop,
    /// Disables any effect.
    None,
}

/// Color mode of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
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

/// Action of a schedule or rule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Action {
    /// Address where the action will be executed.
    pub address: String,
    /// The HTTP method used to send the body to the given address.
    #[serde(rename = "method")]
    pub request_type: ActionRequestType,
    /// Body of the request that the action sends.
    pub body: std::collections::HashMap<String, serde_json::Value>,
}

/// Request type of an action.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionRequestType {
    Put,
    Post,
    Delete,
}

/// Struct for new resources that were scanned by the bridge.
#[derive(Clone, Debug, PartialEq)]
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

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct Scan")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Scan, V::Error> {
                let mut resources = Vec::new();
                let mut last_scan = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LastScan => {
                            last_scan = serde_json::from_value(map.next_value()?)
                                .map_err(V::Error::custom)?
                        }
                        Field::ResourceId(v) => {
                            let resource = ScanResource {
                                id: v,
                                name: map.next_value()?,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LastScan {
    /// Date and time of the last scan.
    DateTime(chrono::NaiveDateTime),
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
                chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S")
                    .map_err(D::Error::custom)?,
            ),
        })
    }
}

/// Information about a resource that is returned from a scan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanResource {
    /// Identifier of the resource.
    pub id: String,
    /// Name of the resource.
    pub name: String,
}

/// Type of a modifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModifierType {
    /// Override the current value with the given value.
    Override,
    /// Add the given value to the current value.
    Increment,
    /// Subtract the given value to the current value.
    Decrement,
}

/// Type of a modifier for coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateModifierType {
    /// Override both current values with the given values.
    Override,
    /// Add the given values to the current values.
    Increment,
    /// Subtract the given values to the current values.
    Decrement,
    /// Add the given value for the first coordinate to the current value of the first coordinate
    /// and subtract the given value for the second coordinate to the current value of the second
    /// coordinate.
    IncrementDecrement,
    /// Subtract the given value for the first coordinate to the current value of the first
    /// coordinate and add the given value for the second coordinate to the current value of the
    /// second coordinate.
    DecrementIncrement,
}

/// Trait for resources.
pub trait Resource {}

/// Trait for modifiers.
pub trait Modifier: Default + PartialEq {
    /// Creates a new modifier.
    fn new() -> Self {
        Default::default()
    }

    /// Whether the modifier will not modify anything.
    fn is_empty(&self) -> bool {
        self == &Default::default()
    }
}

/// Trait for creators.
pub trait Creator {}
