use serde::{de::Deserializer, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

/// A scene.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    /// Identifier of the scene.
    #[serde(skip_deserializing)]
    pub id: String,
    /// Name of the scene.
    pub name: String,
    /// Type of the scene.
    #[serde(rename = "type")]
    pub kind: Type,
    /// Identifier of the group that the scene is linked to.
    pub group: Option<String>,
    /// Identifier of the lights that are in this scene.
    pub lights: Option<Vec<String>>,
    /// Whitelist user that created or modified the content of the scene. Note that changing name
    /// does not change the owner.
    #[serde(deserialize_with = "deserialize_owner")]
    pub owner: Option<String>,
    /// Indicates whether the scene can be automatically deleted by the bridge.
    pub recycle: bool,
    /// Indicates that the scene is locked by a rule or a schedule and cannot be deleted until all
    /// resources requiring or that reference the scene are deleted.
    pub locked: bool,
    /// App specific data linked to the scene.
    #[serde(rename = "appdata")]
    pub app_data: AppData,
    /// Only available with an individual scene resource. Reserved by the Philips Hue API for
    /// future use.
    pub picture: Option<String>,
    /// Time the scene has been created or updated. Not available for legacy scenes.
    #[serde(rename = "lastupdate")]
    pub last_update: Option<chrono::NaiveDateTime>,
    /// Version of the scene document.
    pub version: Version,
}

fn deserialize_owner<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(value),
    })
}

impl Scene {
    pub(crate) fn with_id(self, id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..self
        }
    }
}

/// Type of a scene.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Type {
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
    pub fn version(self, value: i8) -> Self {
        Self {
            version: Some(value),
            ..self
        }
    }

    /// Sets the data.
    pub fn data(self, value: String) -> Self {
        Self {
            data: Some(value),
            ..self
        }
    }
}

/// Version of a scene document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
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
    name: String,
    lights: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    kind: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_data: Option<AppData>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightstates")]
    light_states: Option<HashMap<String, LightStateModifier>>,
}

impl crate::Creator for Creator {}

impl Creator {
    /// Creates a new creator.
    pub fn new(name: &str, lights: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            lights,
            ..Default::default()
        }
    }

    /// Sets the type of the scene.
    pub fn kind(self, value: Type) -> Self {
        Self {
            kind: Some(value),
            ..self
        }
    }

    /// Sets the app data of the scene.
    pub fn app_data(self, value: AppData) -> Self {
        Self {
            app_data: Some(value),
            ..self
        }
    }

    /// Sets the state of the lights.
    pub fn light_states(self, value: HashMap<String, LightStateModifier>) -> Self {
        Self {
            light_states: Some(value),
            ..self
        }
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
    effect: Option<crate::Effect>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    transition_time: Option<u16>,
}

impl crate::Modifier for LightStateModifier {}

impl LightStateModifier {
    /// Turns the lights on or off.
    pub fn on(self, value: bool) -> Self {
        Self {
            on: Some(value),
            ..self
        }
    }

    /// Sets the brightness of the lights.
    pub fn brightness(self, value: u8) -> Self {
        Self {
            brightness: Some(value),
            ..self
        }
    }

    /// Sets the hue of the lights.
    pub fn hue(self, value: u16) -> Self {
        Self {
            hue: Some(value),
            ..self
        }
    }

    /// Sets the saturation of the lights.
    pub fn saturation(self, value: u8) -> Self {
        Self {
            saturation: Some(value),
            ..self
        }
    }

    /// Sets the color space coordinates of the lights.
    pub fn color_space_coordinates(self, value: (f32, f32)) -> Self {
        Self {
            color_space_coordinates: Some(value),
            ..self
        }
    }

    /// Sets the colot temperature of the lights.
    pub fn color_temperature(self, value: u16) -> Self {
        Self {
            color_temperature: Some(value),
            ..self
        }
    }

    /// Sets the effect of the lights.
    pub fn effect(self, value: crate::Effect) -> Self {
        Self {
            effect: Some(value),
            ..self
        }
    }

    /// Sets the transition time of the lights.
    pub fn transition_time(self, value: u16) -> Self {
        Self {
            transition_time: Some(value),
            ..self
        }
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
    pub fn name(self, value: &str) -> Self {
        Self {
            name: Some(value.to_owned()),
            ..self
        }
    }

    /// Sets the indentifiers of the lights that are in this scene.
    pub fn lights(self, value: Vec<String>) -> Self {
        Self {
            lights: Some(value),
            ..self
        }
    }

    /// Sets the state of the lights in this scene.
    pub fn light_states(self, value: HashMap<String, LightStateModifier>) -> Self {
        Self {
            light_states: Some(value),
            ..self
        }
    }

    /// Sets whether the state of the lights will be overwritten by the current state of the
    /// lights.
    pub fn store_light_state(self, value: bool) -> Self {
        Self {
            store_light_state: Some(value),
            ..self
        }
    }
}
