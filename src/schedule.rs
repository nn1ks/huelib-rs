use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Command to execute when the scheduled event occurs.
    pub command: Command,
    /// Time when the scheduled event will occur.
    #[serde(rename = "localtime")]
    pub local_time: String,
    /// UTC time that the timer was started. Only provided for timers.
    #[serde(rename = "starttime")]
    pub start_time: Option<chrono::NaiveDateTime>,
    /// Status of the schedule.
    pub status: Status,
    /// Whether the schedule will be removed after it expires.
    #[serde(rename = "autodelete")]
    pub auto_delete: Option<bool>,
}

impl Schedule {
    pub(crate) fn with_id<S: Into<String>>(self, id: S) -> Self {
        Self {
            id: id.into(),
            ..self
        }
    }
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

/// Command of a schedule.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Command {
    /// Address where the command will be executed.
    pub address: String,
    /// The HTTP method used to send the body to the given address.
    #[serde(rename = "method")]
    pub request_type: CommandRequestType,
    /// Body of the request that the command sends.
    pub body: HashMap<String, serde_json::Value>,
}

/// Request method of a command.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommandRequestType {
    Put,
    Post,
    Delete,
}

/// Struct for creating a schedule.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recycle: Option<bool>,
}

impl crate::Creator for Creator {}

impl Creator {
    /// Creates a new schedule creator.
    pub fn new(command: Command, localtime: String) -> Self {
        Self {
            command: Some(command),
            localtime: Some(localtime),
            ..Default::default()
        }
    }

    /// Sets the name of the schedule.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn description<S: Into<String>>(self, value: S) -> Self {
        Self {
            description: Some(value.into()),
            ..self
        }
    }

    /// Sets the status of the schedule.
    pub fn status(self, value: Status) -> Self {
        Self {
            status: Some(value),
            ..self
        }
    }

    /// Sets whether the schedule will be removed after it expires.
    pub fn auto_delete(self, value: bool) -> Self {
        Self {
            auto_delete: Some(value),
            ..self
        }
    }
    /// Sets whether resource is automatically deleted when not referenced anymore in any resource
    /// link.
    pub fn recycle(self, value: bool) -> Self {
        Self {
            recycle: Some(value),
            ..self
        }
    }
}

/// Struct for modifying attributes of a schedule.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autodelete")]
    auto_delete: Option<bool>,
}

impl crate::Modifier for Modifier {}

impl Modifier {
    /// Creates a new modifier.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the name of the schedule.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn description<S: Into<String>>(self, value: S) -> Self {
        Self {
            description: Some(value.into()),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn command(self, value: Command) -> Self {
        Self {
            command: Some(value),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn localtime<S: Into<String>>(self, value: S) -> Self {
        Self {
            localtime: Some(value.into()),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn status(self, value: Status) -> Self {
        Self {
            status: Some(value),
            ..self
        }
    }

    /// Sets the description of the schedule.
    pub fn auto_delete(self, value: bool) -> Self {
        Self {
            auto_delete: Some(value),
            ..self
        }
    }
}
