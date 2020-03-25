//! Modifies the state of a specific light.

use huelib::{bridge, light, Modifier};

fn main() {
    // Discover bridges in the local network and save the first IP address as `bridge_ip`.
    let bridge_ip = bridge::discover().unwrap().pop().unwrap();

    // Register a new user.
    let user = bridge::register_user(bridge_ip, "huelib-rs example", false).unwrap();

    // Create a new bridge.
    let bridge = huelib::Bridge::new(bridge_ip, &user.name);

    // Creates a new light modifier to turn on the light, set the saturation to 10 and decrement
    // the brightness by 40.
    let light_modifier = light::StateModifier::new()
        .on(true)
        .saturation(huelib::ModifierType::Override, 10)
        .alert(huelib::Alert::Select)
        .brightness(huelib::ModifierType::Decrement, 40);

    // Modify the attributes declared in `light_modifier` on the light with the id 1.
    let response = bridge.set_light_state("1", &light_modifier).unwrap();
    println!("{:?}", response);
}
