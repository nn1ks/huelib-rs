//! Prints every light that is connect to a bridge.

use huelib::{bridge, Bridge};

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover().unwrap().pop().unwrap();

    // Register a new user.
    let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();

    // Create a new bridge.
    let bridge = Bridge::new(bridge_ip, username);

    // Print out every light that is connected to the bridge.
    let lights = bridge.get_all_lights().unwrap();
    println!("{:?}", lights);
}
