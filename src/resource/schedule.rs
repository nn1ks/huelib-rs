use crate::resource;
use chrono::NaiveDateTime;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value as JsonValue};

/// Schedule of a resource.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Schedule {
    /// Identifier of the schedule.
    #[serde(skip)]
    pub id: String,
    /// Name of the schedule.
    pub name: String,
    /// Description of the schedule.
    pub description: String,
    /// Action to execute when the scheduled event occurs.
    pub command: Command,
    /// Time when the scheduled event will occur.
    #[serde(rename = "localtime")]
    pub local_time: String,
    /// UTC time that the timer was started. Only provided for timers.
    #[serde(rename = "starttime")]
    pub start_time: Option<NaiveDateTime>,
    /// Status of the schedule.
    pub status: Status,
    /// Whether the schedule will be removed after it expires.
    #[serde(rename = "autodelete")]
    pub auto_delete: Option<bool>,
}

impl Schedule {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Schedule {}

/// Command of a schedule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Command {
    /// Address where the command will be executed.
    pub address: String,
    /// The HTTP method used to send the body to the given address.
    #[serde(rename = "method")]
    pub request_method: CommandRequestMethod,
    /// Body of the request that the command sends.
    pub body: JsonValue,
}

impl Command {
    /// Creates a new command from a [`Creator`].
    ///
    /// [`Creator`]: resource::Creator
    pub fn from_creator<C, S>(creator: &C, username: S) -> Result<Self, JsonError>
    where
        C: resource::Creator,
        S: AsRef<str>,
    {
        Ok(Self {
            address: format!("/api/{}/{}", username.as_ref(), C::url_suffix()),
            request_method: CommandRequestMethod::Post,
            body: serde_json::to_value(creator)?,
        })
    }

    /// Creates a new command from a [`Modifier`].
    ///
    /// [`Modifier`]: resource::Modifier
    pub fn from_modifier<M, S>(modifier: &M, id: M::Id, username: S) -> Result<Self, JsonError>
    where
        M: resource::Modifier,
        S: AsRef<str>,
    {
        Ok(Self {
            address: format!("/api/{}/{}", username.as_ref(), M::url_suffix(id)),
            request_method: CommandRequestMethod::Put,
            body: serde_json::to_value(modifier)?,
        })
    }

    /// Creates a new command from a [`Scanner`].
    ///
    /// [`Scanner`]: resource::Scanner
    pub fn from_scanner<T, S>(scanner: &T, username: S) -> Result<Self, JsonError>
    where
        T: resource::Scanner,
        S: AsRef<str>,
    {
        Ok(Self {
            address: format!("/api/{}/{}", username.as_ref(), T::url_suffix()),
            request_method: CommandRequestMethod::Post,
            body: serde_json::to_value(scanner)?,
        })
    }
}

/// Request method of an command.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommandRequestMethod {
    Put,
    Post,
    Delete,
}

/// Status of a schedule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The schedule is enabled.
    Enabled,
    /// The schedule is disabled.
    Disabled,
}

/// Struct for creating a schedule.
#[derive(Clone, Debug, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the description of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the command of the schedule.
    #[setters(skip)]
    pub command: Command,
    /// Sets the local time of the schedule.
    #[serde(rename = "localtime")]
    #[setters(skip)]
    pub local_time: String,
    /// Sets the status of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Sets whether the schedule will be removed after it expires.
    #[serde(skip_serializing_if = "Option::is_none", rename = "autodelete")]
    pub auto_delete: Option<bool>,
    /// Sets whether resource is automatically deleted when not referenced anymore.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(command: Command, local_time: String) -> Self {
        Self {
            name: None,
            description: None,
            command,
            local_time,
            status: None,
            auto_delete: None,
            recycle: None,
        }
    }
}

impl resource::Creator for Creator {
    fn url_suffix() -> String {
        "schedules".to_owned()
    }
}

/// Struct for modifying attributes of a schedule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the description of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the command of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    /// Sets the local time of the schedule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "localtime")]
    pub local_time: Option<String>,
    /// Sets the status of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Sets whether the schedule is removed after it expires.
    #[serde(skip_serializing_if = "Option::is_none", rename = "autodelete")]
    pub auto_delete: Option<bool>,
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
        format!("schedules/{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_command() {
        let command = Command {
            address: "/api/user/lights/1/state".into(),
            request_method: CommandRequestMethod::Put,
            body: json!({"on": true}),
        };
        let command_json = serde_json::to_value(command).unwrap();
        let expected_json = json!({
            "address": "/api/user/lights/1/state",
            "method": "PUT",
            "body": {
                "on": true
            }
        });
        assert_eq!(command_json, expected_json);

        let creator = resource::group::Creator::new("test".into(), vec!["1".into()]);
        let command = Command::from_creator(&creator, "user").unwrap();
        let command_json = serde_json::to_value(command).unwrap();
        let expected_json = json!({
            "address": "/api/user/groups",
            "method": "POST",
            "body": {
                "name": "test",
                "lights": ["1"]
            }
        });
        assert_eq!(command_json, expected_json);

        let modifier = resource::light::StateModifier::new().with_on(true);
        let command = Command::from_modifier(&modifier, "1".into(), "user").unwrap();
        let command_json = serde_json::to_value(command).unwrap();
        let expected_json = json!({
            "address": "/api/user/lights/1/state",
            "method": "PUT",
            "body": {
                "on": true
            }
        });
        assert_eq!(command_json, expected_json);

        let scanner = resource::light::Scanner::new();
        let command = Command::from_scanner(&scanner, "user").unwrap();
        let command_json = serde_json::to_value(command).unwrap();
        let expected_json = json!({
            "address": "/api/user/lights",
            "method": "POST",
            "body": {}
        });
        assert_eq!(command_json, expected_json);
    }

    #[test]
    fn serialize_creator() {
        let command = Command {
            address: "/api/user/lights/1/state".into(),
            request_method: CommandRequestMethod::Put,
            body: json!({"on": true}),
        };

        let creator = Creator::new(command.clone(), "2020-01-01T00:00:00".into());
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "command": {
                "address": "/api/user/lights/1/state",
                "method": "PUT",
                "body": {
                    "on": true
                }
            },
            "localtime": "2020-01-01T00:00:00"
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: Some("test".into()),
            description: Some("description test".into()),
            command,
            local_time: "2020-01-01T00:00:00".into(),
            status: Some(Status::Enabled),
            auto_delete: Some(false),
            recycle: Some(true),
        };
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "description": "description test",
            "command": {
                "address": "/api/user/lights/1/state",
                "method": "PUT",
                "body": {
                    "on": true
                }
            },
            "localtime": "2020-01-01T00:00:00",
            "status": "enabled",
            "autodelete": false,
            "recycle": true
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
            description: Some("description test".into()),
            command: Some(Command {
                address: "/api/user/lights/1/state".into(),
                request_method: CommandRequestMethod::Put,
                body: json!({"on": true}),
            }),
            local_time: Some("2020-01-01T00:00:00".into()),
            status: Some(Status::Disabled),
            auto_delete: Some(true),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "description": "description test",
            "command": {
                "address": "/api/user/lights/1/state",
                "method": "PUT",
                "body": {
                    "on": true
                }
            },
            "localtime": "2020-01-01T00:00:00",
            "status": "disabled",
            "autodelete": true
        });
        assert_eq!(modifier_json, expected_json);
    }
}
