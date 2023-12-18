use prse::{Parse, ParseError};
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl<'a> Parse<'a> for RGB {
    fn from_str(value: &str) -> Result<Self, ParseError> {
        if value.len() != 7 {
            return Err(ParseError::new(
                "Expected 7 characters to describe the color",
            ));
        } else if !value.starts_with('#') {
            return Err(ParseError::new(
                "Expected a # at the start of the color definition",
            ));
        }

        let red = u8::from_str_radix(&value[1..3], 16).map_err(|_| {
            ParseError::new("Could not parse the red color, is it a valid 2 hexadecimal digits?")
        })?;
        let green = u8::from_str_radix(&value[3..5], 16).map_err(|_| {
            ParseError::new("Could not parse the green color, is it a valid 2 hexadecimal digits?")
        })?;
        let blue = u8::from_str_radix(&value[5..7], 16).map_err(|_| {
            ParseError::new("Could not parse the blue color, is it a valid 2 hexadecimal digits?")
        })?;

        Ok(RGB { red, green, blue })
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "#{:x}{:x}{:x}",
            self.red, self.green, self.blue
        ))
    }
}

impl Debug for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "RGB(#{:x}{:x}{:x})",
            self.red, self.green, self.blue
        ))
    }
}
