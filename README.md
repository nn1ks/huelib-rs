# huelib-rs

[![Build](https://img.shields.io/github/workflow/status/yuqio/huelib-rs/Rust)](https://github.com/yuqio/huelib-rs/actions)
[![Crate](https://img.shields.io/crates/v/huelib)](https://crates.io/crates/huelib)
[![Docs](https://docs.rs/huelib/badge.svg)](https://docs.rs/huelib)
[![License](https://img.shields.io/github/license/yuqio/huelib-rs)](https://github.com/yuqio/huelib-rs/blob/master/LICENSE)
[![Code size](https://img.shields.io/github/languages/code-size/yuqio/huelib-rs)]()
[![Lines of code](https://tokei.rs/b1/github/yuqio/huelib-rs?category=code)]()

<!-- cargo-sync-readme start -->

Rust bindings for the [Philips Hue API].

## About

This library sends HTTP requests to the bridge using the [ureq] crate. The responses/requests
are deserialized/serialized using the [serde], [serde_json] and [serde_repr] crates.

[Philips Hue API]: https://developers.meethue.com/develop/hue-api
[ureq]: https://github.com/algesten/ureq
[serde]: https://github.com/serde-rs/serde
[serde_json]: https://github.com/serde-rs/json
[serde_repr]: https://github.com/dtolnay/serde-repr

## Example

Register a user and set the brightness and saturation of a light.
```rust
use huelib::{bridge, light, Modifier};
use std::net::{IpAddr, Ipv4Addr};

let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
let username = match bridge::register_user(bridge_ip, "huelib-rs example", false) {
    Ok(v) => v.name,
    Err(e) => {
        println!("Failed to register user: {}", e);
        return;
    }
};
let bridge = huelib::Bridge::new(bridge_ip, &username);
let state_modifier = light::StateModifier::new()
    .brightness(huelib::ModifierType::Increment, 40)
    .saturation(huelib::ModifierType::Override, 200);
match bridge.set_light_state("1", &state_modifier) {
    Ok(v) => {
        for response in v {
            println!("{}", response);
        }
    },
    Err(e) => {
        println!("Failed to set the state of the light: {}", e);
        return;
    }
};
```

<!-- cargo-sync-readme end -->

## Todo

- [x] Lights API
- [x] Groups API
- [x] Scenes API
- [x] Configuration API
- [x] Capabilities API
- [x] Schedules API
- [x] Resourcelinks API
- [ ] Rules API
- [ ] Sensors API
