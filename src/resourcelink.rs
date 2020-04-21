use serde::{de, de::Error, Deserialize, Serialize};
use std::fmt;

/// A resourcelink to group resources in the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Resourcelink {
    /// Identifier of the resourcelink.
    #[serde(skip)]
    pub id: String,
    /// Name of the resourcelink.
    pub name: String,
    /// Description of the resourcelink.
    pub description: String,
    /// Owner of the resourcelink.
    pub owner: String,
    /// Kind of the resourcelink.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Class identifier of the resourcelink.
    #[serde(rename = "classid")]
    pub class_id: u16,
    /// Whether the resource is automatically deleted when not referenced anymore in any
    /// resourcelink
    pub recycle: bool,
    /// References to resources which are used by this resourcelink.
    pub links: Vec<Link>,
}

impl Resourcelink {
    pub(crate) fn with_id<S: Into<String>>(self, id: S) -> Self {
        Self {
            id: id.into(),
            ..self
        }
    }
}

/// Kind of a resourcelink.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    /// The only variant.
    Link,
}

/// A reference to a resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    /// Kind of the resource.
    pub kind: LinkKind,
    /// Identifier of the resource.
    pub id: String,
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        let mut values: Vec<&str> = value.split('/').collect();
        let id_str = values
            .pop()
            .ok_or_else(|| D::Error::custom("expected link in the format /<kind>/<id>"))?;
        let kind_str = values
            .pop()
            .ok_or_else(|| D::Error::custom("expected link in the format /<kind>/<id>"))?;
        Ok(Self {
            kind: LinkKind::from_str(kind_str)
                .ok_or_else(|| D::Error::custom(format!("invalid link type '{}'", kind_str)))?,
            id: id_str.to_owned(),
        })
    }
}

/// Kind of a link.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LinkKind {
    Group,
    Light,
    Resourcelink,
    Rule,
    Scene,
    Schedule,
    Sensor,
}

impl LinkKind {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "groups" => Some(Self::Group),
            "lights" => Some(Self::Light),
            "resourcelinks" => Some(Self::Resourcelink),
            "rules" => Some(Self::Rule),
            "scenes" => Some(Self::Scene),
            "schedules" => Some(Self::Schedule),
            "sensors" => Some(Self::Sensor),
            _ => None,
        }
    }
}

impl fmt::Display for LinkKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Group => "groups",
                Self::Light => "lights",
                Self::Resourcelink => "resourcelinks",
                Self::Rule => "rules",
                Self::Scene => "scenes",
                Self::Schedule => "schedules",
                Self::Sensor => "sensors",
            }
        )
    }
}

/// Struct for creating a resourcelink.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_id: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recycle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<String>>,
}

impl crate::Creator for Creator {}

impl Creator {
    /// Creates a new resourcelink creator.
    pub fn new<S: Into<String>>(name: S, class_id: u16) -> Self {
        Self {
            name: Some(name.into()),
            class_id: Some(class_id),
            links: Some(Vec::new()),
            ..Default::default()
        }
    }

    /// Sets the description of the resourcelink.
    pub fn description<S: Into<String>>(self, value: S) -> Self {
        Self {
            description: Some(value.into()),
            ..self
        }
    }

    /// Sets the owner of the resourcelink.
    pub fn owner<S: Into<String>>(self, value: S) -> Self {
        Self {
            owner: Some(value.into()),
            ..self
        }
    }

    /// Sets the kind of the resourcelink.
    pub fn kind(self, value: Kind) -> Self {
        Self {
            kind: Some(value),
            ..self
        }
    }

    /// Sets the whether to recycle the resourcelink.
    pub fn recycle(self, value: bool) -> Self {
        Self {
            recycle: Some(value),
            ..self
        }
    }

    /// Adds a link to the resourcelink.
    pub fn link<S: AsRef<str>>(self, kind: LinkKind, id: S) -> Self {
        let mut links = self.links.unwrap_or_default();
        links.push(format!("/{}/{}", kind, id.as_ref()));
        Self {
            links: Some(links),
            ..self
        }
    }
}

/// Modifier for a resourcelink.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_id: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<String>>,
}

impl crate::Modifier for Modifier {}

impl Modifier {
    /// Sets the name of the resourcelink.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
            ..self
        }
    }

    /// Sets the description of the resourcelink.
    pub fn description<S: Into<String>>(self, value: S) -> Self {
        Self {
            description: Some(value.into()),
            ..self
        }
    }

    /// Sets the class id of the resourcelink.
    pub fn class_id(self, value: u16) -> Self {
        Self {
            class_id: Some(value),
            ..self
        }
    }

    /// Sets the kind of the resourcelink.
    pub fn kind(self, value: Kind) -> Self {
        Self {
            kind: Some(value),
            ..self
        }
    }

    /// Sets a link of the resourcelink.
    pub fn link<S: AsRef<str>>(self, kind: LinkKind, id: S) -> Self {
        let mut links = self.links.unwrap_or_default();
        links.push(format!("/{}/{}", kind, id.as_ref()));
        Self {
            links: Some(links),
            ..self
        }
    }
}
