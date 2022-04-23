//! EarthMC is a large Minecraft server this package lets you get info about things on that server.
#![warn(clippy::pedantic)]

use std::fmt::{Display, Formatter};

pub mod util;

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
