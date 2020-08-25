use crate::resource;
use derive_setters::Setters;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

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
    /// Whether the resource is automatically deleted when not referenced anymore.
    pub recycle: bool,
    /// References to resources which are used by this resourcelink.
    pub links: Vec<Link>,
}

impl Resourcelink {
    pub(crate) fn with_id(self, id: String) -> Self {
        Self { id, ..self }
    }
}

impl resource::Resource for Resourcelink {}

/// Kind of a resourcelink.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("/{}/{}", self.kind.to_str(), self.id))
    }
}

/// Kind of a link.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    fn to_str(&self) -> &str {
        match self {
            Self::Group => "groups",
            Self::Light => "lights",
            Self::Resourcelink => "resourcelinks",
            Self::Rule => "rules",
            Self::Scene => "scenes",
            Self::Schedule => "schedules",
            Self::Sensor => "sensors",
        }
    }
}

/// Struct for creating a resourcelink.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the resourcelink.
    #[setters(skip)]
    pub name: String,
    /// Sets the description of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the owner of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Sets the kind of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<Kind>,
    /// Sets the class id of the resourcelink.
    #[serde(rename = "classid")]
    #[setters(skip)]
    pub class_id: u16,
    /// Sets the whether to recycle the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
    /// Sets the links of the resourcelink.
    #[setters(skip)]
    pub links: Vec<Link>,
}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(name: String, class_id: u16, links: Vec<Link>) -> Self {
        Self {
            name,
            description: None,
            owner: None,
            kind: None,
            class_id,
            recycle: None,
            links,
        }
    }
}

impl resource::Creator for Creator {
    fn url_suffix() -> String {
        "resourcelinks".to_owned()
    }
}

/// Modifier for a resourcelink.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the description of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the class id of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<Kind>,
    /// Sets the kind of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none", rename = "classid")]
    pub class_id: Option<u16>,
    /// Sets the links of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
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
        format!("resourcelinks/{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize_creator() {
        let links = vec![
            Link {
                kind: LinkKind::Sensor,
                id: "1".into(),
            },
            Link {
                kind: LinkKind::Schedule,
                id: "2".into(),
            },
        ];

        let creator = Creator::new("test".into(), 1, links.clone());
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "classid": 1,
            "links": ["/sensors/1", "/schedules/2"],
        });
        assert_eq!(creator_json, expected_json);

        let creator = Creator {
            name: "test".into(),
            description: Some("description test".into()),
            owner: Some("owner test".into()),
            kind: Some(Kind::Link),
            class_id: 1,
            recycle: Some(true),
            links,
        };
        let creator_json = serde_json::to_value(creator).unwrap();
        let expected_json = json!({
            "name": "test",
            "description": "description test",
            "owner": "owner test",
            "type": "Link",
            "classid": 1,
            "recycle": true,
            "links": ["/sensors/1", "/schedules/2"]
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
            kind: Some(Kind::Link),
            class_id: Some(1),
            links: Some(vec![
                Link {
                    kind: LinkKind::Group,
                    id: "1".into(),
                },
                Link {
                    kind: LinkKind::Scene,
                    id: "2".into(),
                },
            ]),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "description": "description test",
            "type": "Link",
            "classid": 1,
            "links": ["/groups/1", "/scenes/2"]
        });
        assert_eq!(modifier_json, expected_json);
    }
}
