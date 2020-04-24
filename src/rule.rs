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
    #[serde(deserialize_with = "crate::util::deserialize_option_string")]
    pub owner: Option<String>,
    /// When the rule was last triggered.
    #[serde(
        rename = "lasttriggered",
        deserialize_with = "crate::util::deserialize_option_date_time"
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
    pub actions: Vec<crate::Action>,
}

impl crate::Resource for Rule {}

impl Rule {
    pub(crate) fn with_id<S: Into<String>>(self, id: S) -> Self {
        Self {
            id: id.into(),
            ..self
        }
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<crate::Action>>,
}

impl crate::Creator for Creator {}

impl Creator {
    /// Creates a new rule creator.
    pub fn new(conditions: Vec<Condition>, actions: Vec<crate::Action>) -> Self {
        Self {
            conditions: Some(conditions),
            actions: Some(actions),
            ..Default::default()
        }
    }

    /// Sets the name of the rule.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
            ..self
        }
    }

    /// Sets the status of the rule.
    pub fn status(self, value: Status) -> Self {
        Self {
            status: Some(value),
            ..self
        }
    }
}

/// Struct for modifying a rule.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<crate::Action>>,
}

impl crate::Modifier for Modifier {}

impl Modifier {
    /// Sets the name of the rule.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
            ..self
        }
    }

    /// Sets the status of the rule.
    pub fn status(self, value: Status) -> Self {
        Self {
            status: Some(value),
            ..self
        }
    }

    /// Sets the conditions of the rule.
    pub fn conditions(self, value: Vec<Condition>) -> Self {
        Self {
            conditions: Some(value),
            ..self
        }
    }

    /// Sets the actions of the rule.
    pub fn actions(self, value: Vec<crate::Action>) -> Self {
        Self {
            actions: Some(value),
            ..self
        }
    }
}
