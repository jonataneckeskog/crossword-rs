#![allow(dead_code)]

use crate::constants::{EMPTY_TILE, RACK_SIZE};

pub struct Rack {
    pub tiles: [char; RACK_SIZE],
    pub len: usize,
    pub used_mask: u8,
}

impl Rack {
    pub fn from_arrays(tiles: [char; RACK_SIZE], len: usize) -> Self {
        let mut used_mask: u8 = 0;
        for (i, tile) in tiles.iter().enumerate() {
            if *tile != EMPTY_TILE {
                used_mask |= 1 << i;
            }
        }
        Self {
            tiles,
            len,
            used_mask,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn is_index_used(&self, idx: usize) -> bool {
        (self.used_mask & (1 << idx)) != 0
    }

    #[inline]
    pub fn mark_used(&mut self, idx: usize) {
        self.used_mask |= 1 << idx;
        self.len -= 1;
    }

    #[inline]
    pub fn unmark_used(&mut self, idx: usize) {
        self.used_mask &= !(1 << idx);
        self.len += 1;
    }

    /// Iterate over available tiles with their indices.
    pub fn available_tiles(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        // Iterate over tiles that are present on the rack (marked as used)
        self.tiles
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_index_used(*i))
            .map(|(i, &c)| (i, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{EMPTY_TILE, RACK_SIZE};

    #[test]
    fn test_from_arrays_sets_used_mask_and_len() {
        // build an array with two tiles placed at indices 0 and 3
        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        tiles[0] = 'A';
        tiles[3] = 'D';
        let len = 2usize;

        let rack = Rack::from_arrays(tiles, len);

        let expected_mask: u8 = (1 << 0) | (1 << 3);
        assert_eq!(rack.used_mask, expected_mask);
        assert_eq!(rack.len, len);
        assert!(rack.is_index_used(0));
        assert!(rack.is_index_used(3));
        assert!(!rack.is_index_used(1));
    }

    #[test]
    fn test_available_tiles_returns_placed_tiles() {
        // tiles present at 1 and 4, other slots are EMPTY
        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        tiles[1] = 'X';
        tiles[4] = 'Z';
        let rack = Rack::from_arrays(tiles, 2);

        let avail: Vec<(usize, char)> = rack.available_tiles().collect();

        // available_tiles should return only the placed tiles (indices 1 and 4)
        assert_eq!(avail, vec![(1usize, 'X'), (4usize, 'Z')]);
    }

    #[test]
    fn test_mark_and_unmark_used_updates_mask_and_len() {
        let tiles = [EMPTY_TILE; RACK_SIZE];
        // start with all slots available and len = RACK_SIZE
        let mut rack = Rack::from_arrays(tiles, RACK_SIZE);

        // mark index 2 as used
        rack.mark_used(2);
        assert!(rack.is_index_used(2));
        assert_eq!(rack.len, RACK_SIZE - 1);

        // unmark it and ensure state is restored
        rack.unmark_used(2);
        assert!(!rack.is_index_used(2));
        assert_eq!(rack.len, RACK_SIZE);
    }
}
