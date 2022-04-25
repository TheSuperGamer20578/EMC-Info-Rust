use crate::{Colour, Error, Result};
use crate::data::Data;
use crate::resident::Resident;
use crate::town::{NATION_REGEX, Town};

#[derive(Debug, Clone)]
pub struct Nation {
    pub name: String,
    // pub area: u16, TODO
}

impl Nation {
    pub fn towns(&self) -> Result<Vec<Town>> {
        todo!()
    }

    pub fn capital(&self) -> Result<Town> {
        todo!()
    }

    pub fn leader(&self) -> Result<Resident> {
        todo!()
    }

    pub fn fill_colour(&self) -> Result<Colour> {
        todo!()
    }

    pub fn stroke_colour(&self) -> Result<Colour> {
        todo!()
    }

    pub fn citizens(&self) -> Result<Vec<Resident>> {
        todo!()
    }
}

pub fn get(data: &Data, nation: &String) -> Result<Nation> {
    let name = if data.ignore_case { nation.to_lowercase() } else { nation.to_string() };
    let mut nation_name: Option<&str> = None;
    for (_, town) in data.towns.iter() {
        let town_nation = NATION_REGEX.captures(&town.desc).ok_or(Error::ParseError("Nation regex did not match"))?
            .name("nation").ok_or(Error::ParseError("Nation regex did not capture nation"))?.as_str();
        if if data.ignore_case { town_nation.to_lowercase() } else { town_nation.to_string() } != name {
            continue;
        }
        if let None = nation_name {
            nation_name = Some(town_nation);
        }
    }
    match nation_name {
        Some(nation_name) => { Ok(Nation { name: nation_name.to_string() }) }
        None => { Err(Error::NationNotFound) }
    }
}
