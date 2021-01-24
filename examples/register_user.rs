//! Registers a new user on a bridge and prints out the name of the new user.

use huelib::bridge;

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover_nupnp().unwrap().pop().unwrap();

    // Register a new user.
    let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();
    println!("Registered a new user with username: {}", username);
}
