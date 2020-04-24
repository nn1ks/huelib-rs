use crate::util;
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
    /// Software version of the sensor.
    #[serde(rename = "swversion")]
    pub software_verion: String,
    /// Current state of the sensor.
    pub state: State,
    /// Configuration of the sensor.
    pub config: Config,
    /// Indicates whether the sensor can be automatically deleted by the bridge.
    pub recycle: Option<bool>,
}

impl crate::Resource for Sensor {}

impl Sensor {
    pub(crate) fn with_id<S: Into<String>>(self, id: S) -> Self {
        Self {
            id: id.into(),
            ..self
        }
    }
}

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
    // TODO: Add missing attributes (missing due to incomplete documentation)
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct AttributeModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl crate::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Changes the name of the sensor.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
        }
    }
}

/// Modifier for the sensor state.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct StateModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    presence: Option<bool>,
}

impl crate::Modifier for StateModifier {}

impl StateModifier {
    /// Sets the presence of the sensor.
    pub fn presence(self, value: bool) -> Self {
        Self {
            presence: Some(value),
        }
    }
}

/// Modifier for the sensor configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct ConfigModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
}

impl crate::Modifier for ConfigModifier {}

impl ConfigModifier {
    /// Sets whether the sensor is on.
    pub fn on(self, value: bool) -> Self {
        Self { on: Some(value) }
    }
}
