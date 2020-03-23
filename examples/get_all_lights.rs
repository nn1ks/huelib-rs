//! Prints every light that is connect to a bridge.

use huelib::bridge;

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover().unwrap().pop().unwrap();

    // Register a new user.
    let user = bridge::register_user(bridge_ip, "huelib-rs example", false).unwrap();

    // Create a new bridge.
    let bridge = huelib::Bridge::new(bridge_ip, &user.name);

    // Print out every light that is connected to the bridge.
    let lights = bridge.get_all_lights().unwrap();
    println!("{:?}", lights);
}
