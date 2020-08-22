use serde::{de, Deserialize};
use serde_json::Value as JsonValue;
use serde_repr::Deserialize_repr;
use std::fmt;
use thiserror::Error as ThisError;

/// A response that is returned from the Philips Hue API.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Response<T> {
    /// The response from the API if the request succeeded.
    Success(T),
    /// The response from the API if the request failed.
    Error(Error),
}

impl<T> Response<T> {
    /// Converts the response into a result.
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Success(v) => write!(f, "Success: {}", v),
            Response::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

/// Errors that can be returned by responses from the Philips Hue API.
///
/// View the [API documentation] for more information.
///
/// [API documentation]: https://developers.meethue.com/develop/hue-api/error-messages
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, ThisError)]
#[error("{description}")]
pub struct Error {
    /// Kind of the error.
    #[serde(rename = "type")]
    pub kind: ErrorKind,
    /// Address where the error occurred.
    pub address: String,
    /// Description of the error.
    pub description: String,
}

/// Kind of an error from a response.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u16)]
pub enum ErrorKind {
    UnauthorizedUser = 1,
    BodyContainsInvalidJson = 2,
    ResourceNotAvailable = 3,
    MethodNotAvailableForResource = 4,
    MissingParametersInBody = 5,
    ParameterNotAvailable = 6,
    InvalidValueForParameter = 7,
    ParameterIsNotModifiable = 8,
    TooManyItemsInList = 11,
    PortalConnectionRequired = 12,
    LinkButtonNotPressed = 101,
    DHCPCannotBeDisabled = 110,
    InvalidUpdateState = 111,
    DeviceIsSetToOff = 201,
    CommissionableLightListIsFull = 203,
    GroupTableIsFull = 301,
    UpdateOrDeleteGroupOfThisTypeNotAllowed = 305,
    LightAlreadyUsedInAnotherRoom = 306,
    SceneCouldNotBeCreatedBufferIsFull = 402,
    SceneCouldNotBeRemoved = 403,
    SceneCouldNotBeCreatedGroupIsEmpty = 404,
    NotAllowedToCreateSensorType = 501,
    SensorListIsFull = 502,
    CommissionableSensorListIsFull = 503,
    RuleEngineFull = 601,
    ConditionError = 607,
    ActionError = 608,
    UnableToActivate = 609,
    ScheduleListIsFull = 701,
    ScheduleTimezoneNotValid = 702,
    ScheduleCannotSetTimeAndLocalTime = 703,
    CannotCreateSchedule = 704,
    CannotEnableScheduleTimeInPast = 705,
    CommandError = 706,
    SourceModelInvalid = 801,
    SourceFactoryNew = 802,
    InvalidState = 803,
    InternalError = 901,
    UnkownError,
}

/// A response type that is used when modifying a resource.
#[derive(Clone, Debug, PartialEq)]
pub struct Modified {
    /// Address of the changed attribute.
    pub address: String,
    /// New value of the attribute.
    pub value: JsonValue,
}

impl fmt::Display for Modified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set '{}' to {}", self.address, self.value)
    }
}

impl<'de> de::Deserialize<'de> for Modified {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ModifiedVisitor;

        impl<'de> de::Visitor<'de> for ModifiedVisitor {
            type Value = Modified;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("struct Modified")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Modified, V::Error> {
                let mut address = None;
                let mut value = None;
                while let Some(key) = map.next_key()? {
                    address = Some(key);
                    value = Some(map.next_value()?);
                }
                let address = address.ok_or_else(|| de::Error::missing_field("address"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(Modified { address, value })
            }
        }

        const FIELDS: &[&str] = &["address", "value"];
        deserializer.deserialize_struct("Modified", FIELDS, ModifiedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Number as JsonNumber};

    #[test]
    fn deserialize_response_success() {
        let json = json!({"success": "test"});
        let response: Response<String> = serde_json::from_value(json).unwrap();
        assert_eq!(response, Response::Success("test".to_owned()));
        let json = json!({"success": 0});
        let response: Response<i32> = serde_json::from_value(json).unwrap();
        assert_eq!(response, Response::Success(0));
    }

    #[test]
    fn deserialize_response_error() {
        let json = json!({
            "error": {
                "type": 1,
                "address": "/address/123",
                "description": "description test",
            }
        });
        let response: Response<String> = serde_json::from_value(json).unwrap();
        let error = Error {
            kind: ErrorKind::UnauthorizedUser,
            address: "/address/123".to_owned(),
            description: "description test".to_owned(),
        };
        assert_eq!(response, Response::Error(error));
    }

    #[test]
    fn deserialize_response_modifier() {
        let json = json!({
            "success": {
                "/light/1": 0.1,
            }
        });
        let response: Response<Modified> = serde_json::from_value(json).unwrap();
        let modified = Modified {
            address: "/light/1".to_owned(),
            value: JsonValue::Number(JsonNumber::from_f64(0.1).unwrap()),
        };
        assert_eq!(response, Response::Success(modified));
    }
}
