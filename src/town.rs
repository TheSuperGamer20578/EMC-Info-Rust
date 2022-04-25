use lazy_static::lazy_static;
use regex::Regex;

use crate::{Bounds, Colour, Error, nation, Position, Result};
use crate::data::Data;
use crate::nation::Nation;
use crate::resident::Resident;

lazy_static! {
    static ref DESC_REGEX: Regex = Regex::new(r">[^<>]+ \([^<>)]+\)<.+> Mayor <.+>(?<mayor>[^<>]+)<.+> Members <.+>(?<residents>[^<>]+)<.+>Flags<.+>hasUpkeep: (?<upkeep>true|false)<.+>pvp: (?<pvp>true|false)<.+>mobs: (?<mbos>true|false)<.+>public: (?<public>true|false)<.+>explosion: (?<explosions>true|false)<.+>fire: (?<fire>true|false)<.+>capital: (?<capital>true|false)").unwrap();
    pub(crate) static ref NATION_REGEX: Regex = Regex::new(r">[^<>(]+ \((?<nation>[^<>)]+)\)").unwrap();
}

pub struct Town {
    pub name: String,
    pub nation: Nation,
    pub stroke_colour: Colour,
    pub fill_colour: Colour,
    pub flags: TownFlags,
    pub position: Position,
    pub bounds: Bounds,
    // pub ruins: bool, TODO
    // pub area: u16, TODO
}

impl Town {
    pub fn mayor(&self) -> Result<Resident> {
        todo!()
    }

    pub fn residents(&self) -> Result<Vec<Resident>> {
        todo!()
    }
}

pub struct TownFlags {
    pub pvp: bool,
    pub mobs: bool,
    pub explosions: bool,
    pub fire: bool,
    pub capital: bool,
}

pub fn get(data: &Data, town: &String) -> Result<Town> {
    let town = if data.ignore_case { town.to_lowercase() } else { town };
    with_nation(data, &town, nation::get(data, &NATION_REGEX
        .captures(&*data.towns.get(&town).ok_or(Error::TownNotFound)?.desc)
        .ok_or(Error::ParseError("Nation regex did not match"))?
        .name("nation")
        .ok_or(Error::ParseError("Nation regex did not capture nation"))?
        .as_str()
        .to_string(),
    )?)
}

pub(crate) fn with_nation(data: &Data, town: &String, nation: Nation) -> Result<Town> {
    let town = if data.ignore_case { town.to_lowercase() } else { town };
    let town_data = data.towns.get(&town).ok_or(Error::TownNotFound)?;
    let captures = DESC_REGEX.captures(&town_data.desc).ok_or(Error::ParseError("Regex did not match"))?;
    Ok(Town {
        name: town_data.name.clone(),
        nation,
        stroke_colour: town_data.colour.clone(),
        fill_colour: town_data.fill_colour.clone(),
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
