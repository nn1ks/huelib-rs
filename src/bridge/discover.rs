use crate::Result;
use serde::Deserialize;
use std::net::IpAddr;

/// Discovers bridges in the local netowork using N-UPnP.
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
/// let ip_addresses = huelib::bridge::discover_nupnp()?;
/// # Ok(())
/// # }
/// ```
///
/// Register a user on the bridge that was first discovered:
/// ```no_run
/// use huelib::bridge;
///
/// # fn main() -> Result<(), huelib::Error> {
/// let ip = bridge::discover_nupnp()?.pop().expect("found no bridges");
/// let username = bridge::register_user(ip, "example")?;
/// println!("Registered user: {}", username);
/// # Ok(())
/// # }
/// ```
pub fn discover_nupnp() -> Result<Vec<IpAddr>> {
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
