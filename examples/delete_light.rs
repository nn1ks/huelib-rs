//! Deletes a specific light.

use huelib::{bridge, Bridge};

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover_nupnp().unwrap().pop().unwrap();

    // Register a new user.
    let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();

    // Create a new bridge.
    let bridge = Bridge::new(bridge_ip, username);

    // Deletes the light with the id 1.
    match bridge.delete_light("1") {
        Ok(_) => println!("Deleted light"),
        Err(e) => println!("Failed to delete light: {}", e),
    };
}
