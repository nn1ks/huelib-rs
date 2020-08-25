use crate::{resource, util};
use chrono::NaiveDateTime;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value as JsonValue};

/// A rule for resources on a bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Rule {
    /// Identifier of the rule.
    #[serde(skip)]
    pub id: String,
    /// Name of the rule.
    pub name: String,
    /// Owner of the rule.
    #[serde(deserialize_with = "util::deserialize_option_string")]
    pub owner: Option<String>,
    /// When the rule was last triggered.
    #[serde(
        rename = "lasttriggered",
        deserialize_with = "util::deserialize_option_date_time"
    )]
    pub last_triggered: Option<NaiveDateTime>,
    /// How often the rule was triggered.
    #[serde(rename = "timestriggered")]
    pub times_triggered: usize,
    /// When the rule was created.
    pub created: NaiveDateTime,
    /// Status of the rule.
    pub status: Status,
    /// Conditions of the rule.
    pub conditions: Vec<Condition>,
    /// Actions of the rule.
    pub actions: Vec<Action>,
}

impl Rule {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Rule {}

/// Status of a rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The rule is enabled.
    Enabled,
    /// The rule is disabled.
    Disabled,
    /// The rule was deleted.
    ResourceDeleted,
}

/// Condition of a rule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Condition {
    /// Address of an attribute of a sensor resource.
    pub address: String,
    /// Operator of the condition.
    pub operator: ConditionOperator,
    /// Value of the condition.
    ///
    /// The resource attribute is compared to this value using the given operator. The value is
    /// casted to the data type of the resource attribute. If the cast fails or the operator does
    /// not support the data type the value is casted to the rule is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Condition operator of a rule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ConditionOperator {
    /// Less than an int value.
    #[serde(rename = "lt")]
    LessThan,
    /// Greater than an int value.
    #[serde(rename = "gt")]
    GreaterThan,
    /// Equals an int or bool value.
    #[serde(rename = "eq")]
    Equals,
    /// Triggers when value of button event is changed or change of presence is detected.
    #[serde(rename = "dx")]
    Dx,
    /// Triggers when value of button event is changed or change of presence is detected.
    #[serde(rename = "ddx")]
    Ddx,
    /// An attribute has changed for a given time.
    #[serde(rename = "stable")]
    Stable,
    /// An attribute has not changed for a given time.
    #[serde(rename = "not stable")]
    NotStable,
    /// Current time is in given time interval.
    #[serde(rename = "in")]
    In,
    /// Current time is not in given time interval.
    #[serde(rename = "not in")]
    NotIn,
}

/// Action of a schedule or rule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Action {
    /// Address where the action will be executed.
    pub address: String,
    /// The HTTP method used to send the body to the given address.
    #[serde(rename = "method")]
    pub request_method: ActionRequestMethod,
    /// Body of the request that the action sends.
    pub body: JsonValue,
}

impl Action {
    /// Creates a new action from a [`Creator`].
    ///
    /// [`Creator`]: resource::Creator
    pub fn from_creator<C>(creator: &C) -> Result<Self, JsonError>
    where
        C: resource::Creator,
    {
        Ok(Self {
            address: format!("/{}", C::url_suffix()),
            request_method: ActionRequestMethod::Post,
            body: serde_json::to_value(creator)?,
        })
    }

    /// Creates a new action from a [`Modifier`].
    ///
    /// [`Modifier`]: resource::Modifier
    pub fn from_modifier<M>(modifier: &M, id: M::Id) -> Result<Self, JsonError>
    where
        M: resource::Modifier,
    {
        Ok(Self {
            address: format!("/{}", M::url_suffix(id)),
            request_method: ActionRequestMethod::Put,
            body: serde_json::to_value(modifier)?,
        })
    }

    /// Creates a new action from a [`Scanner`].
    ///
    /// [`Scanner`]: resource::Scanner
    pub fn from_scanner<S>(scanner: &S) -> Result<Self, JsonError>
    where
        S: resource::Scanner,
    {
        Ok(Self {
            address: format!("/{}", S::url_suffix()),
            request_method: ActionRequestMethod::Post,
            body: serde_json::to_value(scanner)?,
        })
    }
}

/// Request method of an action.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionRequestMethod {
    Put,
    Post,
    Delete,
}

/// Struct for creating a rule.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the status of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Sets the conditions of the rule.
    #[setters(skip)]
    pub conditions: Vec<Condition>,
    /// Sets the actions of the rule.
    #[setters(skip)]
    pub actions: Vec<Action>,
}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(conditions: Vec<Condition>, actions: Vec<Action>) -> Self {
        Self {
            name: None,
            status: None,
            conditions,
            actions,
        }
    }
}

impl resource::Creator for Creator {
    fn url_suffix() -> String {
        "rules".to_owned()
    }
}

/// Struct for modifying a rule.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the modifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the status of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Sets the conditions of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Sets the actions of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
}

impl Modifier {
    /// Returns a new [`Modifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl resource::Modifier for Modifier {
    type Id = String;
    fn url_suffix(id: Self::Id) -> String {
        format!("rules/{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_creator() {
        let conditions = vec![Condition {
            address: "/sensors/2/state/lastupdated".into(),
            operator: ConditionOperator::Dx,
            value: None,
        }];
        let actions = vec![Action {
            address: "/lights/1/state".into(),
            request_method: ActionRequestMethod::Put,
            body: json!({}),
        }];

        let creator = Creator::new(conditions.clone(), actions.clone());
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "conditions": [
                {
                    "address": "/sensors/2/state/lastupdated",
                    "operator": "dx"
                }
            ],
            "actions": [
                {
                    "address": "/lights/1/state",
                    "method": "PUT",
                    "body": {}
                }
            ],
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: Some("test".into()),
            status: Some(Status::Enabled),
            conditions,
            actions,
        };
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "status": "enabled",
            "conditions": [
                {
                    "address": "/sensors/2/state/lastupdated",
                    "operator": "dx"
                }
            ],
            "actions": [
                {
                    "address": "/lights/1/state",
                    "method": "PUT",
                    "body": {}
                }
            ],
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
            status: Some(Status::Disabled),
            conditions: Some(vec![]),
            actions: Some(vec![]),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "status": "disabled",
            "conditions": [],
            "actions": []
        });
        assert_eq!(modifier_json, expected_json);
    }
}
