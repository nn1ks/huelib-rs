//! Registers a new user on a bridge and prints out the name of the new user.

use huelib::bridge;

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover().unwrap().pop().unwrap();

    // Register a new user.
    match bridge::register_user(bridge_ip, "huelib-rs example", false) {
        Ok(v) => println!("Registered a new user with username: {}", v.name),
        Err(e) => println!("Failed to register user: {}", e),
    }
}
