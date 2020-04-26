use crate::{Alert, Color, ColorMode, CoordinateModifierType, Effect, ModifierType};
use serde::{Deserialize, Serialize};

/// A light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Light {
    /// Identifier of the light.
    #[serde(skip)]
    pub id: String,
    /// Name of the light.
    pub name: String,
    /// Type of the light.
    #[serde(rename = "type")]
    pub kind: String,
    /// Current state of the light.
    pub state: State,
    /// The hardware model of the light.
    #[serde(rename = "modelid")]
    pub model_id: String,
    /// Unique ID of the light.
    #[serde(rename = "uniqueid")]
    pub unique_id: String,
    /// Product ID of the light.
    #[serde(rename = "productid")]
    pub product_id: Option<String>,
    /// Product name of the light.
    #[serde(rename = "productname")]
    pub product_name: Option<String>,
    /// Manufacturer name of the light.
    #[serde(rename = "manufacturername")]
    pub manufacturer_name: Option<String>,
    /// The software version running on the light.
    #[serde(rename = "swversion")]
    pub software_version: String,
    /// Information about software updates of the light.
    #[serde(rename = "swupdate")]
    pub software_update: SoftwareUpdate,
    /// Configuration of the light.
    pub config: Config,
    /// Capabilities of the light.
    pub capabilities: Capabilities,
}

impl crate::Resource for Light {}

impl Light {
    pub(crate) fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }
}

/// State of a light.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct State {
    /// Whether the light is on.
    pub on: Option<bool>,
    /// Brightness of the light.
    ///
    /// The maximum brightness is 254 and 1 is the minimum brightness.
    #[serde(rename = "bri")]
    pub brightness: Option<u8>,
    /// Hue of the light.
    ///
    /// Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: Option<u16>,
    /// Saturation of the light.
    ///
    /// The most saturated (colored) is 254 and 0 is the least saturated (white).
    #[serde(rename = "sat")]
    pub saturation: Option<u8>,
    /// X and y coordinates of a color in CIE color space. Both values must be between 0 and 1.
    #[serde(rename = "xy")]
    pub color_space_coordinates: Option<(f32, f32)>,
    /// Mired color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<u16>,
    /// Alert effect of the light.
    pub alert: Option<Alert>,
    /// Dynamic effect of the light.
    pub effect: Option<Effect>,
    /// Color mode of the light.
    #[serde(rename = "colormode")]
    pub color_mode: Option<ColorMode>,
    /// Whether the light can be reached by the bridge.
    pub reachable: bool,
}

/// Information about software updates of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct SoftwareUpdate {
    /// State of software updates.
    pub state: SoftwareUpdateState,
    /// When the last update was installed.
    #[serde(rename = "lastinstall")]
    pub last_install: Option<chrono::NaiveDateTime>,
}

/// State of a software update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftwareUpdateState {
    /// No updates are available.
    NoUpdates,
    /// Device cannot be updated.
    NotUpdatable,
    // TODO: Add missing variants for states (missing due to incomplete documentation)
}

/// Configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Arche type of the light.
    #[serde(rename = "archetype")]
    pub arche_type: String,
    /// Function of the light.
    pub function: String,
    /// Direction of the light.
    pub direction: String,
    /// Startup configuration of the light.
    pub startup: Option<StartupConfig>,
}

/// Startup configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct StartupConfig {
    /// Mode of the startup.
    pub mode: String,
    /// Whether startup is configured for the light.
    pub configured: bool,
}

/// Capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Capabilities {
    /// Whether the light is certified.
    pub certified: bool,
    /// Control capabilities of the light.
    pub control: ControlCapabilities,
    /// Streaming capabilities of the light.
    pub streaming: StreamingCapabilities,
}

/// Control capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ControlCapabilities {
    /// Minimal dimlevel of the light.
    #[serde(rename = "mindimlevel")]
    pub min_dimlevel: Option<usize>,
    /// Maximal lumen of the light.
    #[serde(rename = "maxlumen")]
    pub max_lumen: Option<usize>,
    /// Color gamut of the light.
    #[serde(rename = "colorgamut")]
    pub color_gamut: Option<Vec<(f32, f32)>>,
    /// Type of the color gamut of the light.
    #[serde(rename = "colorgamuttype")]
    pub color_gamut_type: Option<String>,
    /// Maximal/minimal color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<ColorTemperatureCapabilities>,
}

/// Color temperature capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct ColorTemperatureCapabilities {
    /// Minimal color temperature.
    pub min: usize,
    /// Maximal color temperature.
    pub max: usize,
}

/// Streaming capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct StreamingCapabilities {
    /// Whether a renderer is enabled.
    pub renderer: bool,
    /// Whether a proxy is enabled.
    pub proxy: bool,
}

/// Modifier for light attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct AttributeModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl crate::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Changes the name of the light.
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }
}

/// Modifier for the light state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
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
}

impl crate::Modifier for StateModifier {}

impl StateModifier {
    /// Turns the light on or off.
    pub fn on(mut self, value: bool) -> Self {
        self.on = Some(value);
        self
    }

    /// Sets the brightness of the light.
    pub fn brightness(mut self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => self.brightness = Some(value),
            ModifierType::Increment => self.brightness_increment = Some(value as i16),
            ModifierType::Decrement => self.brightness_increment = Some(-(value as i16)),
        };
        self
    }

    /// Sets the hue of a light.
    pub fn hue(mut self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => self.hue = Some(value),
            ModifierType::Increment => self.hue_increment = Some(value as i32),
            ModifierType::Decrement => self.hue_increment = Some(-(value as i32)),
        };
        self
    }

    /// Sets the saturation of a light.
    pub fn saturation(mut self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => self.saturation = Some(value),
            ModifierType::Increment => self.saturation_increment = Some(value as i16),
            ModifierType::Decrement => self.saturation_increment = Some(-(value as i16)),
        };
        self
    }

    /// Sets the x and y coordinates in the color space to set the color of a light.
    ///
    /// If the modifier type is `Override`, the values must be between 0 and 1. If the modifier
    /// type is not `Override`, the values must be between 0 and 0.5.
    pub fn color_space_coordinates(
        mut self,
        modifier_type: CoordinateModifierType,
        value: (f32, f32),
    ) -> Self {
        match modifier_type {
            CoordinateModifierType::Override => self.color_space_coordinates = Some(value),
            CoordinateModifierType::Increment => {
                self.color_space_coordinates_increment = Some(value)
            }
            CoordinateModifierType::Decrement => {
                self.color_space_coordinates_increment = Some((-value.0, -value.1))
            }
            CoordinateModifierType::IncrementDecrement => {
                self.color_space_coordinates_increment = Some((value.0, -value.1))
            }
            CoordinateModifierType::DecrementIncrement => {
                self.color_space_coordinates_increment = Some((-value.0, value.1))
            }
        };
        self
    }

    /// Sets the color (and brightness) of a light.
    pub fn color(mut self, value: Color) -> Self {
        self.color_space_coordinates = Some(value.space_coordinates);
        if let Some(v) = value.brightness {
            self.brightness = Some(v);
        }
        self
    }

    /// Sets the color temperature of a light.
    pub fn color_temperature(mut self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => self.color_temperature = Some(value),
            ModifierType::Increment => self.color_temperature_increment = Some(value as i32),
            ModifierType::Decrement => self.color_temperature_increment = Some(-(value as i32)),
        };
        self
    }

    /// Sets the alert effect of a light.
    pub fn alert(mut self, value: Alert) -> Self {
        self.alert = Some(value);
        self
    }

    /// Sets the dynamic effect of a light.
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
}
