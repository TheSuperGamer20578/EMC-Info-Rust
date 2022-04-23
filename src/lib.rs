//! EarthMC is a large Minecraft server this package lets you get info about things on that server.
#![warn(clippy::pedantic)]

use std::error;
use std::fmt::{Display, Formatter};

pub mod util;

pub type Result<T> = core::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TownNotFound,
    NationNotFound,
    ReqwestError(reqwest::Error),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TownNotFound => write!(f, "The specified town could not be found"),
            Error::NationNotFound => write!(f, "The specified nation could not be found"),
            Error::ReqwestError(error) => write!(f, "A reqwest error has occurred: {}", error),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        self::ReqwestError(error)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Bounds {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}
