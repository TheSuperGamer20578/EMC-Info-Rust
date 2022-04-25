use crate::Position;

/// Return a link to the map at the specified position
pub fn map_link(position: &Position, zoom: &u8) -> String {
    format!("https://earthmc.net/map/?zoom={}&x={}&z={}", zoom, position.x, position.z)
}

#[cfg(test)]
mod tests {
    use crate::Position;
    use crate::util::map_link;

    #[test]
    fn test_map_link() {
        assert_eq!(map_link(&Position { x: 32943, y: 64, z: -13297 }, &6), "https://earthmc.net/map/?zoom=6&x=32943&z=-13297")
    }
}
