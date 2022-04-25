use crate::{Position, Result};
use crate::data::{Data, Player};
use crate::town::Town;

pub struct Resident {
    pub name: String,
    pub online: bool,
    pub position: Option<Position>,
    pub hidden: bool,
    pub town: Option<Town>,
    pub nation: Option<Nation>,
    pub npc: bool,
}

pub fn get(data: &Data, resident: &String) -> Result<Resident> {
    let resident = if data.ignore_case { resident.to_lowercase() } else { resident };
    Ok(with_town(data, &resident, data.towns.iter()
        .find_map(|(name, town)| {
            if town.desc.contains(&resident) { Some(name) } else { None }
        })
        .map_or(Ok(None), |town| Ok(Some(town::get(data, town)?)))?,
    ))
}

fn with_town(data: &Data, resident: &String, town: Option<Town>) -> Resident {
    let name = if data.ignore_case { resident.to_lowercase() } else { resident };
    match data.players.players.get(&*name) {
        Some(resident_data) => {
            Resident {
                name: resident_data.name.clone(),
                online: true,
                position: Some(Position {
                    x: resident_data.x as i32,
                    y: resident_data.y as i16,
                    z: resident_data.z as i32,
                }),
                hidden: resident_data.x == 0 && resident_data.y == 64 && resident_data.z == 0,
                town,
                nation: match &town {
                    Some(town) => { town.nation }
                    None => { None }
                },
                npc: resident.startswith("NPC") && resident[3..].parse::<u16>().is_ok(),
            }
        }
        None => {
            Resident {
                name: resident.clone(),
                online: false,
                position: None,
                hidden: true,
                town,
                nation: None,
                npc: resident.startswith("NPC") && resident[3..].parse::<u16>().is_ok(),
            }
        }
    }
}
