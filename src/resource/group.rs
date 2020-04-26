use crate::resource::{self, Alert, Effect, ModifierType};
use crate::Color;
use serde::{Deserialize, Serialize};

/// A group of lights.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
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

impl resource::Resource for Group {}

impl Group {
    pub(crate) fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }
}

/// Kind of a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Kind {
    /// Kind of a group that can be manually created.
    Creatable(CreatableKind),
    /// Kind of a group that is automatically created by the bridge and cannot be manually created.
    Immutable(ImmutableKind),
}

/// Kind of a group that can be manually created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
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
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Class {
    Attic,
    Balcony,
    Barbecue,
    Bathroom,
    Bedroom,
    Carport,
    Closet,
    Computer,
    Dining,
    Downstairs,
    Driveway,
    #[serde(rename = "Front door")]
    FrontDoor,
    Garage,
    Garden,
    #[serde(rename = "Guest room")]
    GuestRoom,
    Gym,
    Hallway,
    Home,
    #[serde(rename = "Kids bedroom")]
    KidsBedroom,
    Kitchen,
    #[serde(rename = "Laundry room")]
    LaundryRoom,
    #[serde(rename = "Living room")]
    LivingRoom,
    Lounge,
    #[serde(rename = "Man cave")]
    ManCave,
    Music,
    Nursery,
    Office,
    Other,
    Pool,
    Porch,
    Reading,
    Recreation,
    Staircase,
    Storage,
    Studio,
    TV,
    Terrace,
    Toilet,
    #[serde(rename = "Top floor")]
    TopFloor,
    Upstairs,
}

/// State of a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct State {
    /// Whether any light in a group is on.
    pub any_on: bool,
    /// Whether all lights in a group are on.
    pub all_on: bool,
}

/// Struct for creating a group.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    kind: Option<CreatableKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Class>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recycle: Option<bool>,
}

impl resource::Creator for Creator {}

impl Creator {
    /// Creates a new group creator.
    pub fn new<S: Into<String>, V: Into<String>>(name: S, lights: Vec<V>) -> Self {
        Self {
            name: Some(name.into()),
            lights: Some(lights.into_iter().map(|v| v.into()).collect()),
            ..Default::default()
        }
    }

    /// Sets the identifiers of the sensors of the group.
    pub fn sensors<S: Into<String>>(mut self, value: Vec<S>) -> Self {
        self.sensors = Some(value.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Sets the kind of the group.
    pub fn kind(mut self, value: CreatableKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Sets the class of the group.
    pub fn class(mut self, value: Class) -> Self {
        self.class = Some(value);
        self
    }

    /// Sets whether the group is automatically deleted when not referenced anymore.
    pub fn recycle(mut self, value: bool) -> Self {
        self.recycle = Some(value);
        self
    }
}

/// Struct for modifying group attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct AttributeModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Class>,
}

impl resource::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Sets the name of the group.
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets the identifiers of the lights of the group.
    pub fn lights<S: Into<String>>(mut self, value: Vec<S>) -> Self {
        self.lights = Some(value.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Sets the identifiers of the sensors of the group.
    pub fn sensors<S: Into<String>>(mut self, value: Vec<S>) -> Self {
        self.sensors = Some(value.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Sets the class of the group.
    pub fn class(mut self, value: Class) -> Self {
        self.class = Some(value);
        self
    }
}

/// Struct for modifying the group state.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct StateModifier {
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
    alert: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    transition_time: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri_inc")]
    brightness_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hue_inc")]
    hue_increment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat_inc")]
    saturation_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy_inc")]
    color_space_coordinates_increment: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct_inc")]
    color_temperature_increment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<String>,
}

impl resource::Modifier for StateModifier {}

impl StateModifier {
    /// Turns the lights on or off.
    pub fn on(mut self, value: bool) -> Self {
        self.on = Some(value);
        self
    }

    /// Sets the brightness of the lights.
    pub fn brightness(mut self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => self.brightness = Some(value),
            ModifierType::Increment => self.brightness_increment = Some(value as i16),
            ModifierType::Decrement => self.brightness_increment = Some(-(value as i16)),
        };
        self
    }

    /// Sets the hue of the lights.
    pub fn hue(mut self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => self.hue = Some(value),
            ModifierType::Increment => self.hue_increment = Some(value as i32),
            ModifierType::Decrement => self.hue_increment = Some(-(value as i32)),
        };
        self
    }

    /// Sets the saturation of the lights.
    pub fn saturation(mut self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => self.saturation = Some(value),
            ModifierType::Increment => self.saturation_increment = Some(value as i16),
            ModifierType::Decrement => self.saturation_increment = Some(-(value as i16)),
        };
        self
    }

    /// Sets the color (and brightness) of the lights.
    pub fn color(mut self, value: Color) -> Self {
        self.color_space_coordinates = Some(value.space_coordinates);
        if let Some(v) = value.brightness {
            self.brightness = Some(v);
        }
        self
    }

    /// Sets the color temperature of the lights.
    pub fn color_temperature(mut self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => self.color_temperature = Some(value),
            ModifierType::Increment => self.color_temperature_increment = Some(value as i32),
            ModifierType::Decrement => self.color_temperature_increment = Some(-(value as i32)),
        };
        self
    }

    /// Sets the alert effect of the lights.
    pub fn alert(mut self, value: Alert) -> Self {
        self.alert = Some(value);
        self
    }

    /// Sets the dynamic effect of the lights.
    pub fn effect(mut self, value: Effect) -> Self {
        self.effect = Some(value);
        self
    }

    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    pub fn transition_time(mut self, value: u16) -> Self {
        self.transition_time = Some(value);
        self
    }

    /// Sets the scene identifier of the group.
    pub fn scene<S: Into<String>>(mut self, value: S) -> Self {
        self.scene = Some(value.into());
        self
    }
}
