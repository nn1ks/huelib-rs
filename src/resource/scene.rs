use crate::resource::{self, light};
use crate::util;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

/// A scene.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
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

impl Scene {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Scene {}

/// Kind of a scene.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Kind {
    /// Represents a scene with lights.
    LightScene,
    /// Represents a scene which links to a specific group.
    GroupScene,
}

/// Version of a scene document.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct AppData {
    /// App specific version of the data field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i8>,
    /// App specific data. Free format string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// Version of a scene document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize_repr)]
#[repr(i32)]
pub enum Version {
    /// Scene was created with a PUT request.
    Put = 1,
    /// Scene was created with a POST request.
    Post = 2,
}

/// Struct for creating a scene.
#[derive(Clone, Debug, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the scene.
    #[setters(skip)]
    pub name: String,
    /// Sets the light identifiers of the scene.
    #[setters(skip)]
    pub lights: Vec<String>,
    /// Sets the type of the scene.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<Kind>,
    /// Sets the app data of the scene.
    #[serde(skip_serializing_if = "Option::is_none", rename = "appdata")]
    pub app_data: Option<AppData>,
    /// Sets the state of specific lights.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightstates")]
    pub light_states: Option<HashMap<String, light::StaticStateModifier>>,
}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(name: String, lights: Vec<String>) -> Self {
        Self {
            name,
            lights,
            kind: None,
            app_data: None,
            light_states: None,
        }
    }
}

impl resource::Creator for Creator {
    fn url_suffix() -> String {
        "scenes".to_owned()
    }
}

/// Struct for modifying a scene.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the indentifiers of the lights that are in this scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<String>>,
    /// Sets the state of specific lights.
    ///
    /// The keys of the HashMap are the light identifiers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightstates")]
    pub light_states: Option<HashMap<String, light::StaticStateModifier>>,
    /// Sets whether the state of the lights will be overwritten by the current state of the lights.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storelightstate")]
    pub store_light_state: Option<bool>,
}

impl Modifier {
    /// Creates a new [`Modifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Modifier for Modifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("scenes/{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_creator() {
        let creator = Creator::new("test".into(), vec!["1".into()]);
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1"]
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: "test".into(),
            lights: vec!["1".into()],
            kind: Some(Kind::GroupScene),
            app_data: Some(AppData {
                version: Some(2),
                data: Some("data test".into()),
            }),
            light_states: Some(HashMap::new()),
        };
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1"],
            "type": "GroupScene",
            "appdata": {
                "version": 2,
                "data": "data test"
            },
            "lightstates": {}
        });
        assert_eq!(creator_json, expected_json);
    }

    #[test]
    fn serialize_modifier() {
        let modifier = Modifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = Modifier {
            name: Some("test".into()),
            lights: Some(vec!["1".into(), "2".into()]),
            light_states: Some(HashMap::new()),
            store_light_state: Some(true),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1", "2"],
            "lightstates": {},
            "storelightstate": true
        });
        assert_eq!(modifier_json, expected_json);
    }
}
