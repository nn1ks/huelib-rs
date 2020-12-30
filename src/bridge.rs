use crate::resource::{self, Creator, Modifier, RequestMethod, Scanner};
use crate::{response::Modified, Error, Response, Result};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, net::IpAddr};

type ResponsesModified = Vec<Response<Modified>>;

/// Discovers bridges in the local netowork.
///
/// This sends a HTTP GET request to [https://discovery.meethue.com], to get IP addresses of bridges
/// that are in the local network.
///
/// [https://discovery.meethue.com]: https://discovery.meethue.com
///
/// # Examples
///
/// Get the IP addresses of all discovered bridges:
/// ```no_run
/// # fn main() -> Result<(), huelib::Error> {
/// let ip_addresses = huelib::bridge::discover()?;
/// # Ok(())
/// # }
/// ```
///
/// Register a user on the bridge that was first discovered:
/// ```no_run
/// use huelib::bridge;
///
/// # fn main() -> Result<(), huelib::Error> {
/// let ip = bridge::discover()?.pop().expect("found no bridges");
/// let username = bridge::register_user(ip, "example")?;
/// println!("Registered user: {}", username);
/// # Ok(())
/// # }
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

/// Registers a new user on a bridge.
///
/// This function returns the new username. See the [`register_user_with_clientkey`] function if you
/// also want to generate a clientkey.
///
/// # Examples
///
/// Register a user and print the username:
/// ```no_run
/// use huelib::bridge;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// # fn main() -> Result<(), huelib::Error> {
/// let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
/// let username = bridge::register_user(bridge_ip, "example")?;
/// println!("Registered user with username `{}`", username);
/// # Ok(())
/// # }
/// ```
pub fn register_user<S>(ip_address: IpAddr, devicetype: S) -> Result<String>
where
    S: AsRef<str>,
{
    let url = format!("http://{}/api", ip_address);
    let body = format!("{{\"devicetype\":\"{}\"}}", devicetype.as_ref());
    let http_response = ureq::post(&url).send_string(&body);
    #[derive(Deserialize)]
    struct User {
        username: String,
    }
    let mut responses: Vec<Response<User>> = serde_json::from_value(http_response.into_json()?)?;
    match responses.pop() {
        Some(v) => match v.into_result() {
            Ok(user) => Ok(user.username),
            Err(e) => Err(Error::Response(e)),
        },
        None => Err(Error::GetUsername),
    }
}

/// Registers a new user on a bridge with a clientkey.
///
/// This function returns the new username and a random generated 16 byte clientkey encoded as ASCII
/// string of length 32. See the [`register_user`] function if you don't want to generate a
/// clientkey.
///
/// # Examples
///
/// Register a user and print the username and clientkey:
/// ```no_run
/// use huelib::bridge;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// # fn main() -> Result<(), huelib::Error> {
/// let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
/// let (username, clientkey) = bridge::register_user_with_clientkey(bridge_ip, "example")?;
/// println!("Registered user with username `{}` and clientkey `{}`", username, clientkey);
/// # Ok(())
/// # }
/// ```
pub fn register_user_with_clientkey<S>(
    ip_address: IpAddr,
    devicetype: S,
) -> Result<(String, String)>
where
    S: AsRef<str>,
{
    let url = format!("http://{}/api", ip_address);
    let body = format!(
        "{{\"devicetype\":\"{}\",\"generateclientkey\":true}}",
        devicetype.as_ref()
    );
    let http_response = ureq::post(&url).send_string(&body);
    #[derive(Deserialize)]
    struct User {
        username: String,
        clientkey: String,
    }
    let mut responses: Vec<Response<User>> = serde_json::from_value(http_response.into_json()?)?;
    match responses.pop() {
        Some(v) => match v.into_result() {
            Ok(user) => Ok((user.username, user.clientkey)),
            Err(e) => Err(Error::Response(e)),
        },
        None => Err(Error::GetUsername),
    }
}

fn parse_response<T>(response: JsonValue) -> crate::Result<T>
where
    T: DeserializeOwned,
{
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
    username: String,
    /// IP address of the bridge.
    ip_address: IpAddr,
    /// Url to the Philips Hue API.
    api_url: String,
}

impl Bridge {
    /// Creates a new bridge.
    ///
    /// # Examples
    ///
    /// Create a bridge with an already registered user:
    /// ```
    /// use huelib::Bridge;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
    /// let bridge = Bridge::new(ip, "username");
    /// ```
    pub fn new<S>(ip_address: IpAddr, username: S) -> Self
    where
        S: Into<String>,
    {
        let username = username.into();
        Bridge {
            api_url: format!("http://{}/api/{}", ip_address, username),
            username,
            ip_address,
        }
    }

    /// Returns the name of the user that is connected to the bridge.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the IP address of the bridge.
    pub fn ip_address(&self) -> &IpAddr {
        &self.ip_address
    }

    /// Sends a HTTP request to the Philips Hue API and returns the response.
    pub(crate) fn api_request<S, T>(
        &self,
        url_suffix: S,
        request_method: RequestMethod,
        body: Option<JsonValue>,
    ) -> Result<T>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.api_url, url_suffix.as_ref());
        let mut request = match request_method {
            RequestMethod::Put => ureq::put(&url),
            RequestMethod::Post => ureq::post(&url),
            RequestMethod::Get => ureq::get(&url),
            RequestMethod::Delete => ureq::delete(&url),
        };
        let response = match body {
            Some(v) => request.send_json(v),
            None => request.call(),
        };
        Ok(serde_json::from_value(response.into_json()?)?)
    }

    /// Modifies the configuration of the bridge.
    pub fn set_config(&self, modifier: &resource::config::Modifier) -> Result<ResponsesModified> {
        modifier.execute(self, ())
    }

    /// Returns the configuration of the bridge.
    pub fn get_config(&self) -> Result<resource::Config> {
        parse_response(self.api_request("config", RequestMethod::Get, None)?)
    }

    /// Modifies attributes of a light.
    pub fn set_light_attribute<S>(
        &self,
        id: S,
        modifier: &resource::light::AttributeModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Modifies the state of a light.
    pub fn set_light_state<S>(
        &self,
        id: S,
        modifier: &resource::light::StateModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a light.
    pub fn get_light<S>(&self, id: S) -> Result<resource::Light>
    where
        S: Into<String>,
    {
        let id = id.into();
        let light: resource::Light = parse_response(self.api_request(
            format!("lights/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(light.with_id(id))
    }

    /// Returns all lights that are connected to the bridge.
    pub fn get_all_lights(&self) -> Result<Vec<resource::Light>> {
        let map: HashMap<String, resource::Light> =
            parse_response(self.api_request("lights", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, light)| light.with_id(id))
            .collect())
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
    pub fn search_new_lights(&self, scanner: &resource::light::Scanner) -> Result<()> {
        scanner.execute(self)
    }

    /// Returns discovered lights.
    pub fn get_new_lights(&self) -> Result<resource::Scan> {
        parse_response(self.api_request("lights/new", RequestMethod::Get, None)?)
    }

    /// Deletes a light from the bridge.
    pub fn delete_light<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("lights/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new group.
    pub fn create_group(&self, creator: &resource::group::Creator) -> Result<String> {
        creator.execute(self)
    }

    /// Modifies attributes of a group.
    pub fn set_group_attribute<S>(
        &self,
        id: S,
        modifier: &resource::group::AttributeModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Modifies the state of a group.
    pub fn set_group_state<S>(
        &self,
        id: S,
        modifier: &resource::group::StateModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a group.
    pub fn get_group<S>(&self, id: S) -> Result<resource::Group>
    where
        S: Into<String>,
    {
        let id = id.into();
        let group: resource::Group = parse_response(self.api_request(
            format!("groups/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(group.with_id(id))
    }

    /// Returns all groups.
    pub fn get_all_groups(&self) -> Result<Vec<resource::Group>> {
        let map: HashMap<String, resource::Group> =
            parse_response(self.api_request("groups", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, group)| group.with_id(id))
            .collect())
    }

    /// Deletes a group from the bridge.
    pub fn delete_group<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("groups/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new scene.
    pub fn create_scene(&self, creator: &resource::scene::Creator) -> Result<String> {
        creator.execute(self)
    }

    /// Modifies the state and attributes of a scene.
    pub fn set_scene<S>(
        &self,
        id: S,
        modifier: &resource::scene::Modifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a scene.
    pub fn get_scene<S>(&self, id: S) -> Result<resource::Scene>
    where
        S: Into<String>,
    {
        let id = id.into();
        let scene: resource::Scene = parse_response(self.api_request(
            format!("scenes/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(scene.with_id(id))
    }

    /// Returns all scenes.
    pub fn get_all_scenes(&self) -> Result<Vec<resource::Scene>> {
        let map: HashMap<String, resource::Scene> =
            parse_response(self.api_request("scenes", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, scene)| scene.with_id(id))
            .collect())
    }

    /// Deletes a scene.
    pub fn delete_scene<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("scenes/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Returns the capabilities of resources.
    pub fn get_capabilities(&self) -> Result<resource::Capabilities> {
        parse_response(self.api_request("capabilities", RequestMethod::Get, None)?)
    }

    /// Creates a new schedule and returns the identifier.
    pub fn create_schedule(&self, creator: &resource::schedule::Creator) -> Result<String> {
        creator.execute(&self)
    }

    /// Modifies attributes of a schedule.
    pub fn set_schedule<S>(
        &self,
        id: S,
        modifier: &resource::schedule::Modifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a schedule.
    pub fn get_schedule<S>(&self, id: S) -> Result<resource::Schedule>
    where
        S: Into<String>,
    {
        let id = id.into();
        let schedule: resource::Schedule = parse_response(self.api_request(
            format!("schedules/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(schedule.with_id(id))
    }

    /// Returns all schedules.
    pub fn get_all_schedules(&self) -> Result<Vec<resource::Schedule>> {
        let map: HashMap<String, resource::Schedule> =
            parse_response(self.api_request("schedules", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, schedule)| schedule.with_id(id))
            .collect())
    }

    /// Deletes a schedule.
    pub fn delete_schedule<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("schedules/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new resourcelink and returns the identifier.
    pub fn create_resourcelink(&self, creator: &resource::resourcelink::Creator) -> Result<String> {
        creator.execute(self)
    }

    /// Modifies attributes of a resourcelink.
    pub fn set_resourcelink<S>(
        &self,
        id: S,
        modifier: &resource::resourcelink::Modifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a resourcelink.
    pub fn get_resourcelink<S>(&self, id: S) -> Result<resource::Resourcelink>
    where
        S: Into<String>,
    {
        let id = id.into();
        let resourcelink: resource::Resourcelink = parse_response(self.api_request(
            format!("resourcelinks/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(resourcelink.with_id(id))
    }

    /// Returns all resourcelinks.
    pub fn get_all_resourcelinks(&self) -> Result<Vec<resource::Resourcelink>> {
        let map: HashMap<String, resource::Resourcelink> =
            parse_response(self.api_request("resourcelinks", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, resourcelink)| resourcelink.with_id(id))
            .collect())
    }

    /// Deletes a resourcelink.
    pub fn delete_resourcelink<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("resourcelinks/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Modifies attributes of a sensor.
    pub fn set_sensor_attribute<S>(
        &self,
        id: S,
        modifier: &resource::sensor::AttributeModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Modifies the state of a sensor.
    pub fn set_sensor_state<S>(
        &self,
        id: S,
        modifier: &resource::sensor::StateModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Modifies the configuration of a sensor.
    pub fn set_sensor_config<S>(
        &self,
        id: S,
        modifier: &resource::sensor::ConfigModifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a sensor.
    pub fn get_sensor<S>(&self, id: S) -> Result<resource::Sensor>
    where
        S: Into<String>,
    {
        let id = id.into();
        let sensor: resource::Sensor = parse_response(self.api_request(
            format!("sensors/{}", id),
            RequestMethod::Get,
            None,
        )?)?;
        Ok(sensor.with_id(id))
    }

    /// Returns all sensors that are connected to the bridge.
    pub fn get_all_sensors(&self) -> Result<Vec<resource::Sensor>> {
        let map: HashMap<String, resource::Sensor> =
            parse_response(self.api_request("sensors", RequestMethod::Get, None)?)?;
        Ok(map
            .into_iter()
            .map(|(id, sensor)| sensor.with_id(id))
            .collect())
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
    pub fn search_new_sensors(&self, scanner: &resource::sensor::Scanner) -> Result<()> {
        scanner.execute(self)
    }

    /// Returns discovered sensors.
    pub fn get_new_sensors(&self) -> Result<resource::Scan> {
        parse_response(self.api_request("senors/new", RequestMethod::Get, None)?)
    }

    /// Deletes a sensor from the bridge.
    pub fn delete_sensor<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> = self.api_request(
            &format!("sensors/{}", id.into()),
            RequestMethod::Delete,
            None,
        )?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }

    /// Creates a new rule.
    pub fn create_rule(&self, creator: &resource::rule::Creator) -> Result<String> {
        creator.execute(self)
    }

    /// Modifies attributes of a rule.
    pub fn set_rule<S>(
        &self,
        id: S,
        modifier: &resource::rule::Modifier,
    ) -> Result<ResponsesModified>
    where
        S: Into<String>,
    {
        modifier.execute(self, id.into())
    }

    /// Returns a rule.
    pub fn get_rule<S>(&self, id: S) -> Result<resource::Rule>
    where
        S: Into<String>,
    {
        let id = id.into();
        let rule: resource::Rule =
            parse_response(self.api_request(format!("rules/{}", id), RequestMethod::Get, None)?)?;
        Ok(rule.with_id(id))
    }

    /// Returns all rules.
    pub fn get_all_rules(&self) -> Result<Vec<resource::Rule>> {
        let map: HashMap<String, resource::Rule> =
            parse_response(self.api_request("rules", RequestMethod::Get, None)?)?;
        Ok(map.into_iter().map(|(id, rule)| rule.with_id(id)).collect())
    }

    /// Deletes a rule.
    pub fn delete_rule<S>(&self, id: S) -> Result<()>
    where
        S: Into<String>,
    {
        let response: Vec<Response<JsonValue>> =
            self.api_request(&format!("rules/{}", id.into()), RequestMethod::Delete, None)?;
        for i in response {
            i.into_result()?;
        }
        Ok(())
    }
}
