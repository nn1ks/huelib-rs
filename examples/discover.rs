//! Discovers Philips Hue bridges in the local network and prints out their IP addresses.

fn main() {
    // Get the ip addresses of all bridges that were discovered.
    let ip_addresses = huelib::bridge::discover().unwrap();

    // Print every ip address.
    for i in ip_addresses {
        println!("{}", i);
    }
}
