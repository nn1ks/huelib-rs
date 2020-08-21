use crate::resource::{self, Action};
use crate::util;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

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
    pub last_triggered: Option<chrono::NaiveDateTime>,
    /// How often the rule was triggered.
    #[serde(rename = "timestriggered")]
    pub times_triggered: usize,
    /// When the rule was created.
    pub created: chrono::NaiveDateTime,
    /// Status of the rule.
    pub status: Status,
    /// Conditions of the rule.
    pub conditions: Vec<Condition>,
    /// Actions of the rule.
    pub actions: Vec<Action>,
}

impl resource::Resource for Rule {}

impl Rule {
    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

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

impl resource::Creator for Creator {}

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

impl resource::Modifier for Modifier {}

impl Modifier {
    /// Returns a new [`Modifier`].
    pub fn new() -> Self {
        Self::default()
    }
}
