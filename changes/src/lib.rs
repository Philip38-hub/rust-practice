#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    let iter = lights
        .iter_mut()
    for light in iter {
        if light.alias == alias {
            light.brightness = value;
            break;
        }
    }
}
