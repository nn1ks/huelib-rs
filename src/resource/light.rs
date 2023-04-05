#![allow(clippy::needless_update)]

use crate::resource::{self, Adjust, Alert, ColorMode, Effect};
use crate::Color;
use derive_setters::Setters;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

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
    #[cfg(not(feature = "old-api"))]
    #[serde(rename = "swupdate")]
    pub software_update: SoftwareUpdate,
    /// Configuration of the light.
    #[cfg(not(feature = "old-api"))]
    pub config: Config,
    /// Capabilities of the light.
    #[cfg(not(feature = "old-api"))]
    pub capabilities: Capabilities,
}

impl Light {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Light {}

/// State of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct SoftwareUpdate {
    /// State of software updates.
    pub state: SoftwareUpdateState,
    /// When the last update was installed.
    #[serde(rename = "lastinstall")]
    pub last_install: Option<chrono::NaiveDateTime>,
}

/// State of a software update.
///
/// See [this issue] for the reason why this enum is marked as `non_exhaustive`.
///
/// [this issue]: https://github.com/yuqio/huelib-rs/issues/1
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftwareUpdateState {
    /// Error
    Error,
    /// Installing
    Installing,
    /// No updates are available.
    NoUpdates,
    /// Device cannot be updated.
    NotUpdatable,
    /// Device is downloading new updates.
    Transferring,
    /// Device is ready to install new updates.
    ReadyToInstall,
    // FIXME: Add missing variants for states (https://github.com/yuqio/huelib-rs/issues/1)
}

/// Configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ColorTemperatureCapabilities {
    /// Minimal color temperature.
    pub min: usize,
    /// Maximal color temperature.
    pub max: usize,
}

/// Streaming capabilities of a light.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StreamingCapabilities {
    /// Whether a renderer is enabled.
    pub renderer: bool,
    /// Whether a proxy is enabled.
    pub proxy: bool,
}

/// Modifier for light attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct AttributeModifier {
    /// Sets the name of the light.
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
        format!("lights/{}", id)
    }
}

/// Static modifier for the light state.
///
/// In comparison to [`StateModifier`], this modifier cannot increment/decrement any attributes or
/// change the alert effect.
///
/// This modifier is used in [`scene::Modifier`] and [`scene::Creator`].
///
/// [`scene::Modifier`]: super::scene::Modifier
/// [`scene::Creator`]: super::scene::Creator
#[derive(Clone, Debug, Default, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StaticStateModifier {
    /// Turns the light on or off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    /// Sets the brightness of the light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri")]
    pub brightness: Option<u8>,
    /// Sets the hue of the light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    /// Sets the saturation of a light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat")]
    pub saturation: Option<u8>,
    /// Sets the color space coordinates of the light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy")]
    pub color_space_coordinates: Option<(f32, f32)>,
    /// Sets the color temperature of a light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct")]
    pub color_temperature: Option<u16>,
    /// Sets the dynamic effect of a light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effect>,
    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    pub transition_time: Option<u16>,
}

impl StaticStateModifier {
    /// Creates a new [`StaticStateModifier`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenient method to set the [`color_space_coordinates`] and [`brightness`] fields.
    ///
    /// [`color_space_coordinates`]: Self::color_space_coordinates
    /// [`brightness`]: Self::brightness
    pub fn with_color(self, value: Color) -> Self {
        let mut modifier = Self {
            color_space_coordinates: Some(value.space_coordinates),
            ..self
        };
        if let Some(brightness) = value.brightness {
            modifier.brightness = Some(brightness);
        }
        modifier
    }
}

impl resource::Modifier for StaticStateModifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("lights/{}/state", id)
    }
}

/// Modifier for the light state.
#[derive(Clone, Debug, Default, PartialEq, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StateModifier {
    /// Turns the light on or off.
    pub on: Option<bool>,
    /// Sets the brightness of the light.
    pub brightness: Option<Adjust<u8>>,
    /// Sets the hue of a light.
    pub hue: Option<Adjust<u16>>,
    /// Sets the saturation of a light.
    pub saturation: Option<Adjust<u8>>,
    /// Sets the color space coordinates of the light.
    pub color_space_coordinates: Option<Adjust<(f32, f32)>>,
    /// Sets the color temperature of a light.
    pub color_temperature: Option<Adjust<u16>>,
    /// Sets the alert effect of a light.
    pub alert: Option<Alert>,
    /// Sets the dynamic effect of a light.
    pub effect: Option<Effect>,
    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    pub transition_time: Option<u16>,
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
        format!("lights/{}/state", id)
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
        }
    }
}

/// Scanner for new lights.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Setters)]
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
        "lights".to_owned()
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
    fn serialize_static_state_modifier() {
        let modifier = StaticStateModifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = StaticStateModifier {
            on: Some(true),
            brightness: Some(1),
            hue: Some(2),
            saturation: Some(3),
            color_space_coordinates: None,
            color_temperature: Some(4),
            effect: Some(Effect::Colorloop),
            transition_time: Some(4),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "on": true,
            "bri": 1,
            "hue": 2,
            "sat": 3,
            "ct": 4,
            "effect": "colorloop",
            "transitiontime": 4,
        });
        assert_eq!(modifier_json, expected_json);

        let modifier = StaticStateModifier::new()
            .with_brightness(1)
            .with_color(Color::from_rgb(0, 0, 0));
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "bri": 0,
            "xy": [0.0, 0.0]
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
