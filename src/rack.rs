use crate::constants::RACK_SIZE;

pub struct Rack {
    pub tiles: [char; RACK_SIZE],
    pub len: usize,
}

impl Rack {
    pub fn from_arrays(tiles: [char; RACK_SIZE], len: usize) -> Self {
        Self { tiles, len }
    }
}
