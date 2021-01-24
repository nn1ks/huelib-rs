use crate::{Error, Response, Result};
use serde::Deserialize;
use std::net::IpAddr;

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
    let http_response = ureq::post(&url).send_string(&body)?;
    #[derive(Deserialize)]
    struct User {
        username: String,
    }
    let mut responses: Vec<Response<User>> = http_response.into_json()?;
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
    let http_response = ureq::post(&url).send_string(&body)?;
    #[derive(Deserialize)]
    struct User {
        username: String,
        clientkey: String,
    }
    let mut responses: Vec<Response<User>> = http_response.into_json()?;
    match responses.pop() {
        Some(v) => match v.into_result() {
            Ok(user) => Ok((user.username, user.clientkey)),
            Err(e) => Err(Error::Response(e)),
        },
        None => Err(Error::GetUsername),
    }
}
