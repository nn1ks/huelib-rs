//! Rust bindings for the [Philips Hue API].
//!
//! [Philips Hue API]: https://developers.meethue.com/develop/hue-api
//!
//! # About
//!
//! The minimum supported API version is `1.37`.
//!
//! This library sends HTTP requests to the bridge using the [ureq] crate. The responses/requests
//! are deserialized/serialized using the [serde], [serde_json] and [serde_repr] crates.
//!
//! [ureq]: https://github.com/algesten/ureq
//! [serde]: https://github.com/serde-rs/serde
//! [serde_json]: https://github.com/serde-rs/json
//! [serde_repr]: https://github.com/dtolnay/serde-repr
//!
//! # Connecting to a bridge
//!
//! To connect to a bridge, the IP address of the bridge and the name of a registered user is
//! needed. You can use the [`bridge::discover`] function to get the IP addresses of bridges that
//! are in the local network and the [`bridge::register_user`] function to register a new user on a
//! bridge.
//!
//! To able to send requests to the bridge, a [`Bridge`] must be created. For example:
//! ```no_run
//! use huelib::Bridge;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), "username");
//! ```
//!
//! You can then send requests using either the methods of [`Bridge`] or the different traits
//! ([`Creator`], [`Modifier`], etc.).
//!
//! # Sending requests using `Bridge` methods
//!
//! The methods of [`Bridge`] can be used to send requests.
//!
//! Methods beginning with `create`, `set`, and `search_new` take either a creator, modifier, or
//! scanner as parameter which has to be constructed before sending the request. For example, you
//! can construct a [`group::Creator`] with the `new` function and then pass it to the
//! [`Bridge::create_group`] method as a parameter.
//!
//! For a list of available methods, view the documentation of [`Bridge`].
//!
//! # Sending requests using trait methods
//!
//! Some trait methods can be used to send requests instead of calling a [`Bridge`] method.
//!
//! - [`Creator::execute`]: Can be used instead of `Bridge::create_*` methods.
//! - [`Modifier::execute`]: Can be used instead of `Bridge::set_*` methods.
//! - [`Scanner::execute`]: Can be used instead of `Bridge::search_new_*` methods
//!
//! # Examples
//!
//! _Note: In the following examples the creation of `bridge` is abbreviated to reduce irrelevant
//! code._
//!
//! ## Creating a group
//!
//! Creates a new group with the name `example` and puts the light with the identifier `1` into the
//! group and sets the class to `Office`.
//!
//! - Using the [`Bridge::create_group`] method:
//!
//!     ```no_run
//!     use huelib::resource::group;
//!
//!     # fn main() -> huelib::Result<()> {
//!     # use huelib::Bridge;
//!     # use std::net::{IpAddr, Ipv4Addr};
//!     # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//!     // let bridge = Bridge::new(...);
//!     let creator = group::Creator::new("example".into(), vec!["1".into()])
//!         .with_class(group::Class::Office);
//!     let id = bridge.create_group(&creator)?;
//!     println!("Created group with id `{}`", id);
//!     # Ok(())
//!     # }
//!     ```
//!
//! - Using the [`Creator::execute`] trait method:
//!
//!     ```no_run
//!     // Note that the trait `Creator` has to be in scope because the `execute` method is called.
//!     use huelib::resource::{group, Creator};
//!
//!     # fn main() -> huelib::Result<()> {
//!     # use huelib::Bridge;
//!     # use std::net::{IpAddr, Ipv4Addr};
//!     # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//!     // let bridge = Bridge::new(...);
//!     let id = group::Creator::new("example".into(), vec!["1".into()])
//!         .with_class(group::Class::Office)
//!         .execute(&bridge)?;
//!     println!("Created group with id `{}`", id);
//!     # Ok(())
//!     # }
//!     ```
//!
//! ## Modifying a light state
//!
//! Turns the light with the identifier `1` on and sets the color to red.
//!
//! - Using the [`Bridge::set_light_state`] method:
//!
//!     ```no_run
//!     use huelib::{resource::light, Color};
//!
//!     # fn main() -> huelib::Result<()> {
//!     # use huelib::Bridge;
//!     # use std::net::{IpAddr, Ipv4Addr};
//!     # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//!     // let bridge = Bridge::new(...);
//!     let modifier = light::StateModifier::new()
//!         .with_on(true)
//!         .with_color(Color::from_rgb(255, 0, 0));
//!     let responses = bridge.set_light_state("1", &modifier)?;
//!     # Ok(())
//!     # }
//!     ```
//!
//! - Using the [`Modifier::execute`] trait method:
//!
//!     ```no_run
//!     // Note that the trait `Modifier` has to be in scope because the `execute` method is called.
//!     use huelib::resource::{light, Modifier};
//!     use huelib::Color;
//!
//!     # fn main() -> huelib::Result<()> {
//!     # use huelib::Bridge;
//!     # use std::net::{IpAddr, Ipv4Addr};
//!     # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//!     // let bridge = Bridge::new(...);
//!     let responses = light::StateModifier::new()
//!         .with_on(true)
//!         .with_color(Color::from_rgb(255, 0, 0))
//!         .execute(&bridge, "1".into())?;
//!     # Ok(())
//!     # }
//!     ```
//!
//! ## Getting a light
//!
//! Print the light with the identifier `1`:
//!
//! ```no_run
//! # fn main() -> huelib::Result<()> {
//! # use huelib::Bridge;
//! # use std::net::{IpAddr, Ipv4Addr};
//! # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//! // let bridge = Bridge::new(...);
//! let light = bridge.get_light("1")?;
//! println!("Light 1: {:?}", light);
//! # Ok(())
//! # }
//! ```
//!
//! ## Searching for new sensors
//!
//! Start searching for new sensors:
//!
//! ```no_run
//! use huelib::resource::sensor;
//!
//! # fn main() -> huelib::Result<()> {
//! # use huelib::Bridge;
//! # use std::net::{IpAddr, Ipv4Addr};
//! # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//! // let bridge = Bridge::new(...);
//! let scanner = sensor::Scanner::new();
//! bridge.search_new_sensors(&scanner)?;
//! # Ok(())
//! # }
//! ```
//!
//! Print the discovered sensors:
//!
//! ```no_run
//! # fn main() -> huelib::Result<()> {
//! # use huelib::Bridge;
//! # use std::net::{IpAddr, Ipv4Addr};
//! # let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), String::new());
//! // let bridge = Bridge::new(...);
//! let scan = bridge.get_new_sensors()?;
//! for resource in scan.resources {
//!     println!("Discovered sensor `{}` with ID `{}`", resource.name, resource.id);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`Creator`]: resource::Creator
//! [`Modifier`]: resource::Modifier
//! [`Creator::execute`]: resource::Creator::execute
//! [`Modifier::execute`]: resource::Modifier::execute
//! [`Scanner::execute`]: resource::Scanner::execute
//! [`group::Creator`]: resource::group::Creator

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, missing_docs, missing_debug_implementations)]

#[macro_use]
mod util;
mod error;

/// Module for managing bridges.
pub mod bridge;
/// Module for generating colors.
pub mod color;
/// Module for bridge resources.
pub mod resource;
/// Responses returned from the Philips Hue API.
pub mod response;

pub use bridge::Bridge;
pub use color::Color;
pub use error::{Error, Result};
pub use response::Response;
