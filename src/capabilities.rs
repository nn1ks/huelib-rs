use serde::Deserialize;

/// Capabilities of resources.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Capabilities {
    /// Capabilities of the connected lights.
    pub lights: Info,
    /// Capabilities of groups.
    pub groups: Info,
    /// Capabilities of the connected sensors.
    pub sensors: SensorsInfo,
    /// Capabilities of scenes.
    pub scenes: ScenesInfo,
    /// Capabilities of schedules.
    pub schedules: Info,
    /// Capabilities of rules.
    pub rules: RulesInfo,
    /// Capabilities of resourcelinks.
    pub resourcelinks: Info,
    /// Capabilities of streaming devices.
    pub streaming: StreamingInfo,
    /// Available timezones.
    pub timezones: Timezones,
}

/// Info about the capability of a resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct Info {
    /// Number of currently available resources.
    pub available: usize,
    /// Total number of available resources.
    pub total: usize,
}

/// Info about the capability of sensors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct SensorsInfo {
    /// Number of currently available sensors.
    pub available: usize,
    /// Total number of available sensors.
    pub total: usize,
    /// Capabilities of clip devices.
    pub clip: Info,
    /// Capabilities of zll devices.
    pub zll: Info,
    /// Capabilities of zgp devices.
    pub zgp: Info,
}

/// Info about the capability of scenes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct ScenesInfo {
    /// Number of currently available scenes.
    pub available: usize,
    /// Total number of available scenes.
    pub total: usize,
    /// Capabilities of light states.
    #[serde(rename = "lightstates")]
    pub light_states: Info,
}

/// Info about the capability of rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct RulesInfo {
    /// Number of currently available rules.
    pub available: usize,
    /// Total number of available rules.
    pub total: usize,
    /// Capabilities of conditions.
    pub conditions: Info,
    /// Capabilities of actions.
    pub actions: Info,
}

/// Info about the capability of scenes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct StreamingInfo {
    /// Number of currently available client streams.
    pub available: usize,
    /// Total number of available client streams.
    pub total: usize,
    /// Number of channels per stream.
    pub channels: usize,
}

/// List of timezones.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Timezones {
    values: Vec<String>,
}
