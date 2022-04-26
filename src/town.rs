use lazy_static::lazy_static;
use regex::Regex;

use crate::{Colour, Error, nation, Result};
use crate::data::Data;
use crate::nation::Nation;
use crate::resident::Resident;

lazy_static! {
    static ref DESC_REGEX: Regex = Regex::new(r">[^<>]+ \([^<>)]+\)<.+> Mayor <.+>(?P<mayor>[^<>]+)<.+> Members <.+>(?P<residents>[^<>]+)<.+>Flags<.+>hasUpkeep: (?P<upkeep>true|false)<.+>pvp: (?P<pvp>true|false)<.+>mobs: (?P<mobs>true|false)<.+>public: (?P<public>true|false)<.+>explosion: (?P<explosions>true|false)<.+>fire: (?P<fire>true|false)<.+>capital: (?P<capital>true|false)").unwrap();
    pub(crate) static ref NATION_REGEX: Regex = Regex::new(r">[^<>(]+ \((?P<nation>[^<>)]+)\)<").unwrap();
}

#[derive(Debug, Clone)]
pub struct Town {
    pub name: String,
    pub nation: Nation,
    pub stroke_colour: Colour,
    pub fill_colour: Colour,
    pub flags: TownFlags,
    // pub position: Position, TODO
    // pub bounds: Bounds, TODO
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

#[derive(Debug, Clone)]
pub struct TownFlags {
    pub pvp: bool,
    pub mobs: bool,
    pub explosions: bool,
    pub fire: bool,
    pub capital: bool,
}

pub fn get(data: &Data, town: &String) -> Result<Town> {
    let town = if data.ignore_case { town.to_lowercase() } else { town.to_string() };
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
    let town = if data.ignore_case { town.to_lowercase() } else { town.to_string() };
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
    })
}
