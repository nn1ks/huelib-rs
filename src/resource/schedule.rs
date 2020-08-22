use crate::resource::{self, Action};
use derive_setters::Setters;
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
    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
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
#[derive(Clone, Debug, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the description of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the action of the schedule.
    #[setters(skip)]
    #[serde(rename = "command")]
    pub action: Action,
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

impl resource::Creator for Creator {}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(action: Action, local_time: String) -> Self {
        Self {
            name: None,
            description: None,
            action,
            local_time,
            status: None,
            auto_delete: None,
            recycle: None,
        }
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
    /// Sets the action of the schedule.
    #[serde(skip_serializing_if = "Option::is_none", rename = "command")]
    pub action: Option<Action>,
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

impl resource::Modifier for Modifier {}

impl Modifier {
    /// Creates a new [`Modifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn serialize_creator() {
        let mut action_body = HashMap::new();
        action_body.insert("scene".to_owned(), json!("02b12e930-off-0"));
        let action = Action {
            address: "/api/user/groups/0/action".into(),
            request_type: resource::ActionRequestType::Put,
            body: action_body,
        };

        let creator = Creator::new(action.clone(), "2020-01-01T00:00:00".into());
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "command": {
                "address": "/api/user/groups/0/action",
                "method": "PUT",
                "body": {
                    "scene": "02b12e930-off-0"
                }
            },
            "localtime": "2020-01-01T00:00:00"
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: Some("test".into()),
            description: Some("description test".into()),
            action,
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
                "address": "/api/user/groups/0/action",
                "method": "PUT",
                "body": {
                    "scene": "02b12e930-off-0"
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

        let mut action_body = HashMap::new();
        action_body.insert("scene".to_owned(), json!("02b12e930-off-0"));
        let modifier = Modifier {
            name: Some("test".into()),
            description: Some("description test".into()),
            action: Some(Action {
                address: "/api/user/groups/0/action".into(),
                request_type: resource::ActionRequestType::Put,
                body: action_body,
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
                "address": "/api/user/groups/0/action",
                "method": "PUT",
                "body": {
                    "scene": "02b12e930-off-0"
                }
            },
            "localtime": "2020-01-01T00:00:00",
            "status": "disabled",
            "autodelete": true
        });
        assert_eq!(modifier_json, expected_json);
    }
}
