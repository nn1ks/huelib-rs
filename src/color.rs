use std::num;
use thiserror::Error;

/// Struct for setting the color of a light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Color space coordinates.
    pub(crate) space_coordinates: (f32, f32),
    /// Brightness of the color.
    pub(crate) brightness: Option<u8>,
}

impl Color {
    /// Creates a new color from space coordinates.
    ///
    /// This only changes the color of a light and not the brightness.
    pub fn from_space_coordinates(x: f32, y: f32) -> Self {
        Self {
            space_coordinates: (x, y),
            brightness: None,
        }
    }

    /// Creates a new color from rgb values.
    ///
    /// This changes the color and brightness of a light.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        // NOTE: More information: https://gist.github.com/popcorn245/30afa0f98eea1c2fd34d
        let gamma_correct = |v: f32| {
            if v > 0.04045 {
                ((v + 0.055) / (1.0 + 0.055)).powf(2.4)
            } else {
                v / 12.92
            }
        };
        let red = gamma_correct(red as f32 / 255.0);
        let green = gamma_correct(green as f32 / 255.0);
        let blue = gamma_correct(blue as f32 / 255.0);
        let x = red * 0.649926 + green * 0.103455 + blue * 0.197109;
        let y = red * 0.234327 + green * 0.743075 + blue * 0.022598;
        let z = red * 0.000000 + green * 0.053077 + blue * 1.035763;
        Self {
            space_coordinates: (
                x / (x + y + z + f32::MIN_POSITIVE),
                y / (x + y + z + f32::MIN_POSITIVE),
            ),
            brightness: Some((y * 255.0) as u8),
        }
    }

    /// Creates a new color from a hex value.
    ///
    /// This changes the color and brightness of a light.
    ///
    /// The string must begin with a `#` followed by 3 or 6 hex values.
    ///
    /// # Examples
    ///
    /// Generate a red color.
    /// ```rust
    /// # use huelib::Color;
    /// let red = Color::from_hex("#FF0000").unwrap();
    ///
    /// assert_eq!(red, Color::from_rgb(255, 0, 0));
    /// ```
    ///
    /// Use shorter version with 3 hex values to generate a color.
    /// ```rust
    /// # use huelib::Color;
    /// let color = Color::from_hex("#F40").unwrap();
    ///
    /// assert_eq!(color, Color::from_hex("#ff4400").unwrap())
    /// ```
    pub fn from_hex<S: AsRef<str>>(s: S) -> Result<Self, ParseHexError> {
        let s = s.as_ref();
        match s.len() {
            4 => {
                let red = u8::from_str_radix(&s[1..2], 16)?;
                let green = u8::from_str_radix(&s[2..3], 16)?;
                let blue = u8::from_str_radix(&s[3..4], 16)?;
                Ok(Self::from_rgb(
                    red * 16 + red,
                    green * 16 + green,
                    blue * 16 + blue,
                ))
            }
            7 => Ok(Self::from_rgb(
                u8::from_str_radix(&s[1..3], 16)?,
                u8::from_str_radix(&s[3..5], 16)?,
                u8::from_str_radix(&s[5..7], 16)?,
            )),
            _ => Err(ParseHexError::InvalidLenght),
        }
    }
}

/// Errors that can occur while parsing a hex string to a color.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum ParseHexError {
    /// Error that occurs when the length of the hex string is invalid.
    #[error("Invalid string length")]
    InvalidLenght,
    /// Error that can occur while parsing a int value.
    #[error("Failed to parse a int value")]
    ParseInt(#[from] num::ParseIntError),
}
