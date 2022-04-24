use lazy_static::lazy_static;
use regex::Regex;
use crate::{Bounds, Colour, Error, Position, Result};
use crate::data::Data;

lazy_static! {
    static ref DESC_REGEX: Regex = Regex::new(r">[^<>]+ \((?<nation>[^<>]+)\)<.+> Mayor <.+>(?<mayor>[^<>]+)<.+> Members <.+>(?<residents>[^<>]+)<.+>Flags<.+>hasUpkeep: (?<upkeep>true|false)<.+>pvp: (?<pvp>true|false)<.+>mobs: (?<mbos>true|false)<.+>public: (?<public>true|false)<.+>explosion: (?<explosions>true|false)<.+>fire: (?<fire>true|false)<.+>capital: (?<capital>true|false)").unwrap()
}

pub struct Town {
    pub name: String,
    pub nation: Nation,
    pub stroke_colour: Colour,
    pub fill_colour: Colour,
    pub mayor: Resident,
    pub residents: Vec<Resident>,
    pub flags: TownFlags,
    pub position: Position,
    pub bounds: Bounds,
    // pub ruins: bool, TODO
    // pub area: u16, TODO
}

pub struct TownFlags {
    pub pvp: bool,
    pub mobs: bool,
    pub explosions: bool,
    pub fire: bool,
    pub capital: bool,
}

pub fn get(data: Data, town: String) -> Result<Town> {
    let town = if data.ignore_case { town.to_lowercase() } else { town };
    todo!("Awaiting nation implementation")
}

fn with_nation(data: Data, town: String, nation: Nation) -> Result<Town> {
    let town = if data.ignore_case { town.to_lowercase() } else { town };
    let town_data = data.towns.get(&town).ok_or(Error::TownNotFound)?;
    let captures = DESC_REGEX: Regex.captures(&town_data.desc).ok_or(Error::ParseError("Regex did not match"))?;
    Ok(Town {
        name: town_data.name.clone(),
        nation,
        stroke_colour: town_data.colour.clone(),
        fill_colour: town_data.fill_colour.clone(),
        mayor: captures.name("mayor").ok_or(Error::ParseError("Regex did not capture name")),
        residents: captures.name("residents").ok_or(Error::ParseError("Regex did not capture residents"))?.as_str().split(", ").map(|resident| todo!("Awaiting Resident implementation")).collect(),
        flags: TownFlags {
            pvp: captures.name("pvp").ok_or(Error::ParseError("Regex did not capture pvp"))?.as_str().parse().or(Err(Error::ParseError("pvp is not a bool")))?,
            mobs: captures.name("mobs").ok_or(Error::ParseError("Regex did not capture mobs"))?.as_str().parse().or(Err(Error::ParseError("mobs is not a bool")))?,
            explosions: captures.name("explosions").ok_or(Error::ParseError("Regex did not capture explosions"))?.as_str().parse().or(Err(Error::ParseError("explosions is not a bool")))?,
            fire: captures.name("fire").ok_or(Error::ParseError("Regex did not capture fire"))?.as_str().parse().or(Err(Error::ParseError("fire is not a bool")))?,
            capital: captures.name("capital").ok_or(Error::ParseError("Regex did not capture capital"))?.as_str().parse().or(Err(Error::ParseError("capital is not a bool")))?,
        },
        position: Position {
            x: town_data.x.iter().sum() / town_data.x.len(),
            y: 0,
            z: town_data.z.iter().sum() / town_data.z.len(),
        },
        bounds: Bounds {
            x1: *town_data.x.iter().min().ok_or(Err(Error::ParseError("town_data.x is empty")))?,
            z1: *town_data.z.iter().min().ok_or(Err(Error::ParseError("town_data.z is empty")))?,
            x2: *town_data.x.iter().max().ok_or(Err(Error::ParseError("town_data.x is empty")))?,
            z2: *town_data.z.iter().max().ok_or(Err(Error::ParseError("town_data.z is empty")))?,
        },
    })
}
