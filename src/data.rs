use std::collections::HashMap;
use serde::Deserialize;
#[cfg(feature = "sync")]
use reqwest::blocking as reqwest;
use serde_json::Value;
use crate::{Colour, Result};
use crate::Error::ParseError;

const TOWN_DATA_URL: &str = "https://earthmc.net/map/tiles/_markers_/marker_earth.json";
const PLAYER_DATA_URL: &str = "https://earthmc.net/map/up/world/earth/";

pub struct Data {
    pub(crate) towns: HashMap<String, Town>,
    pub(crate) players: Players,
    pub(crate) ignore_case: bool,
}

pub(crate) struct Town {
    name: String,
    fill_colour: Colour,
    colour: Colour,
    x: Vec<i32>,
    z: Vec<i32>,
    desc: String,
}

#[derive(Deserialize)]
pub(crate) struct Players {
    #[serde(rename = "currentcount")]
    pub(crate) current_count: u16,
    #[serde(rename = "hasStorm")]
    pub(crate) has_storm: bool,
    #[serde(rename = "players")]
    pub(crate) players_vec: Vec<Player>,
    #[serde(default)]
    pub(crate) players: HashMap<String, Player>,
}

#[derive(Deserialize, Clone)]
pub(crate) struct Player {
    pub(crate) world: String,
    pub(crate) armor: u8,
    pub(crate) name: String,
    pub(crate) x: f64,
    pub(crate) y: f32,
    pub(crate) health: u8,
    pub(crate) z: f64,
    pub(crate) sort: u8,
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) account: String,
}

#[cfg(feature = "sync")]
pub fn get_sync(ignore_case: bool) -> Result<Data> {
    let town_data = &reqwest::get(TOWN_DATA_URL)?.json::<Value>()?["sets"]["townyPlugin.markerset"]["areas"];
    let towns = town_data.as_object().ok_or(ParseError("town_data is not an object"))?.iter()
        .filter(|(name, _)| name.ends_with("__0"))
        .map(|(_, town)| {
            let t = Town {
                name: town["label"].as_str().ok_or(ParseError("town[\"label\"] is not a string"))?.to_string(),
                fill_colour: Colour::try_from(town["fillcolor"].as_str().ok_or(ParseError("town[\"fillcolor\"] is not a string"))?.to_string())?,
                colour: Colour::try_from(town["color"].as_str().ok_or(ParseError("town[\"color\"] is not a string"))?.to_string())?,
                x: town["x"].as_array().ok_or(ParseError("town[\"x\"] is not an array"))?.iter().map(|x| Ok(x.as_f64().ok_or(ParseError("x is not an int"))? as i32)).collect::<Result<Vec<i32>>>()?,
                z: town["z"].as_array().ok_or(ParseError("town[\"z\"] is not an array"))?.iter().map(|x| Ok(x.as_f64().ok_or(ParseError("z is not an int"))? as i32)).collect::<Result<Vec<i32>>>()?,
                desc: town["desc"].as_str().ok_or(ParseError("town[\"desc\"] is not a string"))?.to_string(),
            };
            Ok((if ignore_case { t.name.to_lowercase().clone() } else { t.name.clone() }, t))
        })
        .collect::<Result<HashMap<String, Town>>>()?;
    let mut players: Players = reqwest::get(PLAYER_DATA_URL)?.json()?;
    players.players = players.players_vec.iter()
        .map(|player| (if ignore_case { player.account.to_lowercase().clone() } else { player.account.clone() }, (*player).clone()))
        .collect();
    Ok(Data { towns, players, ignore_case })
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "sync")]
    use crate::data::get_sync;

    #[cfg(feature = "sync")]
    #[test]
    fn sync() {
        get_sync(false).unwrap();
        get_sync(true).unwrap();
    }
}
