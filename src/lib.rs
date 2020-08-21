//! Rust bindings for the [Philips Hue API].
//!
//! ## About
//!
//! This library sends HTTP requests to the bridge using the [ureq] crate. The responses/requests
//! are deserialized/serialized using the [serde], [serde_json] and [serde_repr] crates.
//!
//! [Philips Hue API]: https://developers.meethue.com/develop/hue-api
//! [ureq]: https://github.com/algesten/ureq
//! [serde]: https://github.com/serde-rs/serde
//! [serde_json]: https://github.com/serde-rs/json
//! [serde_repr]: https://github.com/dtolnay/serde-repr
//!
//! ## Examples
//!
//! Modifies the state of a light on a specific bridge:
//!
//! ```rust,no_run
//! use huelib::resource::{light, Modifier, Adjuster};
//! use huelib::Bridge;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! // Create a bridge with IP address and username.
//! let bridge = Bridge::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), "username");
//!
//! // Create a state modifier that increments the brightness by 40 and sets the saturation to 200.
//! let modifier = light::StateModifier::new()
//!     .with_brightness(Adjuster::Increment(40))
//!     .with_saturation(Adjuster::Override(200));
//!
//! // Set attributes of the light with index '1' from the modifier and print the responses.
//! match bridge.set_light_state("1", &modifier) {
//!     Ok(v) => v.iter().for_each(|response| println!("{}", response)),
//!     Err(e) => eprintln!("Failed to modify the light state: {}", e),
//! };
//! ```
//!
//! Creates a group and registers a user on a discovered bridge:
//!
//! ```rust,no_run
//! use huelib::{bridge, resource::group, Bridge};
//!
//! // Get the IP address of the bridge that was first discovered in the local network.
//! let ip_address = bridge::discover()
//!     .expect("Failed to discover bridges")
//!     .pop()
//!     .expect("No bridges found in the local network");
//!
//! // Register a user on the discovered bridge.
//! let user = bridge::register_user(ip_address, "huelib-rs example", false)
//!     .expect("Failed to register user");
//!
//! // Create a bridge with IP address and username.
//! let bridge = Bridge::new(ip_address, user.name);
//!
//! // Create a group creator that sets the name to 'group1', adds the lights with the index '1'
//! // and '2' to the group and sets the class to 'Office'.
//! let creator = group::Creator::new("group1".into(), vec!["1".into(), "2".into()])
//!     .with_class(group::Class::Office);
//!
//! // Create the group and print the identifier of the new group.
//! match bridge.create_group(&creator) {
//!     Ok(v) => println!("Created group with id '{}'", v),
//!     Err(e) => eprintln!("Failed to create group: {}", e),
//! };
//! ```

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
