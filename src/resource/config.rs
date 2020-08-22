use crate::{resource, util};
use derive_setters::Setters;
use serde::{Deserialize, Deserializer, Serialize};
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
    #[serde(
        rename = "localtime",
        deserialize_with = "util::deserialize_option_date_time"
    )]
    pub local_time: Option<chrono::NaiveDateTime>,
    /// Timezone of the bridge as OlsenIDs.
    #[serde(deserialize_with = "util::deserialize_option_string")]
    pub timezone: Option<String>,
    /// The current wireless frequency channel used by the bridge.
    ///
    /// It can take values of 11, 15, 20, 25 or 0 if undefined (factory new).
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
    /// Identifier of the bridge where a backup was restored.
    ///
    /// If no backup was restored from another bridge, this will be `None`.
    pub replaces_bridge_id: Option<String>,
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

impl resource::Resource for Config {}

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
    #[serde(
        rename = "updatetime",
        deserialize_with = "util::deserialize_option_time"
    )]
    pub update_time: Option<chrono::NaiveTime>,
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
#[serde(rename_all = "lowercase")]
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
    /// Specifies the last error source if the backup has detected an internal error.
    ///
    /// Cleared at the start of a backup import or export.
    #[serde(rename = "errorcode")]
    pub error: BackupError,
}

/// Status of backup/restore.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum BackupStatus {
    /// No backup or restore ongoing.
    #[serde(rename = "idle")]
    Idle,
    /// Indicates that a file for migration is being created.
    ///
    /// It can only be written if status is `Idle` and puts the bridge in `FilereadyDisable`
    /// status. CLIP is not available for some time after this command.
    #[serde(rename = "startmigration")]
    StartMigration,
    /// Indicates that a backup file is available and that this bridge has been disabled due to a
    /// migration procedure.
    ///
    /// The bridge can be activated again by a factory reset or power cycle.
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
    fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

/// Struct for modifying configuration attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the IP address of the bridge.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ipaddress")]
    pub ip_address: Option<IpAddr>,
    /// Sets the network mask of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    /// Sets the gateway IP address of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<IpAddr>,
    /// Sets whether the IP address of the bridge is obtained with DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    /// Sets the proxy port of the bridge.
    ///
    /// If set to 0 then a proxy is not being used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyport")]
    pub proxy_port: Option<u16>,
    /// Sets the proxy address of the bridge.
    ///
    /// If set to `None` then a proxy is not being used.
    #[serde(skip_serializing_if = "Option::is_none", rename = "proxyaddress")]
    pub proxy_address: Option<String>,
    /// Indicates whether the link button has been pressed within the last 30 seconds.
    ///
    /// Writing is only allowed for portal access via cloud application_key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkbutton: Option<bool>,
    /// Starts a touchlink procedure which adds the closest lamp to the ZigBee network.
    ///
    /// You can then search for new lights and the lamp will show up in the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touchlink: Option<bool>,
    /// Sets the wireless frequency channel used by the bridge.
    ///
    /// It can take values of 11, 15, 20 or 25.
    #[serde(skip_serializing_if = "Option::is_none", rename = "zigbeechannel")]
    pub zigbee_channel: Option<u8>,
    /// Sets the current time of the bridge in UTC.
    #[serde(skip_serializing_if = "Option::is_none", rename = "UTC")]
    pub current_time: Option<String>,
    /// Sets the timezone of the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    use std::net::Ipv4Addr;

    #[test]
    fn serialize_modifier() {
        let modifier = Modifier::new();
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({});
        assert_eq!(modifier_json, expected_json);

        let modifier = Modifier {
            name: Some("test".into()),
            ip_address: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))),
            netmask: Some("255.255.255.0".into()),
            gateway: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
            dhcp: Some(true),
            proxy_port: Some(0),
            proxy_address: Some("192.168.2.1".into()),
            linkbutton: Some(false),
            touchlink: Some(false),
            zigbee_channel: Some(1),
            current_time: Some("2020-01-01T00:00:00".into()),
            timezone: Some("Europe/Berlin".into()),
        };
        let modifier_json = serde_json::to_value(modifier).unwrap();
        let expected_json = json!({
            "name": "test",
            "ipaddress": "192.168.1.2",
            "netmask": "255.255.255.0",
            "gateway": "192.168.1.1",
            "dhcp": true,
            "proxyport": 0,
            "proxyaddress": "192.168.2.1",
            "linkbutton": false,
            "touchlink": false,
            "zigbeechannel": 1,
            "UTC": "2020-01-01T00:00:00",
            "timezone": "Europe/Berlin"
        });
        assert_eq!(modifier_json, expected_json);
    }
}
