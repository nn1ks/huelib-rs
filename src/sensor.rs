use serde::{de, de::Error, Deserialize, Serialize};
use std::fmt;

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
    #[serde(rename = "lastupdated", deserialize_with = "deserialize_last_updated")]
    pub last_updated: Option<chrono::NaiveDateTime>,
    // TODO: Add missing attributes (missing due to incomplete documentation)
}

fn deserialize_last_updated<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDateTime>, D::Error> {
    use std::str::FromStr;
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(chrono::NaiveDateTime::from_str(&value).map_err(D::Error::custom)?),
    })
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

/// Struct for new sensors that were scanned by the bridge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scan {
    /// When the bridge last scanned for new sensors.
    pub last_scan: LastScan,
    /// New sensors that were discovered.
    pub sensors: Vec<ScanSensor>,
}

impl<'de> Deserialize<'de> for Scan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field {
            LastScan,
            SensorId(String),
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value: String = Deserialize::deserialize(deserializer)?;
                Ok(match value.as_ref() {
                    "lastscan" => Field::LastScan,
                    v => Field::SensorId(v.to_owned()),
                })
            }
        }

        struct ScanVisitor;

        impl<'de> de::Visitor<'de> for ScanVisitor {
            type Value = Scan;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct Scan")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Scan, V::Error> {
                let mut sensors = Vec::new();
                let mut last_scan = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LastScan => {
                            last_scan = serde_json::from_value(map.next_value()?)
                                .map_err(V::Error::custom)?
                        }
                        Field::SensorId(v) => {
                            let sensor = ScanSensor {
                                id: v,
                                name: map.next_value()?,
                            };
                            sensors.push(sensor);
                        }
                    }
                }
                let last_scan = last_scan.ok_or_else(|| de::Error::missing_field("lastscan"))?;
                Ok(Scan { sensors, last_scan })
            }
        }

        const FIELDS: &[&str] = &["lastscan", "sensors"];
        deserializer.deserialize_struct("Scan", FIELDS, ScanVisitor)
    }
}

/// Status of the last scan for new sensors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LastScan {
    /// Date and time of the last scan.
    DateTime(chrono::NaiveDateTime),
    /// The bridge is currently scanning.
    Active,
    /// The bridge did not scan since it was powered on.
    None,
}

impl<'de> Deserialize<'de> for LastScan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(match value.as_ref() {
            "active" => LastScan::Active,
            "none" => LastScan::None,
            v => LastScan::DateTime(
                chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S")
                    .map_err(D::Error::custom)?,
            ),
        })
    }
}

/// Information about a sensor that is returned from a scan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanSensor {
    /// Identifier of the sensor.
    pub id: String,
    /// Name of the sensor.
    pub name: String,
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
