use std::num::ParseIntError;
use thiserror::Error as ThisError;

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
    ///
    /// # Examples
    ///
    /// Generate a color and use it in a modifier:
    /// ```
    /// use huelib::{Color, resource::light};
    ///
    /// let color = Color::from_space_coordinates(0.1, 0.2);
    /// let modifier = light::StateModifier::new().with_color(color);
    /// ```
    pub fn from_space_coordinates(x: f32, y: f32) -> Self {
        Self {
            space_coordinates: (x, y),
            brightness: None,
        }
    }

    /// Creates a new color from rgb values.
    ///
    /// This changes the color and brightness of a light.
    ///
    /// # Examples
    ///
    /// Generate a color and use it in a modifier:
    /// ```
    /// use huelib::{Color, resource::light};
    ///
    /// let color = Color::from_rgb(255, 0, 0);
    /// let modifier = light::StateModifier::new().with_color(color);
    /// ```
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
        let x = red * 0.649_926 + green * 0.103_455 + blue * 0.197_109;
        let y = red * 0.234_327 + green * 0.743_075 + blue * 0.022_598;
        let z = red * 0.000_000 + green * 0.053_077 + blue * 1.035_763;
        Self {
            space_coordinates: (
                x / (x + y + z + std::f32::MIN_POSITIVE),
                y / (x + y + z + std::f32::MIN_POSITIVE),
            ),
            brightness: Some((y * 255.0) as u8),
        }
    }

    /// Creates a new color from a hex value.
    ///
    /// The string must begin with a `#` followed by either 3 or 6 hexadecimal digits.
    ///
    /// This changes the color and brightness of a light.
    ///
    /// # Examples
    ///
    /// Generate a color and use it in a modifier:
    /// ```
    /// use huelib::{Color, resource::light};
    ///
    /// # fn main() -> Result<(), huelib::color::ParseHexError> {
    /// let red = Color::from_hex("#FF0000")?;
    /// let modifier = light::StateModifier::new().with_color(red);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Generate a color using the short version:
    /// ```
    /// # fn main() -> Result<(), huelib::color::ParseHexError> {
    /// let color = huelib::Color::from_hex("#02B")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_hex(s: impl AsRef<str>) -> Result<Self, ParseHexError> {
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
#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum ParseHexError {
    /// Error that occurs when the length of the hex string is invalid.
    #[error("Invalid string length")]
    InvalidLenght,
    /// Error that can occur while parsing a int value.
    #[error("Failed to parse a int value")]
    ParseInt(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_coordinates() {
        let color = Color::from_space_coordinates(0.1, 0.2);
        assert_eq!(color.space_coordinates, (0.1, 0.2));
        assert_eq!(color.brightness, None);
    }

    #[test]
    fn rgb_white() {
        let color = Color::from_rgb(255, 255, 255);
        assert_eq!(color.brightness, Some(255));
    }

    #[test]
    fn rgb_black() {
        let color = Color::from_rgb(0, 0, 0);
        assert_eq!(color.brightness, Some(0));
    }

    #[test]
    fn hex_white() {
        let color = Color::from_hex("#FFFFFF").unwrap();
        assert_eq!(color.brightness, Some(255));
        let color = Color::from_hex("#ffffff").unwrap();
        assert_eq!(color.brightness, Some(255));
        let color = Color::from_hex("#FFF").unwrap();
        assert_eq!(color.brightness, Some(255));
        let color = Color::from_hex("#fff").unwrap();
        assert_eq!(color.brightness, Some(255));
    }

    #[test]
    fn hex_black() {
        let color = Color::from_hex("#000000").unwrap();
        assert_eq!(color.brightness, Some(0));
        let color = Color::from_hex("#000").unwrap();
        assert_eq!(color.brightness, Some(0));
    }

    #[test]
    fn hex_short() {
        let color = Color::from_hex("#FFF").unwrap();
        assert_eq!(color, Color::from_hex("#FFFFFF").unwrap());
        let color = Color::from_hex("#f00").unwrap();
        assert_eq!(color, Color::from_hex("#FF0000").unwrap());
        let color = Color::from_hex("#123").unwrap();
        assert_eq!(color, Color::from_hex("#112233").unwrap());
    }

    #[test]
    fn rgb_and_hex() {
        let color1 = Color::from_hex("#fff").unwrap();
        let color2 = Color::from_rgb(255, 255, 255);
        assert_eq!(color1, color2);
        let color1 = Color::from_hex("#000").unwrap();
        let color2 = Color::from_rgb(0, 0, 0);
        assert_eq!(color1, color2);
        let color1 = Color::from_hex("#0F0").unwrap();
        let color2 = Color::from_rgb(0, 255, 0);
        assert_eq!(color1, color2);
        let color1 = Color::from_hex("#100").unwrap();
        let color2 = Color::from_rgb(17, 0, 0);
        assert_eq!(color1, color2);
        let color1 = Color::from_hex("#02F").unwrap();
        let color2 = Color::from_rgb(0, 34, 255);
        assert_eq!(color1, color2);
    }
}
