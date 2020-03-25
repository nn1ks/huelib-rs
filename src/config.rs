use serde::{de::Deserializer, de::Error, Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use std::net::IpAddr;

/// Configuration for a bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Name of the bridge.
    pub name: String,
    /// Information about software updates.
    #[serde(rename = "swupdate2")]
    pub software_update: SoftwareUpdate,
    /// Software version of the bridge.
    #[serde(rename = "swversion")]
    pub software_version: String,
    /// The version of the Philips Hue API.
    #[serde(rename = "apiversion")]
    pub api_version: String,
    /// Indicates whether the link button has been pressed within the last 30 seconds.
    #[serde(rename = "linkbutton")]
    pub link_button: bool,
    /// IP address of the bridge.
    #[serde(rename = "ipaddress")]
    pub ip_address: IpAddr,
    /// MAC address of the bridge.
    #[serde(rename = "mac")]
    pub mac_address: String,
    /// Network mask of the bridge.
    pub netmask: String,
    /// Gateway IP address of the bridge.
    pub gateway: IpAddr,
    /// Whether the IP address of the bridge is obtained with DHCP.
    pub dhcp: bool,
    /// Whether the bridge is registered to synchronize data with a portal account.
    #[serde(rename = "portalservices")]
    pub portal_services: bool,
    /// Status of the portal connection.
    #[serde(rename = "portalconnection")]
    pub portal_connection: ServiceStatus,
    /// Portal state of the bridge.
    #[serde(rename = "portalstate")]
    pub portal_state: PortalState,
    /// Internet services of the bridge.
    #[serde(rename = "internetservices")]
    pub internet_services: InternetServices,
    /// Current time stored on the bridge.
    #[serde(rename = "UTC")]
    pub current_time: chrono::NaiveDateTime,
    /// Local time of the bridge.
    #[serde(rename = "localtime")]
    #[serde(deserialize_with = "deserialize_local_time")]
    pub local_time: Option<chrono::NaiveDateTime>,
    /// Timezone of the bridge as OlsenIDs, like "Europe/Amsterdam".
    #[serde(deserialize_with = "deserialize_timezone")]
    pub timezone: Option<String>,
    /// The current wireless frequency channel used by the bridge. It can take values of 11, 15,
    /// 20, 25 or 0 if undefined (factory new).
    #[serde(rename = "zigbeechannel")]
    pub zigbee_channel: u8,
    /// Uniquely identifies the hardware model of the bridge.
    #[serde(rename = "modelid")]
    pub model_id: String,
    /// The unique bridge id.
    #[serde(rename = "bridgeid")]
    pub bridge_id: String,
    #[serde(rename = "factorynew")]
    /// Indicates if bridge settings are factory new.
    pub factory_new: bool,
    #[serde(rename = "replacesbridgeid")]
    /// If a bridge backup file has been restored on this bridge from a bridge with a different
    /// bridge id, it will indicate that bridge id, otherwise it will be None.
    pub replaces_bridge_id: Option<bool>,
    /// The version of the datastore.
    #[serde(rename = "datastoreversion")]
    pub datastore_version: String,
    /// Name of the starterkit created in the factory.
    #[serde(rename = "starterkitid")]
    pub starterkit_id: String,
    /// Backup information about the bridge.
    pub backup: Backup,
    /// Whitelisted users.
    #[serde(deserialize_with = "deserialize_whitelist")]
    pub whitelist: Vec<User>,
}

fn deserialize_timezone<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(value),
    })
}

fn deserialize_local_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDateTime>, D::Error> {
    use std::str::FromStr;
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.as_ref() {
        "none" => None,
        _ => Some(chrono::NaiveDateTime::from_str(&value).map_err(D::Error::custom)?),
    })
}

fn deserialize_whitelist<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<User>, D::Error> {
    let map: std::collections::HashMap<String, User> = Deserialize::deserialize(deserializer)?;
    let mut users = Vec::new();
    for (id, user) in map {
        users.push(user.with_id(&id));
    }
    Ok(users)
}

/// Information about software updates.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct SoftwareUpdate {
    /// State of software updates.
    pub state: SoftwareUpdateState,
    /// Triggers checking for software updates.
    #[serde(rename = "checkforupdate")]
    pub check: bool,
    /// Configuration for automatically updating.
    #[serde(rename = "autoinstall")]
    pub auto_install: SoftwareUpdateAutoInstall,
    /// Time of last change in system configuration.
    #[serde(rename = "lastchange")]
    pub last_change: Option<chrono::NaiveDateTime>,
    /// Time of last software update.
    #[serde(rename = "lastinstall")]
    pub last_install: Option<chrono::NaiveDateTime>,
}

/// State of software updates.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftwareUpdateState {
    /// System does not know if new updates are available.
    Unkown,
    /// No updates are available.
    NoUpdates,
    /// Updates are being transferred to the devices.
    Transferring,
    /// At least one software update can be installed.
    AnyReadyToInstall,
    /// All software updates can be installed.
    AllReadyToInstall,
    /// System update is installing.
    Installing,
}

/// Configuration for automatically updating.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct SoftwareUpdateAutoInstall {
    /// Whether automatic updates are activated.
    pub on: bool,
    /// The time when updates are installed.
    #[serde(rename = "updatetime", deserialize_with = "deserialize_update_time")]
    pub update_time: Option<chrono::NaiveTime>,
}

fn deserialize_update_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<chrono::NaiveTime>, D::Error> {
    use std::str::FromStr;
    let mut value: String = Deserialize::deserialize(deserializer)?;
    Ok(match value.remove(0) {
        'T' => Some(chrono::NaiveTime::from_str(&value).map_err(D::Error::custom)?),
        _ => None,
    })
}

/// Portal state of the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct PortalState {
    /// Signedon.
    pub signedon: bool,
    /// Incoming communication.
    pub incoming: bool,
    /// Outgoing communication.
    pub outgoing: bool,
    /// Status of communication.
    pub communication: ServiceStatus,
}

/// Internet services of the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct InternetServices {
    /// Whether the bridge is connected to the internet.
    pub internet: ServiceStatus,
    /// Whether remote CLIP is available.
    #[serde(rename = "remoteaccess")]
    pub remote_access: ServiceStatus,
    /// Whether the time was synchronized with internet service in the last 48 hours.
    pub time: ServiceStatus,
    /// Whether the software update server was reachable in the last 24 hours.
    #[serde(rename = "swupdate")]
    pub software_update: ServiceStatus,
}

/// Status of a service.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ServiceStatus {
    /// The serivce is connected.
    Connected,
    /// The serivce is not connected.
    Disconnected,
}

/// Backup information about the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Backup {
    /// Status of backup/restore.
    pub status: BackupStatus,
    /// Specifies the last error source if the backup has detected an internal error. Cleared at
    /// the start of a backup import or export.
    #[serde(rename = "errorcode")]
    pub error: BackupError,
}

/// Status of backup/restore.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum BackupStatus {
    /// No backup/restore ongoing.
    #[serde(rename = "idle")]
    Idle,
    /// Can be written if status is `Idle`. Creates a backup file which can be retrieved via the
    /// backup interface and puts the bridge in `FilereadyDisabled` status. ndicates that a backup
    /// file for migration is being created. CLIP is not available for some time after this
    /// command.
    #[serde(rename = "startmigration")]
    StartMigration,
    /// Indicates that a backup file is available and that this bridge has been disabled due to a
    /// migration procedure. The bridge can be activated again by a factory reset or power cycle.
    #[serde(rename = "fileready_disabled")]
    FilereadyDisabled,
    /// Indicates that the a backup file has been sent to the bridge and the bridge is in the
    /// process of preparing it for restoring.
    #[serde(rename = "prepare_restore")]
    PrepareRestore,
    /// Indicates that the bridge is in the process of restoring the backup file.
    #[serde(rename = "restoring")]
    Restoring,
}

/// Backup error of the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum BackupError {
    /// The backup has not detected an internal error.
    None = 0,
    /// Failed to export a backup.
    ExportFailed = 1,
    /// Failed to import a backup.
    ImportFailed = 2,
}

/// User of a bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct User {
    /// Identifier of the user.
    #[serde(skip)]
    pub id: String,
    /// Name of the user.
    pub name: String,
    /// Date of the last use of the user.
    #[serde(rename = "last use date")]
    pub last_use_date: chrono::NaiveDateTime,
    /// Date when the user was created.
    #[serde(rename = "create date")]
    pub create_date: chrono::NaiveDateTime,
}

impl User {
    fn with_id(self, id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..self
        }
    }
}

/// Struct for modifying configuration attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ipaddress")]
    ip_address: Option<IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netmask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dhcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyport")]
    proxy_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyaddress")]
    proxy_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linkbutton: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    touchlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zigbeechannel")]
    zigbee_channel: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "UTC")]
    current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
}

impl crate::Modifier for Modifier {}

impl Modifier {
    /// Sets the name of the bridge.
    pub fn name(self, value: &str) -> Self {
        Self {
            name: Some(value.to_owned()),
            ..self
        }
    }

    /// Sets the ip address of the bridge.
    pub fn ip_address(self, value: IpAddr) -> Self {
        Self {
            ip_address: Some(value),
            ..self
        }
    }

    /// Sets the network mask of the bridge.
    pub fn netmask(self, value: &str) -> Self {
        Self {
            netmask: Some(value.to_owned()),
            ..self
        }
    }

    /// Sets the gateway ip address of the bridge.
    pub fn gateway(self, value: IpAddr) -> Self {
        Self {
            gateway: Some(value),
            ..self
        }
    }

    /// Sets whether the ip address of the bridge is obtained with DHCP.
    pub fn dhcp(self, value: bool) -> Self {
        Self {
            dhcp: Some(value),
            ..self
        }
    }

    /// Sets the proxy port of the bridge. If set to 0 then a proxy is not being used.
    pub fn proxy_port(self, value: u16) -> Self {
        Self {
            proxy_port: Some(value),
            ..self
        }
    }

    /// Sets the proxy address of the bridge. If set to `None` then a proxy is not being used.
    pub fn proxy_address(self, value: Option<IpAddr>) -> Self {
        Self {
            proxy_address: Some(match value {
                Some(v) => v.to_string(),
                None => "none".to_owned(),
            }),
            ..self
        }
    }

    /// Indicates whether the link button has been pressed within the last 30 seconds. Writing is
    /// only allowed for portal access via cloud application_key.
    pub fn linkbutton(self, value: bool) -> Self {
        Self {
            linkbutton: Some(value),
            ..self
        }
    }

    /// Starts a touchlink procedure which adds the closest lamp (within range) to the ZigBee
    /// network. You can then search for new lights and the lamp will show up in the bridge.
    pub fn touchlink(self) -> Self {
        Self {
            touchlink: Some(true),
            ..self
        }
    }

    /// Sets the wireless frequency channel used by the bridge. It can take values of 11, 15, 20 or
    /// 25.
    pub fn zigbee_channel(self, value: u8) -> Self {
        Self {
            zigbee_channel: Some(value),
            ..self
        }
    }

    /// Sets the current time of the bridge in UTC.
    pub fn current_time(self, value: &str) -> Self {
        Self {
            current_time: Some(value.to_owned()),
            ..self
        }
    }

    /// Sets the timezone of the bridge.
    pub fn timezone(self, value: &str) -> Self {
        Self {
            timezone: Some(value.to_owned()),
            ..self
        }
    }
}
