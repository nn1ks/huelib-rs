use crate::{util, Effect};
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

/// A scene.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Scene {
    /// Identifier of the scene.
    #[serde(skip_deserializing)]
    pub id: String,
    /// Name of the scene.
    pub name: String,
    /// Kind of the scene.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Identifier of the group that the scene is linked to.
    pub group: Option<String>,
    /// Identifier of the lights that are in this scene.
    pub lights: Option<Vec<String>>,
    /// Whitelist user that created or modified the content of the scene.
    #[serde(deserialize_with = "util::deserialize_option_string")]
    pub owner: Option<String>,
    /// Whether the group is automatically deleted when not referenced anymore.
    pub recycle: bool,
    /// Whether the scene is locked by a rule or a schedule.
    ///
    /// If set to true, the scene cannot be deleted until all resources requiring or that reference
    /// the scene are deleted.
    pub locked: bool,
    /// App specific data linked to the scene.
    #[serde(rename = "appdata")]
    pub app_data: AppData,
    /// Only available with an individual scene resource.
    ///
    /// Reserved by the Philips Hue API for future use.
    pub picture: Option<String>,
    /// Time the scene has been created or updated.
    ///
    /// Not available for legacy scenes.
    #[serde(rename = "lastupdate")]
    pub last_update: Option<chrono::NaiveDateTime>,
    /// Version of the scene document.
    pub version: Version,
}

impl crate::Resource for Scene {}

impl Scene {
    pub(crate) fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }
}

/// Kind of a scene.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    /// Represents a scene with lights.
    LightScene,
    /// Represents a scene which links to a specific group.
    GroupScene,
}

/// Version of a scene document.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct AppData {
    /// App specific version of the data field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i8>,
    /// App specific data. Free format string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl crate::Modifier for AppData {}

impl AppData {
    /// Sets the version.
    pub fn version(mut self, value: i8) -> Self {
        self.version = Some(value);
        self
    }

    /// Sets the data.
    pub fn data<S: Into<String>>(mut self, value: S) -> Self {
        self.data = Some(value.into());
        self
    }
}

/// Version of a scene document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(i32)]
pub enum Version {
    /// Scene was created with a PUT request.
    Put = 1,
    /// Scene was created with a POST request.
    Post = 2,
}

/// Struct for creating a scene.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_data: Option<AppData>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightstates")]
    light_states: Option<HashMap<String, LightStateModifier>>,
}

impl crate::Creator for Creator {}

impl Creator {
    /// Creates a new scene creator.
    pub fn new<S: Into<String>, V: Into<String>>(name: S, lights: Vec<V>) -> Self {
        Self {
            name: Some(name.into()),
            lights: Some(lights.into_iter().map(|v| v.into()).collect()),
            ..Default::default()
        }
    }

    /// Sets the type of the scene.
    pub fn kind(mut self, value: Kind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Sets the data of the app data.
    pub fn app_data<S: Into<String>>(mut self, value: S) -> Self {
        self.app_data = Some(AppData {
            data: Some(value.into()),
            version: self.app_data.unwrap_or_default().version,
        });
        self
    }

    /// Sets the version of the app data.
    pub fn app_version(mut self, value: i8) -> Self {
        self.app_data = Some(AppData {
            data: self.app_data.unwrap_or_default().data,
            version: Some(value),
        });
        self
    }

    /// Sets the state of a light.
    pub fn light_state<S: Into<String>>(mut self, id: S, modifier: LightStateModifier) -> Self {
        let mut light_states = self.light_states.unwrap_or_default();
        light_states.insert(id.into(), modifier);
        self.light_states = Some(light_states);
        self
    }
}

/// Struct for modifying the state of a light.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct LightStateModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat")]
    saturation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy")]
    color_space_coordinates: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct")]
    color_temperature: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    transition_time: Option<u16>,
}

impl crate::Modifier for LightStateModifier {}

impl LightStateModifier {
    /// Turns the lights on or off.
    pub fn on(mut self, value: bool) -> Self {
        self.on = Some(value);
        self
    }

    /// Sets the brightness of the lights.
    pub fn brightness(mut self, value: u8) -> Self {
        self.brightness = Some(value);
        self
    }

    /// Sets the hue of the lights.
    pub fn hue(mut self, value: u16) -> Self {
        self.hue = Some(value);
        self
    }

    /// Sets the saturation of the lights.
    pub fn saturation(mut self, value: u8) -> Self {
        self.saturation = Some(value);
        self
    }

    /// Sets the color space coordinates of the lights.
    pub fn color_space_coordinates(mut self, value: (f32, f32)) -> Self {
        self.color_space_coordinates = Some(value);
        self
    }

    /// Sets the colot temperature of the lights.
    pub fn color_temperature(mut self, value: u16) -> Self {
        self.color_temperature = Some(value);
        self
    }

    /// Sets the effect of the lights.
    pub fn effect(mut self, value: Effect) -> Self {
        self.effect = Some(value);
        self
    }

    /// Sets the transition time of the lights.
    pub fn transition_time(mut self, value: u16) -> Self {
        self.transition_time = Some(value);
        self
    }
}

/// Struct for modifying a scene.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightstates")]
    light_states: Option<HashMap<String, LightStateModifier>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "storelightstate")]
    store_light_state: Option<bool>,
}

impl crate::Modifier for Modifier {}

impl Modifier {
    /// Sets the name of the scene.
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets the indentifiers of the lights that are in this scene.
    pub fn lights<S: Into<String>>(mut self, value: Vec<S>) -> Self {
        self.lights = Some(value.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Sets the state of a light.
    pub fn light_state<S: Into<String>>(mut self, id: S, modifier: LightStateModifier) -> Self {
        let mut light_states = self.light_states.unwrap_or_default();
        light_states.insert(id.into(), modifier);
        self.light_states = Some(light_states);
        self
    }

    /// Sets whether the state of the lights will be overwritten by the current state of the
    /// lights.
    pub fn store_light_state(mut self, value: bool) -> Self {
        self.store_light_state = Some(value);
        self
    }
}
