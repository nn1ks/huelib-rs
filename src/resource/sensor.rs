#![allow(clippy::needless_update)]

use crate::{resource, util};
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// A sensor.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Sensor {
    /// Identifier of the sensor.
    #[serde(skip)]
    pub id: String,
    /// Name of the sensor.
    pub name: String,
    /// Type name of the sensor.
    #[serde(rename = "type")]
    pub type_name: String,
    /// Model identifier of the sensor.
    #[serde(rename = "modelid")]
    pub model_id: String,
    /// Unique identifier of the sensor.
    #[serde(rename = "uniqueid")]
    pub unique_id: Option<String>,
    /// Manufacturer name of the sensor.
    #[serde(rename = "manufacturername")]
    pub manufacturer_name: Option<String>,
    /// The product name.
    #[serde(rename = "productname")]
    pub product_name: Option<String>,
    /// Some proprietary id as seen on https://www.senic.com/friends-of-hue-smart-switch.
    #[serde(rename = "diversityid")]
    pub diversity_id: Option<String>,
    /// Software version of the sensor.
    #[serde(rename = "swversion")]
    pub software_version: Option<String>,
    /// Current state of the sensor.
    pub state: State,
    /// Configuration of the sensor.
    pub config: Config,
    /// Whether the group is automatically deleted when not referenced anymore.
    pub recycle: Option<bool>,
}

impl Sensor {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Sensor {}

/// Current state of a sensor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct State {
    /// Whether the sensor is present.
    pub presence: Option<bool>,
    /// Flag of the sensor.
    pub flag: Option<bool>,
    /// The current battery state in percent.
    #[serde(
        rename = "lastupdated",
        deserialize_with = "util::deserialize_option_date_time"
    )]
    pub last_updated: Option<chrono::NaiveDateTime>,
    /// Button id that was pressed last.
    #[serde(rename = "buttonevent")]
    pub button_event: Option<u32>,
    /// The temperature in centigrades.
    pub temperature: Option<u32>,
    /// The light level in centiluxes.
    #[serde(rename = "lightlevel")]
    pub light_level: Option<u32>,
    /// Whether it's dark according to the sensor's sensitivity.
    pub dark: Option<bool>,
    /// Whether it's daytime according to the sensor's sensitivity.
    pub daylight: Option<bool>,
    // TODO: Add missing attributes (https://github.com/yuqio/huelib-rs/issues/2)
}

/// Configuration of a sensor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Whether the sensor is on.
    pub on: bool,
    /// Whether the sensor can be reached by the bridge.
    pub reachable: Option<bool>,
    /// The current battery state in percent.
    ///
    /// Only for battery powered devices. Not present when not provided on creation (CLIP sensors).
    pub battery: Option<u8>,
}

/// Modifier for sensor attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct AttributeModifier {
    /// Sets the name of the sensor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AttributeModifier {
    /// Creates a new [`AttributeModifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Modifier for AttributeModifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("sensors/{}", id)
    }
}

/// Modifier for the sensor state.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StateModifier {
    /// Sets the presence of the sensor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<bool>,
}

impl StateModifier {
    /// Creates a new [`StateModifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Modifier for StateModifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("sensors/{}/state", id)
    }
}

/// Modifier for the sensor configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct ConfigModifier {
    /// Sets whether the sensor is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
}

impl ConfigModifier {
    /// Creates a new [`ConfigModifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Modifier for ConfigModifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("sensors/{}/config", id)
    }
}

/// Scanner for new lights.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Scanner {
    /// The device identifiers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceid")]
    pub device_ids: Option<Vec<String>>,
}

impl Scanner {
    /// Creates a new [`Scanner`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Scanner for Scanner {
    fn url_suffix() -> String {
        "sensors".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_attribute_modifier() {
        let modifier = AttributeModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = AttributeModifier {
            name: Some("test".into()),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({"name": "test"});
        assert_eq!(modifier_json, expected_json);
    }

    #[test]
    fn serialize_state_modifier() {
        let modifier = StateModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = StateModifier {
            presence: Some(true),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({"presence": true});
        assert_eq!(modifier_json, expected_json);
    }

    #[test]
    fn serialize_config_modifier() {
        let modifier = ConfigModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = ConfigModifier { on: Some(true) };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({"on": true});
        assert_eq!(modifier_json, expected_json);
    }

    #[test]
    fn serialize_scanner() {
        let scanner = Scanner::new();
        let scanner_json = serde_json::to_value(scanner).unwrap();
        let expected_json = json!({});
        assert_eq!(scanner_json, expected_json);

        let scanner = Scanner {
            device_ids: Some(vec!["1".into()]),
        };
        let scanner_json = serde_json::to_value(scanner).unwrap();
        let expected_json = json!({
            "deviceid": ["1"]
        });
        assert_eq!(scanner_json, expected_json);
    }
}
