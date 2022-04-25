//! EarthMC is a large Minecraft server this package lets you get info about things on that server.
#![warn(clippy::pedantic)]

use std::error;
use std::fmt::{Display, Formatter};

pub mod util;
pub mod data;
pub mod town;
pub mod resident;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TownNotFound,
    NationNotFound,
    ReqwestError(reqwest::Error),
    ParseError(&'static str),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TownNotFound => write!(f, "The specified town could not be found"),
            Error::NationNotFound => write!(f, "The specified nation could not be found"),
            Error::ReqwestError(error) => write!(f, "A reqwest error has occurred: {}", error),
            Error::ParseError(error) => write!(f, "Something went wrong while parsing: {}", error),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl TryFrom<String> for Colour {
    type Error = Error;

    fn try_from(colour: String) -> Result<Self> {
        if colour.len() != 7 || !colour.starts_with('#') {
            return Err(Error::ParseError("Invalid colour"))
        }
        let colour = u32::from_str_radix(&colour[1..], 16).or(Err(Error::ParseError("Invalid colour")))?;
        #[allow(clippy::unreadable_literal)]
        Ok(Colour {
            red: (colour >> 16) as u8,
            green: (colour >> 8) as u8,
            blue: colour as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Colour;

    const COLOUR: Colour = Colour { red: 98, green: 19, blue: 243 };

    #[test]
    fn colour() {
        assert_eq!(Colour::try_from(String::from("#6213F3")).unwrap(), COLOUR);
        assert_eq!(Colour::try_from(String::from("#6213f3")).unwrap(), COLOUR);
        assert_eq!(format!("{}", COLOUR), "#6213F3");
        assert!(Colour::try_from(String::from("6213f3")).is_err());
        assert!(Colour::try_from(String::from("#62xXx3")).is_err());
    }
}
