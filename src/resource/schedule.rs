use crate::resource::{self, Action};
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "command")]
    pub action: Action,
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

impl resource::Resource for Schedule {}

impl Schedule {
    pub(crate) fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
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

/// Struct for creating a schedule.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recycle: Option<bool>,
}

impl resource::Creator for Creator {}

impl Creator {
    /// Creates a new schedule creator.
    pub fn new(action: Action, localtime: String) -> Self {
        Self {
            action: Some(action),
            localtime: Some(localtime),
            ..Default::default()
        }
    }

    /// Sets the name of the schedule.
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets the description of the schedule.
    pub fn description<S: Into<String>>(mut self, value: S) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets the status of the schedule.
    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets whether the schedule will be removed after it expires.
    pub fn auto_delete(mut self, value: bool) -> Self {
        self.auto_delete = Some(value);
        self
    }

    /// Sets whether resource is automatically deleted when not referenced anymore.
    pub fn recycle(mut self, value: bool) -> Self {
        self.recycle = Some(value);
        self
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
    action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autodelete")]
    auto_delete: Option<bool>,
}

impl resource::Modifier for Modifier {}

impl Modifier {
    /// Sets the name of the schedule.
    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets the description of the schedule.
    pub fn description<S: Into<String>>(mut self, value: S) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets the description of the schedule.
    pub fn action(mut self, value: Action) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets the description of the schedule.
    pub fn localtime<S: Into<String>>(mut self, value: S) -> Self {
        self.localtime = Some(value.into());
        self
    }

    /// Sets the description of the schedule.
    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets the description of the schedule.
    pub fn auto_delete(mut self, value: bool) -> Self {
        self.auto_delete = Some(value);
        self
    }
}
