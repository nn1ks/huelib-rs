use crate::resource::{self, Adjust, Alert, Effect};
use crate::Color;
use derive_setters::Setters;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

/// A group of lights.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct Group {
    /// Identifier of the group.
    #[serde(skip)]
    pub id: String,
    /// Name of the group.
    pub name: String,
    /// Identifiers of lights that are in this group.
    pub lights: Vec<String>,
    /// Identifiers of sensors that are in this group.
    pub sensors: Vec<String>,
    /// Kind of the group.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Class of the group.
    ///
    /// Only used if [`kind`] is [`Room`].
    ///
    /// [`kind`]: #structfield.kind
    /// [`Room`]: enum.CreatableKind.html#variant.Room
    pub class: Option<Class>,
    /// State of the group.
    pub state: Option<State>,
    /// Model identifier of the group.
    ///
    /// Only present for automatically created luminaires.
    #[serde(rename = "modelid")]
    pub model_id: Option<String>,
    /// Unique identifier of the group.
    ///
    /// In AA:BB:CC:DD format for luminaire groups or AA:BB:CC:DD-XX format for
    /// lightsource groups, where XX is the lightsource position.
    #[serde(rename = "unique_id")]
    pub unique_id: Option<String>,
    /// Whether the group is automatically deleted when not referenced anymore.
    pub recycle: Option<bool>,
}

impl Group {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Group {}

/// Kind of a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(untagged)]
pub enum Kind {
    /// Kind of a group that can be manually created.
    Creatable(CreatableKind),
    /// Kind of a group that is automatically created by the bridge and cannot be manually created.
    Immutable(ImmutableKind),
}

/// Kind of a group that can be manually created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum CreatableKind {
    /// A group of lights that can be controlled together.
    ///
    /// This the default group type that the bridge generates for user created groups. Default type
    /// when no type is given on creation.
    LightGroup,
    /// A group of lights that are physically located in the same place.
    ///
    /// Rooms behave similar as light groups, except: (1) A room can be empty and contain 0 lights,
    /// (2) a light is only allowed in one room and (3) a room is not automatically deleted when
    /// all lights in that room are deleted.
    Room,
    /// A group of lights that are used in an entertainment setup.
    ///
    /// Entertainment group behave in a similar way as light groups, with the exception: it can be
    /// empty and contain 0 lights. The group is also not automatically recycled when lights are
    /// deleted. The group of lights can be controlled together as in LightGroup.
    Entertainment,
    /// Zones describe a group of lights that can be controlled together.
    ///
    /// Zones can be empty and contain 0 lights. A light is allowed to be in multiple zones.
    Zone,
}

/// Kind of a group that is automatically created by the bridge and cannot be manually created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum ImmutableKind {
    /// A special group containing all lights in the system.
    ///
    /// This group is not returned by the `get_all_groups` function, and cannot be created,
    /// modified or deleted.
    Zero,
    /// A lighting installation of default groupings of hue lights.
    ///
    /// The bridge will pre-install these groups for ease of use. This type cannot be created
    /// manually. Also, a light can only be in a maximum of one luminaire group.
    Luminaire,
    /// A group of lights based on multisource luminaire attributes.
    ///
    /// This group is created by the bridge.
    #[serde(rename = "Lightsource")]
    LightSource,
}

/// Class of a group.
pub type Class = String;

/// State of a group.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct State {
    /// Whether any light in a group is on.
    pub any_on: bool,
    /// Whether all lights in a group are on.
    pub all_on: bool,
}

/// Struct for creating a group.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the group.
    #[setters(skip)]
    pub name: String,
    /// Sets the light identifiers of the group.
    #[setters(skip)]
    pub lights: Vec<String>,
    /// Sets the sensor identifiers of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Vec<String>>,
    /// Sets the kind of the group.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<CreatableKind>,
    /// Sets the class of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Class>,
    /// Sets whether the group is automatically deleted when not referenced anymore.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(name: String, lights: Vec<String>) -> Self {
        Self {
            name,
            lights,
            sensors: None,
            kind: None,
            class: None,
            recycle: None,
        }
    }
}

impl resource::Creator for Creator {
    fn url_suffix() -> String {
        "groups".to_owned()
    }
}

/// Struct for modifying group attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct AttributeModifier {
    /// Sets the name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the identifiers of the lights of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<Vec<String>>,
    /// Sets the identifiers of the sensors of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Vec<String>>,
    /// Sets the class of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Class>,
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
        format!("groups/{}", id)
    }
}

/// Struct for modifying the group state.
#[derive(Clone, Debug, Default, PartialEq, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StateModifier {
    /// Turns the lights on or off.
    pub on: Option<bool>,
    /// Sets the brightness of the lights.
    pub brightness: Option<Adjust<u8>>,
    /// Sets the hue of the lights.
    pub hue: Option<Adjust<u16>>,
    /// Sets the saturation of the lights.
    pub saturation: Option<Adjust<u8>>,
    /// Sets the color space coordinates of the lights.
    pub color_space_coordinates: Option<Adjust<(f32, f32)>>,
    /// Sets the color temperature of the lights.
    pub color_temperature: Option<Adjust<u16>>,
    /// Sets the alert effect of the lights.
    pub alert: Option<Alert>,
    /// Sets the dynamic effect of the lights.
    pub effect: Option<Effect>,
    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    pub transition_time: Option<u16>,
    /// Sets the scene identifier of the group.
    pub scene: Option<String>,
}

impl StateModifier {
    /// Creates a new [`StateModifier`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenient method to set the [`color_space_coordinates`] and [`brightness`] fields.
    ///
    /// [`color_space_coordinates`]: Self::color_space_coordinates
    /// [`brightness`]: Self::brightness
    pub fn with_color(self, value: Color) -> Self {
        let mut modifier = Self {
            color_space_coordinates: Some(Adjust::Override(value.space_coordinates)),
            ..self
        };
        if let Some(brightness) = value.brightness {
            modifier.brightness = Some(Adjust::Override(brightness));
        }
        modifier
    }
}

impl resource::Modifier for StateModifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("groups/{}/action", id)
    }
}

impl Serialize for StateModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serialize! {
            serializer, "StateModifier";
            on => (&self.on),
            bri => (&self.brightness, to_override),
            bri_inc => (&self.brightness, to_increment, i16),
            hue => (&self.hue, to_override),
            hue_inc => (&self.hue, to_increment, i32),
            sat => (&self.saturation, to_override),
            sat_inc => (&self.saturation, to_increment, i16),
            xy => (&self.color_space_coordinates, to_override),
            xy_inc => (&self.color_space_coordinates, to_increment_tuple, f32),
            ct => (&self.color_temperature, to_override),
            ct_inc => (&self.color_temperature, to_increment, i32),
            alert => (&self.alert),
            effect => (&self.effect),
            transitiontime => (&self.transition_time),
            scene => (&self.scene),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_creator() {
        let creator = Creator::new("test".into(), vec!["1".into(), "2".into()]);
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1", "2"],
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: "test".into(),
            lights: vec!["1".into(), "2".into()],
            sensors: Some(vec!["3".into()]),
            kind: Some(CreatableKind::Room),
            class: Some("Office".to_string()),
            recycle: Some(true),
        };
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1", "2"],
            "sensors": ["3"],
            "type": "Room",
            "class": "Office",
            "recycle": true
        });
        assert_eq!(creator_json, expected_json);
    }

    #[test]
    fn serialize_attribute_modifier() {
        let modifier = AttributeModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = AttributeModifier {
            name: Some("test".into()),
            lights: Some(vec!["1".into(), "2".into()]),
            sensors: Some(vec!["3".into()]),
            class: Some("Office".to_string()),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "lights": ["1", "2"],
            "sensors": ["3"],
            "class": "Office"
        });
        assert_eq!(modifier_json, expected_json);
    }

    #[test]
    fn serialize_state_modifier() {
        let modifier = StateModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = StateModifier {
            on: Some(true),
            brightness: Some(Adjust::Increment(1)),
            hue: Some(Adjust::Override(2)),
            saturation: Some(Adjust::Decrement(3)),
            color_space_coordinates: None,
            color_temperature: Some(Adjust::Override(4)),
            alert: Some(Alert::None),
            effect: Some(Effect::Colorloop),
            transition_time: Some(4),
            scene: Some("1".into()),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "on": true,
            "bri_inc": 1,
            "hue": 2,
            "sat_inc": -3,
            "ct": 4,
            "alert": "none",
            "effect": "colorloop",
            "transitiontime": 4,
            "scene": "1"
        });
        assert_eq!(modifier_json, expected_json);

        let modifier = StateModifier::new()
            .with_brightness(Adjust::Increment(1))
            .with_color(Color::from_rgb(0, 0, 0));
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "bri": 0,
            "xy": [0.0, 0.0]
        });
        assert_eq!(modifier_json, expected_json);
    }
}
