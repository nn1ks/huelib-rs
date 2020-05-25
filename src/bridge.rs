use crate::{resource, response, Error, Response, Result};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::net::IpAddr;

type ResponseModified = Response<response::Modified>;

/// Discovers bridges in the local netowork.
///
/// This will send a HTTP GET request to [https://discovery.meethue.com], to get IP addresses
/// of bridges that are in the local network.
///
/// [https://discovery.meethue.com]: https://discovery.meethue.com
///
/// # Examples
///
/// Save the ip addresses of the discovered bridges into a variable.
/// ```no_run
/// let ip_addresses = huelib::bridge::discover().unwrap();
/// ```
///
/// Print the ip addresses of the discovered bridges and handle errors.
/// ```no_run
/// use huelib::{bridge, Error};
///
/// match bridge::discover() {
///     Ok(v) => {
///         for ip_address in v {
///             println!("{}", ip_address);
///         }
///     },
///     Err(Error::ParseHttpResponse(_)) => eprintln!("Failed to parse http response"),
///     Err(Error::ParseJson(_)) => eprintln!("Failed to parse json content"),
///     Err(Error::ParseIpAddr(_)) => eprintln!("Failed to parse ip address"),
///     Err(_) => unreachable!()
/// };
/// ```
pub fn discover() -> Result<Vec<IpAddr>> {
    let http_response = ureq::get("https://discovery.meethue.com").call();
    #[derive(Deserialize)]
    struct BridgeJson {
        #[serde(rename = "internalipaddress")]
        ip_address: String,
    }
    let bridges: Vec<BridgeJson> = serde_json::from_value(http_response.into_json()?)?;
    let mut ip_addresses = Vec::<IpAddr>::new();
    for b in bridges {
        ip_addresses.push(b.ip_address.parse()?);
    }
    Ok(ip_addresses)
}

/// A user on a bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct User {
    /// Name of the user.
    #[serde(rename = "username")]
    pub name: String,
    /// Generated clientkey of the user.
    pub clientkey: Option<String>,
}

/// Registers a new user on a bridge.
///
/// This will send a HTTP POST request with `devicetype` and `generate_clientkey` as body to the
/// bridge with the specified IP address. The value of `devicetype` usally contains the app and
/// device name. If `generate_clientkey` is set to true the returned user will contain a random
/// generated 16 byte clientkey encoded as ASCII string of length 32.
///
/// # Examples
///
/// Print the response that contains the name of the registered user.
/// ```no_run
/// use huelib::bridge;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
/// match bridge::register_user(bridge_ip, "huelib-rs example", false) {
///     Ok(v) => println!("Registered user with username: {}", v.name),
///     Err(e) => eprintln!("{}", e),
/// };
/// ```
///
/// Print the name of the registered user and handle errors.
/// ```no_run
/// use huelib::{bridge, Error};
/// use std::net::{IpAddr, Ipv4Addr};
///
/// let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
/// match bridge::register_user(bridge_ip, "huelib-rs example", true) {
///     Ok(v) => println!("Registered user: {:?}", v),
///     Err(Error::ParseHttpResponse(_)) => eprintln!("Failed to parse http response"),
///     Err(Error::ParseJson(_)) => eprintln!("Failed to parse json content"),
///     Err(Error::Response(e)) => eprintln!("Error from the Philips Hue API: {}", e),
///     Err(Error::GetUsername) => eprintln!("Failed to get the username"),
///     Err(_) => unreachable!()
/// };
/// ```
pub fn register_user(
    ip_address: IpAddr,
    devicetype: impl AsRef<str>,
    generate_clientkey: bool,
) -> Result<User> {
    let url = format!("http://{}/api", ip_address);
    let body = if generate_clientkey {
        format!(
            "{{\"devicetype\": \"{}\", \"generateclientkey\": true}}",
            devicetype.as_ref()
        )
    } else {
        format!("{{\"devicetype\": \"{}\"}}", devicetype.as_ref())
    };
    let http_response = ureq::post(&url).send_string(&body);
    let mut responses: Vec<Response<User>> = serde_json::from_value(http_response.into_json()?)?;
    match responses.pop() {
        Some(v) => v.into_result().map_err(Error::Response),
        None => Err(Error::GetUsername),
    }
}

enum RequestType {
    Put(JsonValue),
    Post(JsonValue),
    Get,
    Delete,
}

fn parse_response<T: DeserializeOwned>(response: JsonValue) -> Result<T> {
    if let Ok(mut v) = serde_json::from_value::<Vec<Response<JsonValue>>>(response.clone()) {
        if let Some(v) = v.pop() {
            v.into_result()?;
        }
    }
    Ok(serde_json::from_value(response)?)
}

/// A bridge with IP address and username.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bridge {
    /// Name of the user that is connected to the bridge.
    pub username: String,
    /// IP address of the bridge.
    pub ip_address: IpAddr,
    /// Url to the Philips Hue API.
    api_url: String,
}

impl Bridge {
    /// Creates a new bridge.
    ///
    /// # Examples
    ///
    /// Create a bridge with an already registered user.
    /// ```no_run
    /// use huelib::Bridge;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
    /// let bridge = Bridge::new(bridge_ip, "example-username");
    /// ```
    pub fn new(ip_address: IpAddr, username: impl Into<String>) -> Self {
        let username = username.into();
        Bridge {
            api_url: format!("http://{}/api/{}", ip_address, &username),
            username,
            ip_address,
        }
    }

    /// Sends a HTTP request to the Philips Hue API and returns the response.
    fn api_request<T: DeserializeOwned>(
        &self,
        url_suffix: impl AsRef<str>,
        request_type: RequestType,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url, url_suffix.as_ref());
        let response = match request_type {
            RequestType::Put(v) => ureq::put(&url).send_json(v),
            RequestType::Post(v) => ureq::post(&url).send_json(v),
            RequestType::Get => ureq::get(&url).call(),
            RequestType::Delete => ureq::delete(&url).call(),
        };
        Ok(serde_json::from_value(response.into_json()?)?)
    }

    /// Modifies the configuration of the bridge
    pub fn set_config(
        &self,
        modifier: &resource::config::Modifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request("config", RequestType::Put(serde_json::to_value(modifier)?))
    }

    /// Returns the configuration of the bridge.
    pub fn get_config(&self) -> Result<resource::Config> {
        parse_response(self.api_request("config", RequestType::Get)?)
    }

    /// Modifies attributes of a light.
    pub fn set_light_attribute(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::light::AttributeModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("lights/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Modifies the state of a light.
    pub fn set_light_state(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::light::StateModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("lights/{}/state", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a light.
    pub fn get_light(&self, id: impl AsRef<str>) -> Result<resource::Light> {
        let light: resource::Light = parse_response(
            self.api_request(&format!("lights/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(light.with_id(id.as_ref()))
    }

    /// Returns all lights that are connected to the bridge.
    pub fn get_all_lights(&self) -> Result<Vec<resource::Light>> {
        let map: HashMap<String, resource::Light> =
            parse_response(self.api_request("lights", RequestType::Get)?)?;
        let mut lights = Vec::new();
        for (id, light) in map {
            lights.push(light.with_id(id));
        }
        Ok(lights)
    }

    /// Starts searching for new lights.
    ///
    /// The bridge will open the network for 40 seconds. The overall search might take longer since
    /// the configuration of new devices can take longer. If many devices are found the command
    /// will have to be issued a second time after discovery time has elapsed. If the command is
    /// received again during search the search will continue for at least an additional 40
    /// seconds.
    ///
    /// When the search has finished, new lights will be available using the [`get_new_lights`]
    /// function.
    ///
    /// [`get_new_lights`]: #method.get_new_lights
    pub fn search_new_lights(&self, device_ids: Option<&[&str]>) -> Result<()> {
        let body = match device_ids {
            Some(v) => format!("{{\"deviceid\": {}}}", serde_json::to_string(v)?),
            None => "".to_owned(),
        };
        let response: Vec<Response<JsonValue>> =
            self.api_request("lights", RequestType::Post(serde_json::to_value(body)?))?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Returns discovered lights.
    pub fn get_new_lights(&self) -> Result<resource::Scan> {
        parse_response(self.api_request("lights/new", RequestType::Get)?)
    }

    /// Deletes a light from the bridge.
    pub fn delete_light(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("lights/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new group.
    pub fn create_group(&self, creator: &resource::group::Creator) -> Result<String> {
        let mut response: Vec<Response<HashMap<String, String>>> =
            self.api_request("groups", RequestType::Post(serde_json::to_value(creator)?))?;
        match response.pop() {
            Some(v) => match v.into_result()?.get("id") {
                Some(v) => Ok(v.to_string()),
                None => Err(Error::GetCreatedId),
            },
            None => Err(Error::GetCreatedId),
        }
    }

    /// Modifies attributes of a group.
    pub fn set_group_attribute(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::group::AttributeModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("groups/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Modifies the state of a group.
    pub fn set_group_state(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::group::StateModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("groups/{}/action", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a group.
    pub fn get_group(&self, id: impl AsRef<str>) -> Result<resource::Group> {
        let group: resource::Group = parse_response(
            self.api_request(&format!("groups/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(group.with_id(id.as_ref()))
    }

    /// Returns all groups.
    pub fn get_all_groups(&self) -> Result<Vec<resource::Group>> {
        let map: HashMap<String, resource::Group> =
            parse_response(self.api_request("groups", RequestType::Get)?)?;
        let mut groups = Vec::new();
        for (id, group) in map {
            groups.push(group.with_id(id));
        }
        Ok(groups)
    }

    /// Deletes a group from the bridge.
    pub fn delete_group(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("groups/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new scene.
    pub fn create_scene(&self, creator: &resource::scene::Creator) -> Result<String> {
        let mut response: Vec<Response<HashMap<String, String>>> =
            self.api_request("scenes", RequestType::Post(serde_json::to_value(creator)?))?;
        match response.pop() {
            Some(v) => match v.into_result()?.get("id") {
                Some(v) => Ok(v.to_string()),
                None => Err(Error::GetCreatedId),
            },
            None => Err(Error::GetCreatedId),
        }
    }

    /// Modifies the state and attributes of a scene.
    pub fn set_scene(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::scene::Modifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("scenes/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a scene.
    pub fn get_scene(&self, id: impl AsRef<str>) -> Result<resource::Scene> {
        let scene: resource::Scene = parse_response(
            self.api_request(&format!("scenes/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(scene.with_id(id.as_ref()))
    }

    /// Returns all scenes.
    pub fn get_all_scenes(&self) -> Result<Vec<resource::Scene>> {
        let map: HashMap<String, resource::Scene> =
            parse_response(self.api_request("scenes", RequestType::Get)?)?;
        let mut scenes = Vec::new();
        for (id, scene) in map {
            scenes.push(scene.with_id(id));
        }
        Ok(scenes)
    }

    /// Deletes a scene.
    pub fn delete_scene(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("scenes/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Returns the capabilities of resources.
    pub fn get_capabilities(&self) -> Result<resource::Capabilities> {
        parse_response(self.api_request("capabilities", RequestType::Get)?)
    }

    /// Creates a new schedule and returns the identifier.
    pub fn create_schedule(&self, creator: &resource::schedule::Creator) -> Result<String> {
        let mut response: Vec<Response<HashMap<String, String>>> = self.api_request(
            "schedules",
            RequestType::Post(serde_json::to_value(creator)?),
        )?;
        match response.pop() {
            Some(v) => match v.into_result()?.get("id") {
                Some(v) => Ok(v.to_string()),
                None => Err(Error::GetCreatedId),
            },
            None => Err(Error::GetCreatedId),
        }
    }

    /// Modifies attributes of a schedule.
    pub fn set_schedule(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::schedule::Modifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("schedules/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a schedule.
    pub fn get_schedule(&self, id: impl AsRef<str>) -> Result<resource::Schedule> {
        let schedule: resource::Schedule = parse_response(
            self.api_request(&format!("schedules/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(schedule.with_id(id.as_ref()))
    }

    /// Returns all schedules.
    pub fn get_all_schedules(&self) -> Result<Vec<resource::Schedule>> {
        let map: HashMap<String, resource::Schedule> =
            parse_response(self.api_request("schedules", RequestType::Get)?)?;
        let mut schedules = Vec::new();
        for (id, schedule) in map {
            schedules.push(schedule.with_id(id));
        }
        Ok(schedules)
    }

    /// Deletes a schedule.
    pub fn delete_schedule(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("schedules/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new resourcelink and returns the identifier.
    pub fn create_resourcelink(&self, creator: &resource::resourcelink::Creator) -> Result<String> {
        let mut response: Vec<Response<HashMap<String, String>>> = self.api_request(
            "resourcelinks",
            RequestType::Post(serde_json::to_value(creator)?),
        )?;
        match response.pop() {
            Some(v) => match v.into_result()?.get("id") {
                Some(v) => Ok(v.to_string()),
                None => Err(Error::GetCreatedId),
            },
            None => Err(Error::GetCreatedId),
        }
    }

    /// Modifies attributes of a resourcelink.
    pub fn set_resourcelink(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::resourcelink::Modifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("resourcelinks/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a resourcelink.
    pub fn get_resourcelink(&self, id: impl AsRef<str>) -> Result<resource::Resourcelink> {
        let resourcelink: resource::Resourcelink = parse_response(
            self.api_request(&format!("resourcelinks/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(resourcelink.with_id(id.as_ref()))
    }

    /// Returns all resourcelinks.
    pub fn get_all_resourcelinks(&self) -> Result<Vec<resource::Resourcelink>> {
        let map: HashMap<String, resource::Resourcelink> =
            parse_response(self.api_request("resourcelinks", RequestType::Get)?)?;
        let mut resourcelinks = Vec::new();
        for (id, resourcelink) in map {
            resourcelinks.push(resourcelink.with_id(id));
        }
        Ok(resourcelinks)
    }

    /// Deletes a resourcelink.
    pub fn delete_resourcelink(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("resourcelinks/{}", id.as_ref()),
            RequestType::Delete,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Modifies attributes of a sensor.
    pub fn set_sensor_attribute(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::sensor::AttributeModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("sensors/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Modifies the state of a sensor.
    pub fn set_sensor_state(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::sensor::StateModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("sensors/{}/state", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Modifies the configuration of a sensor.
    pub fn set_sensor_config(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::sensor::ConfigModifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("sensors/{}/config", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a sensor.
    pub fn get_sensor(&self, id: impl AsRef<str>) -> Result<resource::Sensor> {
        let sensor: resource::Sensor = parse_response(
            self.api_request(&format!("sensors/{}", id.as_ref()), RequestType::Get)?,
        )?;
        Ok(sensor.with_id(id.as_ref()))
    }

    /// Returns all sensors that are connected to the bridge.
    pub fn get_all_sensors(&self) -> Result<Vec<resource::Sensor>> {
        let map: HashMap<String, resource::Sensor> =
            parse_response(self.api_request("sensors", RequestType::Get)?)?;
        let mut sensors = Vec::new();
        for (id, sensor) in map {
            sensors.push(sensor.with_id(id));
        }
        Ok(sensors)
    }

    /// Starts searching for new sensors.
    ///
    /// The bridge will open the network for 40 seconds. The overall search might take longer since
    /// the configuration of new devices can take longer. If many devices are found the command
    /// will have to be issued a second time after discovery time has elapsed. If the command is
    /// received again during search the search will continue for at least an additional 40
    /// seconds.
    ///
    /// When the search has finished, new sensors will be available using the [`get_new_sensors`]
    /// function.
    ///
    /// [`get_new_sensors`]: #method.get_new_sensors
    pub fn search_new_sensors(&self, device_ids: Option<&[&str]>) -> Result<()> {
        let body = match device_ids {
            Some(v) => format!("{{\"deviceid\": {}}}", serde_json::to_string(v)?),
            None => "".to_owned(),
        };
        let response: Vec<Response<JsonValue>> =
            self.api_request("sensors", RequestType::Post(serde_json::to_value(body)?))?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Returns discovered sensors.
    pub fn get_new_sensors(&self) -> Result<resource::Scan> {
        parse_response(self.api_request("sensors/new", RequestType::Get)?)
    }

    /// Deletes a sensor from the bridge.
    pub fn delete_sensor(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("sensors/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new rule.
    pub fn create_rule(&self, creator: &resource::rule::Creator) -> Result<String> {
        let mut response: Vec<Response<HashMap<String, String>>> =
            self.api_request("rules", RequestType::Post(serde_json::to_value(creator)?))?;
        match response.pop() {
            Some(v) => match v.into_result()?.get("id") {
                Some(v) => Ok(v.to_string()),
                None => Err(Error::GetCreatedId),
            },
            None => Err(Error::GetCreatedId),
        }
    }

    /// Modifies attributes of a rule.
    pub fn set_rule(
        &self,
        id: impl AsRef<str>,
        modifier: &resource::rule::Modifier,
    ) -> Result<Vec<ResponseModified>> {
        self.api_request(
            &format!("rules/{}", id.as_ref()),
            RequestType::Put(serde_json::to_value(modifier)?),
        )
    }

    /// Returns a rule.
    pub fn get_rule(&self, id: impl AsRef<str>) -> Result<resource::Rule> {
        let rule: resource::Rule =
            parse_response(self.api_request(&format!("rules/{}", id.as_ref()), RequestType::Get)?)?;
        Ok(rule.with_id(id.as_ref()))
    }

    /// Returns all rules.
    pub fn get_all_rules(&self) -> Result<Vec<resource::Rule>> {
        let map: HashMap<String, resource::Rule> =
            parse_response(self.api_request("rules", RequestType::Get)?)?;
        let mut rules = Vec::new();
        for (id, rule) in map {
            rules.push(rule.with_id(id));
        }
        Ok(rules)
    }

    /// Deletes a rule.
    pub fn delete_rule(&self, id: impl AsRef<str>) -> Result<()> {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("rules/{}", id.as_ref()), RequestType::Delete)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }
}
