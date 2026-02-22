pub fn into_map_id(x: u16, y: u16, z: u16) -> u64 {
    ((x as u64) << 32) | ((y as u64) << 16) | (z as u64)
}

pub fn from_map_id(map_id: u64) -> (u16, u16, u16) {
    let x = (map_id >> 32) as u16;
    let y = ((map_id >> 16) & 0xFFFF) as u16;
    let z = (map_id & 0xFFFF) as u16;
    (x, y, z)
}
