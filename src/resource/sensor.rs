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
    /// Software version of the sensor.
    #[serde(rename = "swversion")]
    pub software_verion: String,
    /// Current state of the sensor.
    pub state: State,
    /// Configuration of the sensor.
    pub config: Config,
    /// Whether the group is automatically deleted when not referenced anymore.
    pub recycle: Option<bool>,
}

impl resource::Resource for Sensor {}

impl Sensor {
    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
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

impl resource::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Creates a new [`AttributeModifier`].
    pub fn new() -> Self {
        Self::default()
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

impl resource::Modifier for StateModifier {}

impl StateModifier {
    /// Creates a new [`StateModifier`].
    pub fn new() -> Self {
        Self::default()
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

impl resource::Modifier for ConfigModifier {}

impl ConfigModifier {
    /// Creates a new [`ConfigModifier`].
    pub fn new() -> Self {
        Self::default()
    }
}
